mod adapters;
mod own_iter;

fn triangle(n: i32) -> i32 {
    let mut sum = 0;
    // A RangeInclusive<i32> is an iterator that produces the integers from its start value to its end value (both inclu‐ sive),
    // so you can use it as the operand of the for loop to sum the values from 1 to n.
    for i in 1..=n {
        sum += i;
    }
    sum

    //  等价👇下面实现
    //     (1..=n).fold(0, |sum, item| sum + item)
}

#[test]
fn iter_over_vec() {
    println!("There's:");
    let v = vec!["antimony", "arsenic", "aluminum", "selenium"];
    // Since a for loop applies IntoIterator::into_iter to its operand
    for element in &v {
        println!("{}", element);
    }

    // under-the-hood

    let mut iterator = (&v).into_iter();
    while let Some(element) = iterator.next() {
        println!("{}", element);
    }
}

#[test]
fn manually_iter() {
    let v = vec![4, 20, 12, 8, 6];
    let mut iterator = v.iter();
    assert_eq!(iterator.next(), Some(&4));
    assert_eq!(iterator.next(), Some(&20));
    assert_eq!(iterator.next(), Some(&12));
    assert_eq!(iterator.next(), Some(&8));
    assert_eq!(iterator.next(), Some(&6));
    assert_eq!(iterator.next(), None);
}

#[test]
fn iter_mut() {
    use std::ffi::OsStr;
    use std::path::Path;
    let path = Path::new("C:/Users/JimB/Downloads/Fedora.iso");
    let mut iterator = path.iter();
    assert_eq!(iterator.next(), Some(OsStr::new("C:")));
    assert_eq!(iterator.next(), Some(OsStr::new("Users")));
    assert_eq!(iterator.next(), Some(OsStr::new("JimB")));
}

#[test]
fn into_iter() {
    // You should usually use HashSet, but its iteration order is // nondeterministic, so BTreeSet works better in examples.
    use std::collections::BTreeSet;
    let mut favorites = BTreeSet::new();
    favorites.insert("Lucy in the Sky With Diamonds".to_string());
    favorites.insert("Liebesträume No. 3".to_string());
    let mut it = favorites.into_iter();
    assert_eq!(it.next(), Some("Liebesträume No. 3".to_string()));
    assert_eq!(it.next(), Some("Lucy in the Sky With Diamonds".to_string()));
    assert_eq!(it.next(), None);
}

use std::fmt::Debug;

fn dump<T, U>(t: T)
    where T: IntoIterator<Item=U>,
          U: Debug
{
    for u in t {
        println!("{:?}",
                 u);
    }
}

#[test]
fn test_from_fn() {
    use rand::random; // In Cargo.toml dependencies: rand = "0.7"
    use std::iter::from_fn;
// Generate the lengths of 1000 random line segments whose endpoints // are uniformly distributed across the interval [0, 1]. (This isn't a // distribution you're going to find in the `rand_distr` crate, but // it's easy to make yourself.)
    let lengths: Vec<f64> =
        from_fn(|| Some((random::<f64>() - random::<f64>()).abs())).take(1000)
            .collect();
}

#[test]
fn test_successors() {
    use num::Complex;
    use std::iter::successors;
    fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
        let zero = Complex { re: 0.0, im: 0.0 };
        successors(Some(zero), |&z| { Some(z * z + c) })
            .take(limit)
            .enumerate()
            .find(|(_i, z)| z.norm_sqr() > 4.0).map(|(i, _z)| i)
    }
}

#[test]
fn test_capture_env() {
    fn fibonacci() -> impl Iterator<Item=usize> {
        let mut state = (0, 1);
        std::iter::from_fn(move || {
            state = (state.1, state.0 + state.1);
            Some(state.0)
        })
    }
    assert_eq!(fibonacci().take(8).collect::<Vec<_>>(),
               vec![1, 1, 2, 3, 5, 8, 13, 21]);
}

