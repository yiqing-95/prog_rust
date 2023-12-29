
The standard library’s error types do not include a stack trace, 
but the popular anyhow crate provides a ready-made error type that does, when used with an unstable version of the Rust compiler. (As of Rust 1.50, the standard library’s functions for capturing backtraces were not yet stabilized.)

### 错误传播
`?` 语法糖

~~~rust
let weather = get_weather(hometown)?;
// 跟下面的一样效果
let weather = match get_weather(hometown) { 
Ok(success_value) => success_value, 
Err(err) => return Err(err)
};

~~~
The only differences between this and the ? operator are some fine points involving types and conversions.
二者区别在于问好❓会涉及错误类型及其转换问题


The macro expands to a match expression
宏会展开成match表达式

也可用于Option
? also works similarly with the Option type. In a function that returns Option, you can use ? to unwrap a value and return early in the case of None:
let weather = get_weather(hometown).ok()?;

``thiserror`` 在错误类型转换上会是个很好的帮手
All of the standard library error types can be converted to the type Box<dyn std::error::Error + Send + Sync + 'static>.