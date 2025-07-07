#[cfg(test)]
mod tests {
    use crate::node::Node;

    #[test]
    fn new() {
        let node = Node::new("test_key", 42);
        assert_eq!(node.key, "test_key");
        assert_eq!(node.data, 42);
        assert!(node.next.is_none());
    }

    #[test]
    fn set_next() {
        let mut bucket1 = Node::new("bucket1", 1);
        let bucket2 = Node::new("bucket2", 2);
        bucket1.set_next(bucket2);
        assert!(bucket1.get_next().is_some());
        assert_eq!(bucket1.get_next().unwrap().key, "bucket2");
    }
    
    #[test]
    fn get_next() {
        let mut bucket1 = Node::new("bucket1", 1);
        let bucket2 = Node::new("bucket2", 2);
        bucket1.set_next(bucket2);
        let next_bucket = bucket1.get_next();
        assert!(next_bucket.is_some());
        assert_eq!(next_bucket.unwrap().key, "bucket2");
    }
}