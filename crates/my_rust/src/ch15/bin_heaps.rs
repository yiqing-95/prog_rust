use std::cmp::Ordering;

// BinaryHeap::new(), .len(), .is_empty(), .capacity(), .clear(), and .append(&mut heap2).
#[test]
fn test_ops() {
    use std::collections::binary_heap::PeekMut;

    // let heap = BinaryHeap::new();
    use std::collections::BinaryHeap;
    let mut heap = BinaryHeap::from(vec![2, 3, 8, 6, 9, 5, 4]);

    assert_eq!(heap.peek(), Some(&9));
    assert_eq!(heap.pop(), Some(9));

    assert_eq!(heap.pop(), Some(8));
    assert_eq!(heap.pop(), Some(6));
    assert_eq!(heap.pop(), Some(5));

    // if let Some(top) = heap.peek_mut() {
    //     if *top > 10 {
    //         PeekMut::pop(top);
    //     }
    // }
    for item in &heap{
        println!("{item}");
    }

    while let Some(task) = &heap.pop() {
        // handle(task);
        println!("most important task is : {}",task);
    }

    println!("len is : {}",&heap.len());
}

struct MyTask{
    // name: &str,
    some_key: i32,
}

impl PartialEq<Self> for MyTask {
    fn eq(&self, other: &Self) -> bool {
        self.some_key == other.some_key
    }
}

impl Eq for MyTask {}



impl PartialOrd<Self> for MyTask {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
         self.some_key.partial_cmp(&other.some_key)
    }
}

impl Ord for MyTask {
    fn cmp(&self, other: &Self) -> Ordering {
      self.some_key.cmp(&other.some_key)
    }
}