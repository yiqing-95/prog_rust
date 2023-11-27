#[test]
fn test_array() {
    let a: [usize; 10] = [1; 10];
    assert_eq!(a.len(), 10);

    // 声明形式
    let lazy_caterer: [u32; 6] = [1, 2, 4, 7, 11, 16];
    let taxonomy = ["Animalia", "Arthropoda", "Insecta"];
    assert_eq!(lazy_caterer[3], 7);
    assert_eq!(taxonomy.len(), 3);

    // 预填充数组

    let mut sieve = [true; 10000];
    for i in 2..100 {
        if sieve[i] {
            let mut j = i * i;
            while j < 10000 {
                sieve[j] = false;
                j += i;
            }
        }
    }
    assert!(sieve[211]);
    assert!(!sieve[9876]);

    // 隐式转化
    // 数组的大部分方法 都是通过slice实现的 通过方法签名可以看到 &arr &mut arr 就是slice
    // The useful methods you’d like to see on arrays—iterating over elements, searching, sorting, filling, filtering, and so on—are all provided as methods on slices,

    // implicitly converts a reference to an array to a slice when searching for methods, so you can call any slice method on an array directly:
    let mut chaos = [3, 5, 4, 1, 2];
    chaos.sort();
    assert_eq!(chaos, [1, 2, 3, 4, 5]);
}

#[test]
fn test_vector() {
    let mut v: Vec<usize> = vec![1];
    let mut v2: Vec<usize> = vec![20, 30];

    v.push(2);
    v2.remove(1);

    v.append(&mut v2);

    assert_eq!(v.len(), 3);

    // ### 创建
    let mut primes = vec![2, 3, 5, 7];
    assert_eq!(primes.iter().product::<i32>(), 210);
    // OPS
    primes.push(11);
    primes.push(13);
    assert_eq!(primes.iter().product::<i32>(), 30030);

    let mut pal = Vec::new();
    pal.push("step");
    pal.push("on");
    pal.push("no");
    pal.push("pets");
    assert_eq!(pal, vec!["step", "on", "no", "pets"]);
    // from iter
    // Many library functions look for the opportunity to use Vec::with_capacity instead of Vec::new.
    let v: Vec<i32> = (0..5).collect();
    assert_eq!(v, [0, 1, 2, 3, 4]);

    // 同array类似 在向量上使用slice的方法
    // A palindrome!
    let mut palindrome = vec!["a man", "a plan", "a canal", "panama"];
    palindrome.reverse();
// Reasonable yet disappointing:
    assert_eq!(palindrome, vec!["panama", "a canal", "a plan", "a man"]);

    // 预分配容量
    let mut v = Vec::with_capacity(2);
    assert_eq!(v.len(), 0);
    assert_eq!(v.capacity(), 2);
    v.push(1);
    v.push(2);
    assert_eq!(v.len(), 2);
    assert_eq!(v.capacity(), 2);
    v.push(3);
    assert_eq!(v.len(), 3);
// Typically prints "capacity is now 4": println!("capacity is now {}", v.capacity());

    // 增删 导致后续元素挪动位置
    let mut v = vec![10, 20, 30, 40, 50];
// Make the element at index 3 be 35.
    v.insert(3, 35);
    assert_eq!(v, [10, 20, 30, 35, 40, 50]);
// Remove the element at index 1.
    v.remove(1);
    assert_eq!(v, [10, 30, 35, 40, 50]);

    // pop方法

    let mut v = vec!["Snow Puff", "Glass Gem"];
    assert_eq!(v.pop(), Some("Glass Gem"));
    assert_eq!(v.pop(), Some("Snow Puff"));
    assert_eq!(v.pop(), None);

    // 迭代
    // Get our command-line arguments as a vector of Strings.
    let languages: Vec<String> = std::env::args().skip(1).collect();
    for l in languages {
        println!("{}: {}", l, if l.len() % 2 == 0 {
            "functional"
        } else {
            "imperative"
        });
    }
}

fn new_pixel_buffer(rows: usize, cols: usize) -> Vec<u8> {
    vec![0; rows * cols]
}

#[test]
fn test_slice() {
    let s = &[1, 2, 3];

    let mut s2 = &mut [1, 2, 3];

    assert_eq!(s.len(), 3);
    assert_eq!(s2.len(), 3);

    assert_eq!(3, s[s.len() - 1]);// last element


    let v: Vec<f64> = vec![0.0, 0.707, 1.0, 0.707];
    let a: [f64; 4] = [0.0, -0.707, -1.0, -0.707];
    let sv: &[f64] = &v;
    let sa: &[f64] = &a;


    fn print(n: &[f64]) {
        for elt in n {
            println!("{}", elt);
        }
    }
    print(&a); // works on arrays
    print(&v); // works on vectors

// print the first two elements of v
// print elements of a starting with a[2]
// print v[1] and v[2]
    print(&v[0..2]);
    print(&a[2..]);
    print(&sv[1..3]);
}