#[test]
fn test_drain() {
    use std::iter::FromIterator;
    let mut outer = "Earth".to_string();
    // On types that can be indexed by a range, like Strings, vectors, and VecDeques,
    // the drain method takes a range of elements to remove, rather than draining the entire sequence
    let inner = String::from_iter(outer.drain(1..4));
    assert_eq!(outer, "Eh");
    assert_eq!(inner, "art");
}

mod iter_in_std {
    use std::collections::{BTreeMap, HashMap};
    use super::*;

    #[test]
    fn iter_sources() {
        let iter = 1..10;
        let iter_source = (1..10).step_by(2);
        dump(iter);
        dump(iter_source);

        // let iter_source = 1.. ;
        // dump(iter_source);

        let iter_source = 1..=10;
        dump(iter_source);
    }

    #[test]
    fn iter_sources2() {
        let iter_source = Some(10).iter();
        dump(iter_source);

        // let is: Result<&str,&str> = Ok("blah");
        let is: Result<&str, ()> = Ok("blah");
        // let iter_source = Ok("blah").iter();
        let iter_source = is.iter();
        dump(iter_source);
    }

    #[test]
    fn iter_sources3() {
        let v = vec![1, 2, 3, 4, 5, 6, 7];
        let iter_source = v.windows(2);
        dump(iter_source);

        let iter_source = v.chunks(2);
        dump(iter_source);

        let mut v = v.clone();
        let iter_source = v.chunks_mut(2);
        dump(iter_source);

        let mut v = v.clone();
        let iter_source = v.split(|item| *item == 2);
        dump(iter_source);
        let mut v = v.clone();
        let iter_source = v.split_mut(|item| *item == 2);
        dump(iter_source);

        let mut v = v.clone();
        let iter_source = v.rsplit(|item| *item == 2);
        dump(iter_source);

        let mut v = v.clone();
        let iter_source = v.rsplitn(2, |item| *item % 2 == 0);
        dump(iter_source);
    }

    #[test]
    fn iter_sources_string_str() {
        let s = "some string 你好呀 😄\r\n second 行".to_string();

        let iter_source = s.bytes();

        let s = s.clone();
        let iter_rouce = s.chars();
        dump(iter_source);

        let s = s.clone();
        let iter_source = s.split_whitespace();
        dump(iter_source);

        let s = s.clone();
        let iter_source = s.lines();
        dump(iter_source);

        let s = s.clone();
        let iter_source = s.split('/');
        dump(iter_source);

        let s = s.clone();
        let iter_source = s.matches(char::is_numeric);
        dump(iter_source);

        // dump(iter_source) ;


        // let s = "some string"  ;
    }


    #[test]
    fn iter_sources_collections() {
        let mut coll = HashMap::new();
        coll.insert("hello", "world");

        let iter_source = coll.keys();
        dump(iter_source);

        let map = HashMap::from([
            ("a", 1),
            ("b", 2),
            ("c", 3),
        ]);
        dump(map.keys());

        let map = BTreeMap::from([
            ("a", 1),
            ("b", 2),
            ("c", 3),
        ]);
        dump(map.keys());

        let mut map = BTreeMap::from([
            ("a", 1),
            ("b", 2),
            ("c", 3),
        ]);
        dump(map.values_mut());
    }
}

mod popular_adapters {
    // 有点像打仗 对待队伍
    // make sequences of values from other sequences: truncation, skipping, combination, reversal, concatenation, repetition, and more.

    /*
    fn map<B, F>(self, f: F) -> impl Iterator<Item=B> where Self: Sized, F: FnMut(Self::Item) -> B;
fn filter<P>(self, predicate: P) -> impl Iterator<Item=Self::Item> where Self: Sized, P: FnMut(&Self::Item) -> bool;
     */

    use crate::ch15::dump;

