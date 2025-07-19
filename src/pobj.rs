use crate::bucket::Bucket;
use crate::traits::CloneableAny;
use std::any::Any;
use std::fmt;
use std::sync::{Arc, RwLock};
use std::thread;

const DEFAULT_SIZE: usize = 16;

// make a trait to define function headers?
pub struct Pobj {
    buckets: Arc<RwLock<Vec<Bucket>>>,
}

/// Non async table impls
impl Pobj {
    pub fn new() -> Self {
        let mut buckets = Vec::with_capacity(DEFAULT_SIZE);
        for _ in 0..DEFAULT_SIZE {
            buckets.push(Bucket::new());
        }

        Pobj {
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

    pub fn upsert(&mut self) {
        todo!("Upsert for cool people?")
    }

    ///return a read-only clone of the original bucket
    fn get_read_bucket(&self, idx: usize) -> Option<Bucket> {
        match self.buckets.read() {
            Ok(buckets) => Some(buckets[idx].clone()),
            Err(_) => None,
        }
    }

    pub fn get<T>(&self, key: &str) -> Option<T>
    where
        T: CloneableAny + 'static,
     {
        let idx = self.hash(key);
        if let Some(bucket) = self.get_read_bucket(idx) {
            if let Some(value) = bucket.get_value_from_key(key) {
                return value;
            }
        }
        None
    } 

    pub fn items(&self) -> Vec<(String, Box<dyn CloneableAny>)> {
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

    //return all items as an iterator. I thought I'd do this for fun, turning out to be sort of a pain
    //
    // TODO!!!: fix this
    // pub fn iter_items(&self) -> impl Iterator<Item = (String, Any)> {
    //     let buckets = self.buckets.read().unwrap();
    //     let mut cur_idx = 0;
    //     let mut cur_iter = buckets.get(cur_idx).map(|bucket| bucket.iter());

    //     std::iter::from_fn(move || {
    //         loop {
    //             if let Some(iter) = &mut cur_iter {
    //                 if let Some(res) = iter.next() {
    //                     return Some(res);
    //                 }
    //             }

    //             // Move to the next bucket
    //             cur_idx += 1;
    //             if cur_idx >= buckets.len() {
    //                 return None;
    //             }

    //             cur_iter = buckets.get(cur_idx).map(|bucket| bucket.iter());
    //         }
    //     })
    // }
}

///implementation of any threaded methods
impl Pobj {
    /// Apply a function to a bucket at the given index with write access
    fn apply_write_bucket<F>(&mut self, idx: usize, func: F) -> Option<()>
    where
        F: Fn(&mut Bucket) -> Option<()>,
    {
        match self.buckets.write() {
            Ok(mut buckets) => {
                let bucket = &mut buckets[idx];
                func(bucket)
            }
            Err(_) => None,
        }
    }

    /// Put a key/value combo into the object.
    ///
    /// TODO: resize check + actually do it
    /// * make helper function that takes in the table to resize?
    pub fn put<T>(&self, key: &str, data: T) -> Result<(), PobjError>
    where
        T: CloneableAny + Send + Sync + 'static,
    {
        let boxed_data = Box::new(data);
        let idx = self.hash(key);
        let table_clone = Arc::clone(&self.buckets);
        let key_clone = key.to_string();
        let handle = thread::spawn(move || {
            let mut buckets = table_clone.write().unwrap();
            let bucket = buckets.get_mut(idx);
            match bucket {
                Some(b) => Ok(b.add(&key_clone, boxed_data)),
                None => Err(PobjError::new("Could not grab bucket from hash table!")),
            }
        });

        let table_clone = Arc::clone(&self.buckets);

        thread::spawn(move || {
            let self_locked = table_clone.read().unwrap();
            get_load_factor(&self_locked);
            todo!("resize here"); 
        });

        let res = handle.join().unwrap();
        res
    }
}

fn get_load_factor(buckets: &Vec<Bucket>) -> () {
    print!("bruh");
}
fn resize(buckets: &mut Vec<Bucket>) -> () {

}

// custom error implementation down here
#[derive(Debug)]
pub enum PobjError {
    WriteFailed(String),
}

impl PobjError {
    pub fn new(message: &str) -> Self {
        PobjError::WriteFailed(message.to_string())
    }
}

impl fmt::Display for PobjError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PobjError::WriteFailed(message) => {
                write!(f, "Write failed: {}", message)
            }
        }
    }
}
