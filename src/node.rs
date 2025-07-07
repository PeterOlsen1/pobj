#[derive(Clone)]
pub struct Node<Any> {
    pub data: Any,
    pub key: String,
    pub next: Option<Box<Node<Any>>>,
}

impl<Any: Clone> Node<Any> {
    pub fn new(key: &str, data: Any) -> Self {
        Node {
            key: key.to_string(),
            data,
            next: None,
        }
    }

    pub fn set_next(&mut self, next: Node<Any>) {
        self.next = Some(Box::new(next));
    }

    pub fn get_next(&self) -> Option<&Node<Any>> {
        self.next.as_deref()
    }
}