    // A map iterator passes each item to its closure by value and, in turn, passes along ownership of the closure’s result to its consumer.
    // A filter iterator passes each item to its closure by shared reference, retaining ownership in case the item is selected to be passed on to its consumer.
    // This is why the example must dereference s to compare it with "iguanas": the filter iterator’s item type is &str, so the type of the closure’s argument s is &&str.
    #[test]
    fn test_map() {
        let text = " ponies \n giraffes\niguanas \nsquid".to_string();
        let v: Vec<&str> = text.lines()
            .map(str::trim)
            .collect();
        assert_eq!(v, ["ponies", "giraffes", "iguanas", "squid"]);
    }

    #[test]
    fn test_filter() {
        let text = " ponies \n giraffes\niguanas \nsquid".to_string();
        let v: Vec<&str> = text.lines()
            .map(str::trim)
            .filter(|s| *s != "iguanas").collect();
        assert_eq!(v, ["ponies", "giraffes", "squid"]);
    }

    #[test]
    fn test_filter_map() {
        use std::str::FromStr;
        let text = "1\nfrond .25 289\n3.1415 estuary\n";
        for number in text
            .split_whitespace()
            .filter_map(|w| f64::from_str(w).ok()) {
            println!("{:4.2}", number.sqrt());
        }

        // let text = "1\nfrond .25 289\n3.1415 estuary\n";
        // let r = text.split_whitespace()
        //     .map(|w| f64::from_str(w))
        //     .filter(|r| r.is_ok())
        //     .map(|r| r.unwrap());
    }

    #[test]
    fn test_flat_map() {
        use std::collections::HashMap;
        let mut major_cities = HashMap::new();
        major_cities.insert("Japan", vec!["Tokyo", "Kyoto"]);
        major_cities.insert("The United States", vec!["Portland", "Nashville"]);
        major_cities.insert("Brazil", vec!["São Paulo", "Brasília"]);
        major_cities.insert("Kenya", vec!["Nairobi", "Mombasa"]);
        major_cities.insert("The Netherlands", vec!["Amsterdam", "Utrecht"]);
        let countries = ["Japan", "Brazil", "Kenya"];
        // One way to look at this would be to say that, for each country, we retrieve the vector of its cities, concatenate all the vectors together into a single sequence, and print that.
        for &city in countries.iter().flat_map(|country| &major_cities[country]) {
            println!("{}", city);
        }
    }

    #[test]
    fn test_flatten() {
        use std::collections::BTreeMap;
// A table mapping cities to their parks: each value is a vector.
        let mut parks = BTreeMap::new();
        parks.insert("Portland", vec!["Mt. Tabor Park", "Forest Park"]);
        parks.insert("Kyoto", vec!["Tadasu-no-Mori Forest", "Maruyama Koen"]);
        parks.insert("Nashville", vec!["Percy Warner Park", "Dragon Park"]);
// Build a vector of all parks. `values` gives us an iterator producing // vectors, and then `flatten` produces each vector's elements in turn.
        let all_parks: Vec<_> = parks.values().flatten().cloned().collect();
        assert_eq!(all_parks,
                   vec!["Tadasu-no-Mori Forest", "Maruyama Koen", "Percy Warner Park",
                        "Dragon Park", "Mt. Tabor Park", "Forest Park"]);

        // ⚠️ 怪用法
        // This works because Option itself implements IntoIterator, representing a sequence of either zero or one elements. The None elements contribute nothing to the iteration, whereas each Some element contributes a single value. Similarly, you can use flatten to iterate over Option<Vec<...>> values: None behaves the same as an empty vector.
        assert_eq!(vec![None, Some("day"), None, Some("one")].into_iter()
                       .flatten().collect::<Vec<_>>(), vec!["day", "one"]);
    }

