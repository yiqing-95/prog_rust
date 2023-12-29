
Enums in Memory
In memory, enums with data are stored as a small integer tag, 
plus enough memory to hold all the fields of the largest variant.
The tag field is for Rust’s internal use. 
It tells which constructor created the value and therefore which fields it has.

## 可辩驳匹配

A refutable pattern is one that might not match, like Ok(x), 
which doesn’t match an error result, or '0' ..= '9', 
which doesn’t match the character 'Q'. Refutable pat‐ terns can be used in match arms,
because match is designed for them: if one pattern fails to match, it’s clear what happens next.

~~~rust
// ...handle just one enum variant specially
if let RoughTime::InTheFuture(_, _) = user.date_of_birth() { 
  user.set_time_traveler(true);
}
// ...run some code only if a table lookup succeeds
if let Some(document) = cache_map.get(&id) { 
    return send_cached_response(document);
}
// ...repeatedly try something until it succeeds
while let Err(err) = present_cheesy_anti_robot_task() { 
    log_robot_attempt(err);
// let the user try again (it might still be a human)
}
// ...manually loop over an iterator
while let Some(_) = lines.peek() {
    read_paragraph(&mut lines);
}
~~~