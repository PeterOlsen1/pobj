use crate::node::Node;

pub struct Bucket<Any> {
    head: Option<Box<Node<Any>>>,
    length: usize,
}

impl<Any: Clone> Clone for Bucket<Any> {
    fn clone(&self) -> Self {
        Bucket {
            head: self.head.clone(),
            length: self.length,
        }
    }
}

impl<Any: Clone> Bucket<Any> {
    pub fn new() -> Self {
        Bucket {
            head: None,
            length: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn add(&mut self, key: &str, data: Any) {
        let mut new_bucket = Node::new(key, data);
        if let Some(head) = self.head.take() {
            new_bucket.set_next(*head);
        }
        self.head = Some(Box::new(new_bucket));
        self.length += 1;
    }

    pub fn to_vec(&self) -> Vec<(String, Any)> {
        let mut out = Vec::new();
        let ptr = &self.head;
        while let Some(cur) = ptr {
            out.push((cur.key.clone(), cur.data.clone()));
        }
        out
    }

    pub fn iter(&self) -> impl Iterator<Item = (String, Any)> {
        let mut cur = self.get_head();
        std::iter::from_fn(move || {
            if let Some(bucket) = cur {
                let res = (bucket.key.clone(), bucket.data.clone());
                cur = bucket.get_next();
                Some(res)
            } else {
                None
            }
        })
    }

    pub fn get_head(&self) -> Option<&Node<Any>> {
        self.head.as_deref()
    }

    pub fn get_value_from_key(&self, key: &str) -> Option<&Any> {
        let mut cur = self.head.as_deref();
        while let Some(bucket) = cur {
            if bucket.key == key {
                return Some(&bucket.data);
            }
            cur = bucket.get_next();
        }
        None
    }
}