    #[test]
    fn test_take_while() {
        let message = "To: jimb\r\n\
From: superego <editor@oreilly.com>\r\n\
\r\n\
Did you get any writing done today?\r\n\
When will you stop wasting time plotting fractals?\r\n";
        for header in message.lines().take_while(|l| !l.is_empty()) {
            println!("{}", header);
        }
    }

    #[test]
    fn test_skip_while() {
        for arg in std::env::args().skip(1) {
            //...
        }

        let message = "To: jimb\r\n\
From: superego <editor@oreilly.com>\r\n\
\r\n\
Did you get any writing done today?\r\n\
When will you stop wasting time plotting fractals?\r\n";
        // for header in message.lines().take_while(|l| !l.is_empty()) {
        //     println!("{}", header);
        // }
        for body in message.lines().skip_while(|l| !l.is_empty()).skip(1) {
            println!("{}", body);
        }
    }

    // Peekable
    #[test]
    fn test_peekable() {
        use std::iter::Peekable;
        fn parse_number<I>(tokens: &mut Peekable<I>) -> u32 where I: Iterator<Item=char>
        {
            let mut n = 0;
            loop {
                match tokens.peek() {
                    Some(r) if r.is_digit(10) => {
                        n = n * 10 + r.to_digit(10).unwrap();
                    }
                    _ => return n
                }
                tokens.next();
            }
        }
        let mut chars = "226153980,1766319049".chars().peekable();
        assert_eq!(parse_number(&mut chars), 226153980);
// Look, `parse_number` didn't consume the comma! So we will.
        assert_eq!(chars.next(), Some(','));
        assert_eq!(parse_number(&mut chars), 1766319049);
        assert_eq!(chars.next(), None);
    }

    #[test]
    fn test_fuse() {
        struct Flaky(bool);
        impl Iterator for Flaky {
            type Item = &'static str;
            fn next(&mut self) -> Option<Self::Item> {
                if self.0 {
                    self.0 = false;
                    Some("totally the last item")
                } else {
                    self.0 = true; // D'oh!
                    None
                }
            }
        }
        let mut flaky = Flaky(true);
        assert_eq!(flaky.next(), Some("totally the last item"));
        assert_eq!(flaky.next(), None);
        assert_eq!(flaky.next(), Some("totally the last item"));
        let mut not_flaky = Flaky(true).fuse();
        assert_eq!(not_flaky.next(), Some("totally the last item"));
        assert_eq!(not_flaky.next(), None);
        assert_eq!(not_flaky.next(), None);
    }

    #[test]
    fn test_rev() {
        let bee_parts = ["head", "thorax", "abdomen"];
        let mut iter = bee_parts.iter();
        assert_eq!(iter.next(), Some(&"head"));
        assert_eq!(iter.next_back(), Some(&"abdomen"));
        assert_eq!(iter.next(), Some(&"thorax"));
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next(), None);

        // #2
        let meals = ["breakfast", "lunch", "dinner"];
        let mut iter = meals.iter().rev();
        assert_eq!(iter.next(), Some(&"dinner"));
        assert_eq!(iter.next(), Some(&"lunch"));
        assert_eq!(iter.next(), Some(&"breakfast"));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_inspect() {
        let upper_case: String = "große".chars().inspect(|c| println!("before: {:?}", c)).flat_map(|c| c.to_uppercase())
            .inspect(|c| println!(" after: {:?}", c)).collect();
        assert_eq!(upper_case, "GROSSE");
    }

    #[test]
    fn test_chain() {
        // you can chain an iterator together with any iterable that produces the
        // same item type.
        let v: Vec<i32> = (1..4).chain(vec![20, 30, 40]).collect();
        assert_eq!(v, [1, 2, 3, 20, 30, 40]);

        // A chain iterator is reversible, if both of its underlying iterators are:
        let v: Vec<i32> = (1..4).chain(vec![20, 30, 40]).rev().collect();
        assert_eq!(v, [40, 30, 20, 3, 2, 1]);
    }

