struct Selector<T> {
    /// Elements available in this `Selector`.
    elements: Vec<T>,
    /// The index of the "current" element in `elements`. A `Selector` /// behaves like a pointer to the current element.
    current: usize,
}


use std::ops::{Deref, DerefMut};

impl<T> Deref for Selector<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.elements[self.current]
    }
}

impl<T> DerefMut for Selector<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.elements[self.current]
    }
}

#[test]
fn test_deref() {
    let mut s = Selector {
        elements: vec!['x', 'y', 'z'],
        current: 2,
    };
// Because `Selector` implements `Deref`, we can use the `*` operator to // refer to its current element.
    assert_eq!(*s, 'z');
// Assert that 'z' is alphabetic, using a method of `char` directly on a // `Selector`, via deref coercion.
    assert!(s.is_alphabetic());
// Change the 'z' to a 'w', by assigning to the `Selector`'s referent.
    *s = 'w';

    assert_eq!(s.elements, ['x', 'y', 'w']);

    let s = Selector { elements: vec!["good", "bad", "ugly"], current: 2 };
    fn show_it(thing: &str) { println!("{}", thing); }
    show_it(&s);

    use std::fmt::Display;
    fn show_it_generic<T: Display>(thing: T) { println!("{}", thing); }
    // show_it_generic(&s); // 不能工作

    show_it_generic(&s as &str);

    show_it_generic(&*s);
}