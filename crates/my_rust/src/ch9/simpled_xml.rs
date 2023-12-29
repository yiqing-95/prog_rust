use std::rc::Rc;
struct Node { tag: String,
    children: Vec<Rc<Node>>
}
impl Node {
    fn new(tag: &str) -> Node {
        Node {
            tag: tag.to_string(), children: vec![],
        } }
}