    #[test]
    fn test_enumerate() {
        let v = vec![40, 30, 20, 3, 2, 1];
        let adapter = v.into_iter().enumerate();
        dump(adapter);
    }

    #[test]
    fn test_zip() {
        // zipping the unbounded-end range 0.. with the other iterator
        // zip 时 以短的那个迭代器结束而结束另一个
        // In this sense, you can think of zip as a generalization of enumerate: whereas enumerate attaches indices to the sequence, zip attaches any arbitrary iterator’s items. We suggested before that enumerate can help provide context for processing items; zip is a more flexible way to do the same.
        let v: Vec<_> = (0..).zip("ABCD".chars()).collect();
        assert_eq!(v, vec![(0, 'A'), (1, 'B'), (2, 'C'), (3, 'D')]);

        // The argument to zip doesn’t need to be an iterator itself; it can be any iterable:
        use std::iter::repeat;
        let endings = vec!["once", "twice", "chicken soup with rice"];
        let rhyme: Vec<_> = repeat("going")
            .zip(endings)
            .collect();
        assert_eq!(rhyme, vec![("going", "once"),
                               ("going", "twice"),
                               ("going", "chicken soup with rice")]);
    }

    #[test]
    fn test_by_ref() {
        /**

        impl<'a, I: Iterator + ?Sized> Iterator for &'a mut I { type Item = I::Item;
        fn next(&mut self) -> Option<I::Item> {
        (**self).next() }
        fn size_hint(&self) -> (usize, Option<usize>) { (**self).size_hint()
        } }

         */
        let message = "To: jimb\r\n From: id\r\n
\r\n
Oooooh, donuts!!\r\n";
        let mut lines = message.lines();
        println!("Headers:");
        // When you call an adapter on a mutable reference to an iterator, the adapter takes ownership of the reference, not the iterator itself. That’s just a borrow that ends when the adapter goes out of scope.
        for header in lines.by_ref().take_while(|l| !l.is_empty()) {
            println!("{}", header);
        }
        println!("\nBody:");
        for body in lines {
            println!("{}", body);
        }
    }

    #[test]
    fn test_cloned() {
        let a = ['1', '2', '3', '∞'];
        assert_eq!(a.iter().next(), Some(&'1'));
        // Since every type that implements Copy also implements Clone, cloned is strictly more general, but depending on the item type, a clone call can do arbitrary amounts of allocation and copying.
        assert_eq!(a.iter().cloned().next(), Some('1'));
    }

    #[test]
    fn test_cycle() {
        //  The underlying iterator must implement std::clone::Clone so that cycle can save its initial state and reuse it each time the cycle starts again.
        let dirs = ["North", "East", "South", "West"];
        let mut spin = dirs.iter().cycle();
        assert_eq!(spin.next(), Some(&"North"));
        assert_eq!(spin.next(), Some(&"East"));
        assert_eq!(spin.next(), Some(&"South"));
        assert_eq!(spin.next(), Some(&"West"));
        assert_eq!(spin.next(), Some(&"North"));
        assert_eq!(spin.next(), Some(&"East"));

        // #2
        use std::iter::{once, repeat};
        let fizzes = repeat("").take(2).chain(once("fizz")).cycle();
        let buzzes = repeat("").take(4).chain(once("buzz")).cycle();
        let fizzes_buzzes = fizzes.zip(buzzes);
        let fizz_buzz = (1..100).zip(fizzes_buzzes).map(|tuple|
            match tuple {
                (i, ("", "")) => i.to_string(),
                (_, (fizz, buzz)) => format!("{}{}", fizz, buzz)
            });
        for line in fizz_buzz {
            println!("{}", line);
        }
    }
}

mod consuming {
    use std::collections::HashMap;
    use std::io::prelude::*;

    fn main() {
        let stdin = std::io::stdin();
        println!("{}", stdin.lock().lines().count());
    }

