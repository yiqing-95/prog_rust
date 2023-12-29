pub struct Queue<T> {
    older: Vec<T>,
    younger: Vec<T>,
}

impl<T> Queue<T> {
    // pub fn new() -> Queue<T> {
    //     Queue { older: Vec::new(), younger: Vec::new() }
    // }
    pub fn new() -> Self {
        Queue { older: Vec::new(), younger: Vec::new() }
    }

    pub fn push(&mut self, t: T) {
        self.younger.push(t);
    }
    pub fn is_empty(&self) -> bool {
        self.older.is_empty() && self.younger.is_empty()
    }
    // ...
}


impl Queue<f64> {

    fn sum(&self) -> f64 {
        // ...
        0.0
    }
}

#[test]
fn test_new() {
    let mut q = Queue::<char>::new();

    let mut q = Queue::new();
    let mut r = Queue::new();
    q.push("CAD"); // apparently a Queue<&'static str> r.push(0.74); // apparently a Queue<f64>
    q.push("BTC"); // Bitcoins per USD, 2019-6
    r.push(13764.0); // Rust fails to detect irrational exuberance
}