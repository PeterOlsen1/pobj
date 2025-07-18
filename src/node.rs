use crate::traits::CloneableAny;

pub struct Node {
    pub data: Box<dyn CloneableAny>, // Updated to use Box<dyn CloneableAny>
    pub key: String,
    pub next: Option<Box<Node>>, // Pointer to the next node
}

impl Clone for Node {
    ///this method might just clone the entire list
    /// if used, be careful of the self.next.clone()
    /// 
    /// try to see if we can get by without using this
    fn clone(&self) -> Self {
        let mut out = Node::new(&self.key, self.data.clone_box());
        out.next = self.next.clone();
        out
    }
}

impl Node {
    pub fn new(key: &str, data: Box<dyn CloneableAny>) -> Self {
        Node {
            key: key.to_string(),
            data,
            next: None,
        }
    }

    pub fn set_next(&mut self, next: Node) {
        self.next = Some(Box::new(next));
    }

    pub fn get_next(&self) -> Option<&Node> {
        self.next.as_deref()
    }
}
