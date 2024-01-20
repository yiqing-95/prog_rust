Vec<T>
A growable, heap-allocated array of values of type T. About half of this chapter is dedicated to Vec and its many useful methods.
VecDeque<T>
Like Vec<T>, but better for use as a first-in-first-out queue. It supports efficiently adding and removing values at the front of the list as well as the back. This comes at the cost of making all other operations slightly slower.

BinaryHeap<T>
A priority queue. The values in a BinaryHeap are organized so that it’s always efficient to find and remove the maximum value.
HashMap<K, V>
A table of key-value pairs. Looking up a value by its key is fast. The entries are stored in an arbitrary order.
BTreeMap<K, V>
Like HashMap<K, V>, but it keeps the entries sorted by key. A BTreeMap<String, i32> stores its entries in String comparison order. Unless you need the entries to stay sorted, a HashMap is faster.
HashSet<T>
A set of values of type T. Adding and removing values is fast, and it’s fast to ask whether a given value is in the set or not.

BTreeSet<T>
Like HashSet<T>, but it keeps the elements sorted by value. Again, unless you need the data sorted, a HashSet is faster.


Several methods provide easy access to particular elements of a vector or slice (note that all slice methods are available on arrays and vectors too):

vec.resize(new_len, value)
Sets vec’s length to new_len. If this increases vec’s length, copies of value are added to fill the new space. The element type must implement the Clone trait.
vec.resize_with(new_len, closure)
Just like vec.resize, but calls the closure to construct each new element. It can be used with vectors of elements that are not Clone.
vec.truncate(new_len)
Reduces the length of vec to new_len, dropping any elements that were in the range vec[new_len..].
If vec.len() is already less than or equal to new_len, nothing happens. vec.clear()
Removes all elements from vec. It’s the same as vec.truncate(0).