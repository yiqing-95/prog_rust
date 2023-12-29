In fact, every closure you write has its own type, because a closure may contain data: values either borrowed or stolen from enclosing scopes. This could be any number of variables, in any combination of types. So every closure has an ad hoc type created by the compiler, 
large enough to hold that data.
No two closures have exactly the same type. But every closure implements an Fn trait;


Rust’s closures are designed to be fast: faster than function pointers, fast enough that you can use them even in red-hot, performance-sensitive code. 
If you’re familiar with C++ lambdas, you’ll find that Rust closures are just as fast and compact, but safer.

In most languages, closures are allocated in the heap, dynamically dispatched, and garbage collected. So creating, calling, and collecting each of them costs a tiny bit of extra CPU time. Worse, closures tend to rule out inlining, a key technique compilers use to eliminate function call overhead and enable a raft of other optimizations. All told, closures are slow enough in these languages that it can be worth manually removing them from tight inner loops.



Rust closures have none of these performance drawbacks. 
They’re not garbage collected. Like everything else in Rust, they aren’t allocated on the heap unless you put them in a Box, Vec, or other container. And since each closure has a distinct type, whenever the Rust compiler knows the type of the closure you’re calling, it can inline the code for that particular closure. This makes it OK to use closures in tight loops,

Often, the compiler can inline all calls to a closure,

闭包的内存使用类似小的一个struct类型 通常也是在栈内分配内存 ！
闭包经常从其环境中``借用``或者``偷取``值

FnMut
There is one more kind of closure, the kind that contains mutable data or mut references.

~~~rust
// Pseudocode for `Fn`, `FnMut`, and `FnOnce` traits. trait Fn() -> R {
fn call(&self) -> R; }
trait FnMut() -> R {
fn call_mut(&mut self) -> R;
}
trait FnOnce() -> R {
fn call_once(self) -> R;
}
~~~

• Fn is the family of closures and functions that you can call multiple times without restriction. This highest category also includes all fn functions.
• FnMut is the family of closures that can be called multiple times if the closure itself is declared mut.
• FnOnce is the family of closures that can be called once, if the caller owns the closure.

Every Fn meets the requirements for FnMut, and every FnMut meets the requirements for FnOnce.
Instead, Fn() is a subtrait of FnMut(), which is a subtrait of FnOnce().
This makes Fn the most exclusive and most powerful category.
FnMut and FnOnce are broader categories that include closures with usage restrictions.