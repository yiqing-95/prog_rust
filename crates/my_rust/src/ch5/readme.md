
Rust refers to creating a reference to some value as bor‐ rowing the value:
what you have borrowed, you must eventually return to its owner.

必须还回去 有点像栈 函数返回的感觉 

A reference lets you access a value without affecting its ownership. References come in two kinds:
• A shared reference lets you read but not modify its referent. However, you can have as many shared references to a particular value at a time as you like. The expression &e yields a shared reference to e’s value; if e has the type T, then &e has the type &T, pronounced “ref T.” Shared references are Copy.
• If you have a mutable reference to a value, you may both read and modify the value. However, you may not have any other references of any sort to that value active at the same time. 
The expression &mut e yields a mutable reference to e’s value;
you write its type as &mut T, which is pronounced “ref mute T.” Mutable references are not Copy.

## . 操作符

in Rust you use the & and * operators to create and follow ref‐ erences, 
with the exception of the . operator, which borrows and dereferences implicitly.


## 生命周期
Rust tries to assign each reference type in your program a lifetime that meets the con‐ straints imposed by how it is used.
A lifetime is some stretch of your program for which a reference could be safe to use:
a statement, an expression, the scope of some variable, or the like. 
Lifetimes are entirely figments of Rust’s compile-time imagina‐ tion. At run time, 
a reference is nothing but an address; its lifetime is part of its type and has no run-time representation.