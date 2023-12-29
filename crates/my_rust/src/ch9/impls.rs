/// A first-in, first-out queue of characters.
pub struct Queue {
    older: Vec<char>,
    // older elements, eldest last.
    younger: Vec<char>, // younger elements, youngest last.
}

impl Queue {
    /// Push a character onto the back of a queue.
    pub fn push(&mut self, c: char) {
        self.younger.push(c);
    }
    pub fn pop(&mut self) -> Option<char> {
        if self.older.is_empty() {
            if self.younger.is_empty() {
                return None;
            }
            // Bring the elements in younger over to older, and put them in // the promised order.
            use std::mem::swap;
            swap(&mut self.older, &mut self.younger);
            self.older.reverse();
        }
// Now older is guaranteed to have something. Vec's pop method // already returns an Option, so we're set.
        self.older.pop()
    }
}

#[test]
fn test_queue_works() {
    let mut q = Queue { older: Vec::new(), younger: Vec::new() };
    q.push('0');
    q.push('1');
    assert_eq!(q.pop(), Some('0'));
    q.push('∞');
    assert_eq!(q.pop(), Some('1'));
    assert_eq!(q.pop(), Some('∞'));
    assert_eq!(q.pop(), None);
}

impl Queue {
    pub fn is_empty(&self) -> bool {
        self.older.is_empty() && self.younger.is_empty()
    }
}

#[test]
fn test_queue_is_empty() {
    let mut q = Queue { older: Vec::new(), younger: Vec::new() };
    assert!(q.is_empty());
    q.push('☉');
    assert!(!q.is_empty());
}

impl Queue {
    pub fn split(self) -> (Vec<char>, Vec<char>) {
        (self.older, self.younger)
    }
}

#[test]
fn test_split() {
    let mut q = Queue { older: Vec::new(), younger: Vec::new() };
    q.push('P');
    q.push('D');
    assert_eq!(q.pop(), Some('P'));
    q.push('X');
    let (older, younger) = q.split(); // q is now uninitialized.
    assert_eq!(older, vec!['D']);
    assert_eq!(younger, vec!['X']);
}

// type-associated functions. other associated functions that serve as constructors, like Vec::with_capacity.
impl Queue {
    pub fn new() -> Queue {
        Queue { older: Vec::new(), younger: Vec::new() }
    }
}

// associated consts.
pub struct Vector2 {
    x: f32,
    y: f32,
}

impl Vector2 {
    const ZERO: Vector2 = Vector2 { x: 0.0, y: 0.0 };
    const UNIT: Vector2 = Vector2 { x: 1.0, y: 0.0 };
}

impl Vector2 {
    const NAME: &'static str = "Vector2";
    const ID: u32 = 18;
}
