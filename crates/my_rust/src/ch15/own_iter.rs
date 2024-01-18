struct I32Range {
    start: i32,
    end: i32,
}

impl Iterator for I32Range {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        if self.start >= self.end {
            return None;
        }
        let result = Some(self.start);
        self.start += 1;
        result
    }
}

#[test]
fn it_works() {
    let mut pi = 0.0;
    let mut numerator = 1.0;
    // a for loop uses IntoIterator::into_iter to convert its operand into an iterator. But the standard library provides a blanket implementation of IntoIterator for every type that implements Iterator, so I32Range is ready for use
    for k in (I32Range { start: 0, end: 14 }) {
        pi += numerator / (2 * k + 1) as f64;
        numerator /= -3.0;
    }
    pi *= f64::sqrt(12.0);
// IEEE 754 specifies this result exactly.
    assert_eq!(pi as f32, std::f32::consts::PI);
}

enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}

struct TreeNode<T> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

impl <T: Ord>  BinaryTree<T> {
    fn add(&mut self, value: T) {
        match *self {
            BinaryTree::Empty => {
                *self = BinaryTree::NonEmpty(Box::new(TreeNode {
                    element: value,
                    left: BinaryTree::Empty,
                    right: BinaryTree::Empty,
                }))
            }
            BinaryTree::NonEmpty(ref mut node) => {
                if value <= node.element {
                    node.left.add(value);
                } else {
                    node.right.add(value);
                }
            }
        }
    }

}

use self::BinaryTree::*;

// The state of an in-order traversal of a `BinaryTree`.
struct TreeIter<'a, T> {
    // A stack of references to tree nodes. Since we use `Vec`'s
// `push` and `pop` methods, the top of the stack is the end of the
// vector.
//
// The node the iterator will visit next is at the top of the stack,
// with those ancestors still unvisited below it. If the stack is empty,
// the iteration is over.
    unvisited: Vec<&'a TreeNode<T>>,
}

impl<'a, T: 'a> TreeIter<'a, T> {
    fn push_left_edge(&mut self, mut tree: &'a BinaryTree<T>) {
        while let NonEmpty(ref node) = *tree {
            self.unvisited.push(node);
            tree = &node.left;
        }
    }
}

impl<T> BinaryTree<T> {
    fn iter(&self) -> TreeIter<T> {
        let mut iter = TreeIter { unvisited: Vec::new() };
        iter.push_left_edge(self);
        iter
    }
}


impl<'a, T: 'a> IntoIterator for &'a BinaryTree<T> {
    type Item = &'a T;
    type IntoIter = TreeIter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> Iterator for TreeIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
// Find the node this iteration must produce,
// or finish the iteration. (Use the `?` operator // to return immediately if it's `None`.)
        let node = self.unvisited.pop()?;
// After `node`, the next thing we produce must be the leftmost // child in `node`'s right subtree, so push the path from here // down. Our helper method turns out to be just what we need.
        self.push_left_edge(&node.right);
// Produce a reference to this node's value.
        Some(&node.element)
    }
}

#[test]
fn test_tree_iter() {
    // Build a small tree.
    let mut tree = BinaryTree::Empty;
    tree.add("jaeger");
    tree.add("robot");
    tree.add("droid");
    tree.add("mecha");
// Iterate over it.
    let mut v = Vec::new();
    for kind in &tree {
        v.push(*kind);
    }
    assert_eq!(v, ["droid", "jaeger", "mecha", "robot"]);

    assert_eq!(tree.iter()
                   .map(|name| format!("mega-{}", name))
                   .collect::<Vec<_>>(), vec!["mega-droid", "mega-jaeger",
                                              "mega-mecha", "mega-robot"]);
}