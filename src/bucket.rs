use crate::node::Node;
use crate::traits::CloneableAny;

pub struct Bucket {
    head: Option<Box<Node>>, // Updated to use Box<Node> with Box<dyn CloneableAny> inside Node
    length: usize,
}

impl Clone for Bucket {
    fn clone(&self) -> Self {
        Bucket {
            head: self.head.as_ref().map(|node| Box::new((**node).clone())),
            length: self.length,
        }
    }
}

impl Bucket {
    pub fn new() -> Self {
        Bucket {
            head: None,
            length: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn add(&mut self, key: &str, data: Box<dyn CloneableAny>) {
        let mut new_bucket = Node::new(key, data);
        if let Some(head) = self.head.take() {
            new_bucket.set_next(*head);
        }
        self.head = Some(Box::new(new_bucket));
        self.length += 1;
    }

    pub fn to_vec(&self) -> Vec<(String, Box<dyn CloneableAny>)> {
        let mut out = Vec::new();
        let mut cur = self.head.as_deref();
        while let Some(bucket) = cur {
            out.push((bucket.key.clone(), bucket.data.clone_box()));
            cur = bucket.get_next();
        }
        out
    }

    pub fn iter(&self) -> impl Iterator<Item = (String, Box<dyn CloneableAny>)> {
        let mut cur = self.get_head();
        std::iter::from_fn(move || {
            if let Some(bucket) = cur {
                let res = (bucket.key.clone(), bucket.data.clone_box());
                cur = bucket.get_next();
                Some(res)
            } else {
                None
            }
        })
    }

    pub fn get_head(&self) -> Option<&Node> {
        self.head.as_deref()
    }

    pub fn get_value_from_key(&self, key: &str) -> Option<&dyn CloneableAny> {
        let mut cur = self.head.as_deref();
        while let Some(bucket) = cur {
            if bucket.key == key {
                return Some(bucket.data.as_ref() as &dyn CloneableAny);
            }
            cur = bucket.get_next();
        }
        None
    }
}