    #[test]
    fn it_works() {
        fn triangle(n: u64) -> u64 {
            (1..=n).sum()
        }
        assert_eq!(triangle(20), 210);
        fn factorial(n: u64) -> u64 {
            (1..=n).product()
        }
        assert_eq!(factorial(20), 2432902008176640000);

        // #2 max,min
        assert_eq!([-2, 0, 1, 0, -2, -5].iter().max(), Some(&1));
        assert_eq!([-2, 0, 1, 0, -2, -5].iter().min(), Some(&-5));
        //#3
        use std::cmp::Ordering;
        // Compare two f64 values. Panic if given a NaN.
        fn cmp(lhs: &f64, rhs: &f64) -> Ordering {
            lhs.partial_cmp(rhs).unwrap()
        }
        let numbers = [1.0, 4.0, 2.0];
        assert_eq!(numbers.iter().copied().max_by(cmp), Some(4.0));
        assert_eq!(numbers.iter().copied().min_by(cmp), Some(1.0));
        let numbers = [1.0, 4.0, std::f64::NAN, 2.0];
        // assert_eq!(numbers.iter().copied().max_by(cmp), Some(4.0)); // panics

        // #4
        use std::collections::HashMap;
        let mut populations = HashMap::new();
        populations.insert("Portland", 583_776);
        populations.insert("Fossil", 449);
        populations.insert("Greenhorn", 2);
        populations.insert("Boring", 7_762);
        populations.insert("The Dalles", 15_340);
        assert_eq!(populations.iter().max_by_key(|&(_name, pop)| pop), Some((&"Portland", &583_776)));
        assert_eq!(populations.iter().min_by_key(|&(_name, pop)| pop), Some((&"Greenhorn", &2)));
    }

    #[test]
    fn test_compare() {
        let packed = "Helen of Troy";
        let spaced = "Helen of Troy";
        let obscure = "Helen of Sandusky"; // nice person, just not famous
        // assert!(packed != spaced);
        assert!(packed.split_whitespace().eq(spaced.split_whitespace()));
// This is true because ' ' < 'o'.
        assert!(spaced < obscure);
// This is true because 'Troy' > 'Sandusky'.
        assert!(spaced.split_whitespace().gt(obscure.split_whitespace()));
    }

    #[test]
    fn test_any_all()
    {
        let id = "Iterator";
        assert!(id.chars().any(char::is_uppercase));
        assert!(!id.chars().all(char::is_uppercase));
    }

    #[test]
    fn test_pos() {
        let text = "Xerxes";
        // The position method applies a closure to each item from the iterator and returns the index of the first item for which the closure returns true.
        assert_eq!(text.chars().position(|c| c == 'e'), Some(1));
        assert_eq!(text.chars().position(|c| c == 'z'), None);

        let bytes = b"Xerxes";
        assert_eq!(bytes.iter().rposition(|&c| c == b'e'), Some(4));
        assert_eq!(bytes.iter().rposition(|&c| c == b'X'), Some(0));
    }

    #[test]
    fn test_fold() {
        let a = [5, 6, 7, 8, 9, 10];
        assert_eq!(a.iter().fold(0, |n, _| n + 1), 6);
        assert_eq!(a.iter().fold(0, |n, i| n + i), 45);
        assert_eq!(a.iter().fold(1, |n, i| n * i), 151200);
// max
// count
// sum
// product
        assert_eq!(a.iter().cloned().fold(i32::min_value(), std::cmp::max), 10);

        let a = ["Pack", "my", "box", "with", "five", "dozen", "liquor", "jugs"];
// See also: the `join` method on slices, which won't // give you that extra space at the end.
        let pangram = a.iter()
            .fold(String::new(), |s, w| s + w + " ");
        assert_eq!(pangram, "Pack my box with five dozen liquor jugs ");

        // NOTE⚠️：r系列方法 requires a double-ended iterator,
        let weird_pangram = a.iter().rfold(String::new(), |s, w| s + w + " ");
        assert_eq!(weird_pangram, "jugs liquor dozen five with box my Pack ");
    }

