use std::collections::HashSet;

#[test]
fn test_create() {
    // Create an empty vector
    let mut numbers: Vec<i32> = vec![];
    // Create a vector with given contents
    let words = vec!["step", "on", "no", "pets"];
    let mut buffer = vec![0u8; 1024]; // 1024 zeroed-out bytes
}

#[test]
fn test_from_iter() {
    // Convert another collection to a vector. TODO:
    // let my_vec = my_set.into_iter().collect::<Vec<String>>();
}

// Because returning a T by value would mean moving it, methods that access elements in place typically return those elements by reference.
#[test]
fn test_accessing() {
//     // Get a reference to an element
//     let first_line = &lines[0];
// // Get a copy of an element
//     let fifth_number = numbers[4]; // requires Copy let second_line = lines[1].clone(); // requires Clone
//     // Get a reference to a slice
//     let my_ref = &buffer[4..12]; // Get a copy of a slice
//     let my_copy = buffer[4..12].to_vec(); // requires Clone

    let v = vec!["step", "on", "no", "pets"];
    if let Some(item) = v.first() {
        println!("We got one! {}", item);
    }

    if let Some(item) = v.last() {
        println!("[last]: We got one! {}", item);
    }

    // #GET
    let slice = [0, 1, 2, 3];
    assert_eq!(slice.get(2), Some(&2));
    assert_eq!(slice.get(4), None);

    // #mut Variations of the preceding that borrow mut references:
    let mut slice = [0, 1, 2, 3];
    {
        let last = slice.last_mut().unwrap();  // type of last: &mut i32
        assert_eq!(*last, 3);
        *last = 100;
    }
    assert_eq!(slice, [0, 1, 2, 100]);
}

