use std::fmt::Debug;

mod _0 {
    pub trait Iterator {
        type Item;
        fn next(&mut self) -> Option<Self::Item>;
    }

}

/// Loop over an iterator, storing the values in a new vector.
fn collect_into_vector<I: Iterator>(iter: I) -> Vec<I::Item> {
    let mut results = Vec::new();
    for value in iter {
        results.push(value);
    }
    results
}

/// Print out all the values produced by an iterator
// fn dump<I>(iter: I)
// where I: Iterator<Item=String>
fn dump<I>(iter: I) where I: Iterator, I::Item: Debug
{
    for (index, value) in iter.enumerate() {
        println!("{}: {:?}", index, value);
    }
}

fn dump2(iter: &mut dyn Iterator<Item=String>) {
    for (index, s) in iter.enumerate() {
        println!("{}: {:?}", index, s);
    }
}


fn cyclical_zip(v: Vec<u8>, u: Vec<u8>) -> impl Iterator<Item=u8> {
    v.into_iter().chain(u.into_iter()).cycle()
}