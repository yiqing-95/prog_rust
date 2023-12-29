
Expressions have values. Statements don’t.

Most of the control flow tools in C are statements. In Rust, they are all expressions.

Blocks are the most general kind of expression. A block produces a value and can be used anywhere a value is needed:
~~~rust

let display_name = match post.author() { 
Some(author) => author.name(),
None => {
let network_info = post.get_network_metadata() ?; let ip = network_info.client_address(); ip.to_string()
} };
~~~

The code after Some(author) => is the simple expression author.name(). 
The code after None => is a block expression. It makes no difference to Rust.

The value of the block is the value of its last expression, ip.to_string().


A block can also contain item declarations. An item is simply any declaration that could appear globally in a program or module, such as a fn, struct, or use.

## if let 

~~~rust
if let pattern = expr { block1
}else{
block2
}

~~~

It’s never strictly necessary to use if let, because match can do everything if let can do. An if let expression is shorthand for a match with just one pattern:
~~~rust
match expr {
pattern => { block1 } _=>{block2}
}

~~~

## loops
4种
~~~rust
while condition { block
}
while let pattern = expr { block
}
loop {
block
}
for pattern in iterable { block
}
~~~

Loops are expressions in Rust, but the value of a while or for loop is always (), so their value isn’t very useful. A loop expression can produce a value if you specify one.

### for-loop
~~~rust
let strings: Vec<String> = error_messages();
for s in strings { // each String is moved into s here...
println!("{}", s);
println!("{} error(s)", strings.len()); // error: use of moved value

// ok
for rs in &strings {
println!("String {:?} is at address {:p}.", *rs, rs);
}


for rs in &mut strings { // the type of rs is &mut String 
rs.push('\n'); // add a newline to each string
}

~~~

### break

~~~rust
// Each call to `next_line` returns either `Some(line)`, where
// `line` is a line of input, or `None`, if we've reached the end of // the input. Return the first line that starts with "answer: ".
// Otherwise, return "answer: nothing".
let answer = loop {
if let Some(line) = next_line() {
if line.starts_with("answer: ") {
break line; }
}else{
break "answer: nothing";
} };
~~~

### continue

~~~rust
// Read some data, one line at a time.
for line in input_lines {
let trimmed = trim_comments_and_whitespace(line); if trimmed.is_empty() {
// Jump back to the top of the loop and // move on to the next line of input. 
continue;
} ...
}
~~~

### babeled loop

~~~rust

'search:
for room in apartment {
for spot in room.hiding_spots() { if spot.contains(keys) {
println!("Your keys are {} in the {}.", spot, room);
break 'search; }
} }


// Find the square root of the first perfect square // in the series.
let sqrt = 'outer: loop {
let n = next_number(); foriin1..{
letsquare=i*i; ifsquare==n{
// Found a square root.
break 'outer i; }
ifsquare>n{
// `n` isn't a perfect square, try the next break;
} }
};

~~~

## return 表达式

~~~rust
let output = File::create(filename)?;
// We explained that this is shorthand for a match expression:
let output = match File::create(filename) { Ok(f) => f,
Err(err) => return Err(err) };
~~~