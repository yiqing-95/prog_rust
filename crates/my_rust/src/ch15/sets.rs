use std::collections::{BTreeSet, HashSet};

#[test]
fn it_works(){
    let large_vector = vec!["hi"];
    let mut large_hash_set = HashSet::new() ;
    // let mut large_hash_set = BTreeSet::new() ;

    large_hash_set.insert(&"needle");

    let b1 = large_vector.contains(&"needle"); // slow, checks every element
    let b2 = large_hash_set.contains(&"needle"); // fast, hash lookup

    println!("{b1} and {b2}");

    let set: HashSet<&str> = large_vector.into_iter().collect();
}