use std::fmt::Error;
use std::sync::{Arc, RwLock};
use std::thread;
use crate::{
    bucket::Bucket,
};

const DEFAULT_SIZE: usize = 16;

pub struct Table<Any> {
    buckets: Arc<RwLock<Vec<Bucket<Any>>>>
}


/// Non async table impls
impl<Any: Clone> Table<Any> {
    pub fn new() -> Self {
        let mut buckets = Vec::with_capacity(DEFAULT_SIZE);
        for _ in 0..DEFAULT_SIZE {
            buckets.push(Bucket::new());
        }

        Table {
            buckets: Arc::new(RwLock::new(buckets)),
        }
    }

    ///get the length of the table
    pub fn len(&self) -> usize {
        (&self.buckets).read().unwrap().len()
    }

    ///get the total number of entries in the table (not recursive)
    pub fn size(&self) -> usize {
        let table = &self.buckets.read().unwrap();
        let mut len = 0;
        for i in 0..table.len() {
            len += &table[i].len();
        }
        len
    }

    ///naive hashing algorithm for a string. nothing crazy
    fn hash(&self, key: &str) -> usize {
        let table_len = self.len();
        let mut idx: usize = 4096;
        key.chars().into_iter().for_each(|c| {
            idx += c as usize * 2;
        });
        idx % table_len
    }

    ///this could be done in a different thread,
    /// spawn a new one on table insert to see if the size is okay?
    fn resize(&mut self) {
        todo!("Resize the table on insert if the load is too big?")
    }

    pub fn upsert(&mut self) {
        todo!("Upsert for cool people?")
    }
}

impl<Any: Clone + Copy> Table<Any> {
    ///return a read-only clone of the original bucket
    fn get_read_bucket(&self, idx: usize) -> Option<Bucket<Any>> {
        match self.buckets.read() {
            Ok(buckets) => Some(buckets[idx].clone()),
            Err(_) => None,
        }
    }

    pub fn get(&self, key: &str) -> Option<Any> {
        let idx = self.hash(key);
        if let Some(bucket) = self.get_read_bucket(idx) {
            bucket.get_value_from_key(key).copied()
        }
        else {
            None
        }
    }

    ///can't return a mutable reference here, create a method to take in a functino
    /// and apply it to a given bucket?
    /// 
    /// would either be:
    /// * update
    /// * delete
    /// * create
    /// 
    /// ```rust
    /// apply_write_bucket(&mut self, idx: usize, function?) {
    ///     //apply the function to the given bucket
    /// }
    /// ```
    // fn apply_write_bucket<F>(&mut self, idx: usize, func: F) -> Option<()>
    // where F: Fn(&mut Bucket<Any>) -> Option<()> {
    //     match self.buckets.write() {
    //         Ok(mut buckets) =>  {
    //             let bucket = &mut buckets[idx];
    //             func(bucket)
    //         },
    //         Err(_) => None,
    //     }
    // }

    fn add(&mut self, key: &str, data: Any) -> Option<()> {
        let idx = self.hash(key);
        self.apply_write_bucket(idx, |bucket| {
            Some(bucket.add(key, data))
        })
    }
}


///implementation of any threaded methods
impl<Any: Clone + Copy + Send + Sync + 'static> Table<Any> {
    fn apply_write_bucket<F>(&mut self, idx: usize, func: F) -> Option<()>
    where F: Fn(&mut Bucket<Any>) -> Option<()> {
        match self.buckets.write() {
            Ok(mut buckets) =>  {
                let bucket = &mut buckets[idx];
                func(bucket)
            },
            Err(_) => None,
        }
    }

    
    ///put a key/value combo into the object
    /// 
    /// also do a check for resizing here?
    pub fn put(&mut self, key: &str, data: Any) -> Result<(), Error> {
        let idx = self.hash(key);
        let table_clone = Arc::clone(&self.buckets);

        let data_clone = data.clone();
        let key_clone = key.clone();
        thread::spawn(move || {
            let buckets = table_clone.write().unwrap();
            let bucket = buckets.get(idx);
            match bucket {
                Some(b) => b.add(key, data),
                None => Error::new("Could not grab bucket from hash table!")
            }
        });
        Ok(())
    }
}