#[test]
fn makes_copies() {
    // Clones a whole slice, returning a new vector
    // This method is available only if the elements are cloneable, that is, where T: Clone.
    let v = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    assert_eq!(v.to_vec(),
               vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    assert_eq!(v[0..6].to_vec(),
               vec![1, 2, 3, 4, 5, 6]);
}

//  not present on arrays and slices, which can’t be resized once created.
#[test]
fn test_growing_shrinking() {
    // let v = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    println!("[len]: We got : {}", v.len());

    if v.is_empty() {
        println!("it is empty");
    } else {
        println!("it is not empty");
    }

    let mut v = Vec::with_capacity(10);
    v.push(10);
    v.push(1);
    if v.capacity() >= v.len() {
        println!("this is always true!");
    }

    // vec.reserve(n)|vec.reserve_exact(n)|vec.shrink_to_fit()

    let last = v.pop();
    if last.is_some() {
        println!("last element is : {}", last.unwrap());
    }

    // Inserts the given value at vec[index], sliding any existing values in vec[index..] one spot to the right to make room.
    // Panics if index > vec.len().
    v.insert(1, 5);
    println!("index:1 is {}", v.get(1).unwrap());

    println!("== before == len is {}", v.len());
    // Both .insert() and .remove() are slower the more elements have to be shifted.
    v.remove(1);
    println!("== after == len is {}", v.len());

    // vec.resize(new_len, value)| vec.resize_with(new_len, closure) |vec.truncate(new_len)| vec.clear()
    // vec.extend(iterable) |vec.split_off(index)   | vec.append(&mut vec2)
    // vec.drain(range)
    // vec.retain(test_fn_or_closure)
    let mut byte_vec = b"Misssssssissippi".to_vec();
    byte_vec.dedup();
    assert_eq!(&byte_vec, b"Misisipi");
    // #dedup2

    let mut byte_vec = b"Misssssssissippi".to_vec();
    let mut seen = HashSet::new();
    byte_vec.retain(|r| seen.insert(*r));
    assert_eq!(&byte_vec, b"Misp");

    // | vec.dedup_by(same)| vec.dedup_by_key(key)
}

#[test]
fn test_join() {
    assert_eq!([[1, 2], [3, 4], [5, 6]].concat(), vec![1, 2, 3, 4, 5, 6]);

    assert_eq!([[1, 2], [3, 4], [5, 6]].join(&0), vec![1, 2, 0, 3, 4, 0, 5, 6]);
}

#[test]
fn test_splitting() {
    let i = 1;
    let j = 3;
    let v = vec![0, 1, 2, 3];
    let a = &v[i];
    let b = &v[j];
    let mid = v.len() / 2;
    let front_half = &v[..mid];
    let back_half = &v[mid..];
}

mod swappings {
    #[test]
    fn test_swapping() {
        // let mut s = ["a", "b", "c", "d", "e"];
        let mut s = vec![1, 2, 3];
        s.swap(1, 2);

        println!("{:#?}", s);

        let mut s2 = vec![5, 6, 7];
        s.swap_with_slice(&mut s2);

        println!("{:#?}", s);

        let old_len = s.len();
        s.swap_remove(0);
        println!("{:#?}", s);
        assert_eq!(s.len(), old_len - 1);
    }
}

mod sorting_search {
    #[test]
    fn test_ordering() {
        let mut s = ["x", "y", "z", "a", "b", "c", "d", "e"];
        s.sort();

        println!("{:?}", s);

        let mut students = ["zhang san", "li si"];
        // students.sort_by(|a,b|  a.cmp(b)) ;
        students.sort_by(|a, b| b.cmp(a));
        println!("{:?}", students);

        students.sort_by_key(|item| *item);
        println!("{:?}", students);

        students.reverse();
        println!("{:?}", students);
    }

    #[test]
    fn test_searching() {
        let mut students = ["zhang san", "wang ma zi", "li si"];

        // 未排序也可以搜索！🔍
        if students.contains(&"zhang san") {
            println!("ok find it!");
        }
        let loc = students.iter().position(|s| {
            // return *s == "zhang san" ;
            return *s == "li si";
        });
        println!("got it at :{}", loc.unwrap());

        students.sort()
        ;

        let s = students.binary_search(&"li si");
        println!("{s:?}");

        // #?: slice.binary_search_by_key(&value, key)
        // methods that work on floating-point data, use the ord_subset crate.
    }
}

mod compare_slices {
    // If a type T supports the == and != operators (the PartialEq trait), then arrays [T; N], slices [T], and vectors Vec<T> support them too.

    #[test]
    fn test_eq() {
        let mut students = ["zhang san", "wang ma zi", "li si"];
        let s2 = ["zhang san", "wang ma zi", "li si"];

        if students == s2 {
            println!("these two slices are equal");
        }
        // If T supports the operators <, <=, >, and >= (the PartialOrd trait,  ), then arrays, slices, and vectors of T do too. Slice comparisons are lexicographical.
    }

    #[test]
    fn test_comparisons() {
        assert_eq!([1, 2, 3, 4].starts_with(&[1, 2]), true);
        assert_eq!([1, 2, 3, 4].starts_with(&[2, 3]), false);

        assert_eq!([1, 2, 3, 4].ends_with(&[3, 4]), true);
    }

    #[test]
    fn test_random() {
        use rand::seq::SliceRandom;
        use rand::thread_rng;

        let mut my_vec = [1, 3, 4, 6, 7, 8];
        my_vec.shuffle(&mut thread_rng());

        println!("{my_vec:?}");

        let mut rng = thread_rng();
        let r = my_vec.choose(&mut rng);
        println!("{r:?}");
    }
}

#[test]
fn invalidation_error() {
    let mut my_vec = vec![1, 3, 5, 7, 9];
    for (index, &val) in my_vec.iter().enumerate() {
        if val > 4 {
            // my_vec.remove(index); // error: can't borrow `my_vec` as mutable }
        }
        println!("{:?}", my_vec);
    }

    //
    my_vec.retain(|&val| val <= 4); 
}