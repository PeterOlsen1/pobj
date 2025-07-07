use crate::bucket::Bucket;
use std::fmt;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;

const DEFAULT_SIZE: usize = 16;

// make a trait to define function headers?
pub struct Table<Any> {
    buckets: Arc<RwLock<Vec<Bucket<Any>>>>,
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
        } else {
            None
        }
    }

    pub fn items(&self) -> Vec<(String, Any)> {
        let mut out = Vec::new();
        let buckets = match self.buckets.read() {
            Ok(buckets) => buckets,
            Err(_) => return out,
        };
        let bucket_len = buckets.len();

        for i in 0..bucket_len {
            if let Some(bucket) = buckets.get(i) {
                out.extend(bucket.to_vec());
            }
        }

        out
    }

    ///return all items as an iterator. I thought I'd do this for fun, turning out to be sort of a pain
    ///
    /// TODO!!!: fix this
    pub fn iter_items(&self) -> impl Iterator<Item = (String, Any)> {
        let buckets = self.buckets.read().unwrap();
        let mut cur_idx = 0;
        let mut cur_iter = buckets.get(cur_idx).map(|bucket| bucket.iter());

        std::iter::from_fn(move || {
            loop {
                if let Some(iter) = &mut cur_iter {
                    if let Some(res) = iter.next() {
                        return Some(res);
                    }
                }

                // Move to the next bucket
                cur_idx += 1;
                if cur_idx >= buckets.len() {
                    return None;
                }

                cur_iter = buckets.get(cur_idx).map(|bucket| bucket.iter());
            }
        })
    }
}

///implementation of any threaded methods
impl<Any: Clone + Copy + Send + Sync + 'static> Table<Any> {
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
    fn apply_write_bucket<F>(&mut self, idx: usize, func: F) -> Option<()>
    where
        F: Fn(&mut Bucket<Any>) -> Option<()>,
    {
        match self.buckets.write() {
            Ok(mut buckets) => {
                let bucket = &mut buckets[idx];
                func(bucket)
            }
            Err(_) => None,
        }
    }

    ///put a key/value combo into the object
    ///
    /// TODO: resize check + actually do it
    /// * make helper function that takes in the table to resize?
    pub fn put(self, key: &str, data: Any) -> Result<(), TableError>
// where Self: Send + 'static
    {
        let idx = self.hash(key);
        let table_clone = Arc::clone(&self.buckets);
        let data_clone = data.clone();
        let key_clone = key.to_string();
        let handle = thread::spawn(move || {
            let mut buckets = table_clone.write().unwrap();
            let bucket = buckets.get_mut(idx);
            match bucket {
                Some(b) => Ok(b.add(&key_clone, data_clone)),
                None => Err(TableError::new("Could not grab bucket from hash table!")),
            }
        });

        let self_arc = Arc::new(Mutex::new(self));
        let self_arc_clone = Arc::clone(&self_arc);

        thread::spawn(move || {
            let mut self_locked = self_arc_clone.lock().unwrap();
            todo!("resize here");
        });

        let res = handle.join().unwrap();
        res
    }
}

// custom error implementation down here
#[derive(Debug)]
pub enum TableError {
    WriteFailed(String),
}

impl TableError {
    pub fn new(message: &str) -> Self {
        TableError::WriteFailed(message.to_string())
    }
}

impl fmt::Display for TableError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TableError::WriteFailed(message) => {
                write!(f, "Write failed: {}", message)
            }
        }
    }
}
