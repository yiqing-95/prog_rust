
There is one unusual rule about trait methods: the trait itself must be in scope. Other‐
wise, all its methods are hidden:

~~~rust

// use std::io::Write; // 添加这个 就有write_all 方法了
let mut buf: Vec<u8> = vec![];
buf.write_all(b"hello")?; // error: no method named `write_all`
~~~


vtable. In Rust, as in C++, the vtable is generated once, at compile time, and shared by all objects of the same type.


In fact, Rust’s subtraits are really just a shorthand for a bound on Self. A definition of Creature like this is exactly equivalent to the one shown earlier:

trait Creature where Self: Visible { ...
}


### associated constants
associated constants can’t be used with trait objects, since the compiler relies on type information about the implementation in order to pick the right value at compile time.