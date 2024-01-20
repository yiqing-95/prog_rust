

## Iterator and IntoIterator
An iterator is any value that implements the std::iter::Iterator trait:

~~~rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>; ... // many default methods
}

trait IntoIterator where Self::IntoIter: Iterator<Item=Self::Item> {
    type Item;
    type IntoIter: Iterator;
    fn into_iter(self) -> Self::IntoIter;
}
~~~

IntoIter is the type of the iterator value itself, and Item is the type of value it pro‐ duces. We call any type that implements IntoIterator an iterable, because it’s some‐ thing you could iterate over if you asked.


there is no iter method on the &str string slice type. Instead, if s is a &str, then s.bytes() returns an iterator that produces each byte of s, whereas s.chars() interprets the contents as UTF-8 and produces each Unicode character.



Since a for loop applies IntoIterator::into_iter to its operand, these three imple‐ mentations are what create the following idioms for iterating over shared or mutable references to a collection, or consuming the collection and taking ownership of its elements:
for element in &collection { ... } for element in &mut collection { ... } for element in collection { ... }
Each of these simply results in a call to one of the IntoIterator implementations lis‐ ted here.
Not every type provides all three implementations. For example, HashSet, BTreeSet, and BinaryHeap don’t implement IntoIterator on mutable references, since modify‐ ing their elements would probably violate the type’s invariants: the modified value might have a different hash value, or be ordered differently with respect to its neigh‐ bors, so modifying it would leave it incorrectly placed. Other types do support muta‐ tion, but only partially. For example, HashMap and BTreeMap produce mutable reference to their entries’ values, but only shared references to their keys

Slices implement two of the three IntoIterator variants; since they don’t own their elements, there is no “by value” case. Instead, into_iter for &[T] and &mut [T] returns an iterator that produces shared and mutable references to the elements.

## adapter

There are two important points to notice about iterator adapters.
First, simply calling an adapter on an iterator doesn’t consume any items; it just returns a new iterator, ready to produce its own items by drawing from the first itera‐ tor as needed. In a chain of adapters, the only way to make any work actually get done is to call next on the final iterator.