    #[test]
    fn test_nth_back() {
        let mut squares = (0..10).map(|i| i * i);
        assert_eq!(squares.nth(4), Some(16));
        assert_eq!(squares.nth(0), Some(25));
        assert_eq!(squares.nth(6), None);
    }

    fn test_last() {
        let squares = (0..10).map(|i| i * i);
        assert_eq!(squares.last(), Some(81));
    }

    #[test]
    fn test_find_xxx() {
        let mut populations = HashMap::new();
        populations.insert("Portland", 583_776);
        populations.insert("Fossil", 449);
        populations.insert("Greenhorn", 2);
        populations.insert("Boring", 7_762);
        populations.insert("The Dalles", 15_340);

        assert_eq!(populations.iter().find(|&(_name, &pop)| pop > 1_000_000), None);
        assert_eq!(populations.iter().find(|&(_name, &pop)| pop > 500_000), Some((&"Portland", &583_776)));
    }
}

fn try_fold() {
    use std::error::Error;
    use std::io::prelude::*;
    use std::str::FromStr;
    fn main() -> Result<(), Box<dyn Error>> {
        let stdin = std::io::stdin();
        let sum = stdin.lock()
            .lines()
            .try_fold(0, |sum, line| -> Result<u64, Box<dyn Error>> {
                Ok(sum + u64::from_str(&line?.trim())?)
            })?;
        println!("{}", sum);
        Ok(())
    }
}

mod build_collections {
    #[test]
    fn it_works() {
        use std::collections::{HashSet, BTreeSet, LinkedList, HashMap, BTreeMap};
        let args: HashSet<String> = std::env::args().collect();
        let args: BTreeSet<String> = std::env::args().collect();
        let args: LinkedList<String> = std::env::args().collect();
// Collecting a map requires (key, value) pairs, so for this example, // zip the sequence of strings with a sequence of integers.
        let args: HashMap<String, usize> = std::env::args().zip(0..).collect();
        let args: BTreeMap<String, usize> = std::env::args().zip(0..).collect();
// and so on
    }

    // If a type implements the std::iter::Extend trait, then its extend method adds an iterable’s items to the collection:
    #[test]
    fn test_extend() {
        let mut v: Vec<i32> = (0..5).map(|i| 1 << i).collect();
        // All of the standard collections implement Extend, so they all have this method; so does String. Arrays and slices, which have a fixed length, do not.
        v.extend(&[31, 57, 99, 163]);
        assert_eq!(v, &[1, 2, 4, 8, 16, 31, 57, 99, 163]);
    }

    #[test]
    fn test_partition() {
        let things = ["doorknob", "mushroom", "noodle", "giraffe", "grapefruit"]; // Amazing fact: the name of a living thing always starts with an
        // odd-numbered letter.
        let (living, nonliving): (Vec<&str>, Vec<&str>)
            = things.iter().partition(|name| name.as_bytes()[0] & 1 != 0);
        assert_eq!(living, vec!["mushroom", "giraffe", "grapefruit"]);
        assert_eq!(nonliving, vec!["doorknob", "noodle"]);
    }

    #[test]
    fn test_for_each() {
        ["doves", "hens", "birds"].iter().zip(["turtle", "french", "calling"].iter()).zip(2..5)
            .rev()
            .map(|((item, kind), quantity)| {
                format!("{} {} {}", quantity, kind, item)
            })
            .for_each(|gift| {
                println!("You have received: {}", gift);
            });


        for gift in ["doves", "hens", "birds"].iter().zip(["turtle", "french", "calling"].iter()).zip(2..5)
            .rev()
            .map(|((item, kind), quantity)| {
                format!("{} {} {}", quantity, kind, item)
            }) {
            println!("You have received: {}", gift);
        }
    }
}