
## COPY
we pointed out the excep‐ tion: simple types that don’t own any resources can be Copy types, where assignment makes a copy of the source, rather than moving the value and leaving the source uninitialized.
At that time, we left it vague exactly what Copy was, but now we can tell you: a type is Copy if it implements the std::marker::Copy marker trait, which is defined as follows:
trait Copy: Clone { }
This is certainly easy to implement for your own types:
impl Copy for MyType { }
But because Copy is a marker trait with special meaning to the language, Rust permits a type to implement Copy only if a shallow byte-for-byte copy is all it needs. Types that own any other resources, like heap buffers or operating system handles, cannot implement Copy.

Any type that implements the Drop trait cannot be Copy. Rust presumes that if a type needs special cleanup code, it must also require special copying code and thus can’t be Copy.


## deref
The Deref and DerefMut traits play another role as well. Since deref takes a &Self reference and returns a &Self::Target reference, Rust uses this to automatically con‐ vert references of the former type into the latter. In other words, if inserting a deref call would prevent a type mismatch, Rust inserts one for you. Implementing

DerefMut enables the corresponding conversion for mutable references. These are called the deref coercions: one type is being “coerced” into behaving as another.

The Deref and DerefMut traits are designed for implementing smart pointer types, like Box, Rc, and Arc, and types that serve as owning versions of something you would also frequently use by reference, the way Vec<T> and String serve as owning versions of [T] and str. You should not implement Deref and DerefMut for a type just to make the Target type’s methods appear on it automatically, the way a C++ base class’s methods are visible on a subclass. This will not always work as you expect and can be confusing when it goes awry.

## default

All of Rust’s collection types—Vec, HashMap, BinaryHeap, and so on—implement Default, with default methods that return an empty collection. 

If a type T implements Default, then the standard library implements Default auto‐ matically for Rc<T>, Arc<T>, Box<T>, Cell<T>, RefCell<T>, Cow<T>, Mutex<T>, and RwLock<T>. 
The default value for the type Rc<T>, for example, is an Rc pointing to the default value for type T.