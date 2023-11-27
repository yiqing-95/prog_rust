fn print_padovan() {
    let mut padovan = vec![1, 1, 1]; // allocated here


    for i in 3..10
    {
        let next = padovan[i - 3] + padovan[i - 2];
        padovan.push(next);
    }
    println!("P(1..10) = {:?}", padovan);
} // // dropped here

fn example1() {
    let point = Box::new((0.625, 0.5)); // point allocated here
    let label = format!("{:?}", point); // label allocated here
    assert_eq!(label, "(0.625, 0.5)");
}// both dropped here

fn example2() {
    struct Person {
        name: String,
        birth: i32,
    }
    let mut composers = Vec::new();
    composers.push(Person {
        name: "Palestrina".to_string(),
        birth: 1525,
    });
    composers.push(Person {
        name: "Dowland".to_string(),
        birth: 1563,
    });
    composers.push(Person {
        name: "Lully".to_string(),
        birth: 1632,
    });
    for composer in &composers {
        println!("{}, born {}", composer.name, composer.birth);
    }
}

fn deep_copy() {
    let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
    let t = s.clone();
    let u = s.clone();
}

fn example3() {
    let mut s = "Govinda".to_string();
    let t = s;
    s = "Siddhartha".to_string(); // nothing is dropped here

    println!("{} , {}", t, s);
}

#[test]
fn test_move_from_vector() {
    // Build a vector of the strings "101", "102", ... "105"
    let mut v = Vec::new();
    for i in 101..106
    {
        v.push(i.to_string());
    }
// 1. Pop a value off the end of the vector:
    let fifth = v.pop().expect("vector empty!");
    assert_eq!(fifth, "105");
// 2. Move a value out of a given index in the vector, // and move the last element into its spot:
    let second = v.swap_remove(1);
    assert_eq!(second, "102");

    // 3. Swap in another value for the one we're taking out:
    let third = std::mem::replace(&mut v[2], "substitute".to_string());
    assert_eq!(third, "103");
// Let's see what's left of our vector.
    assert_eq!(v, vec!["101", "104", "substitute"]);
}

#[test]
fn consume_elements() {
    let v = vec!["liberté".to_string(), "égalité".to_string(),
                 "fraternité".to_string()];
    // NOTE: 这块不好理解 🤔v 所有权转交给for for内部分解其成为元素的集合
    for mut s in v {
        s.push('!');
        println!("{}", s);
    }
}

#[test]
fn another_move_from_vector() {
    struct Person {
        name: Option<String>,
        birth: i32,
    }
    let mut composers = Vec::new();
    composers.push(Person {
        name: Some("Palestrina".to_string()),
        birth: 1525,
    });
    // You can’t do this:
    // let first_name = composers[0].name;
    // 但可以这样
    // let first_name = composers[0].name.take();

    // 等价下面👇这样
    let first_name = std::mem::replace(&mut composers[0].name, None);
    assert_eq!(first_name, Some("Palestrina".to_string()));
    assert_eq!(composers[0].name, None);
}

#[test]
fn copy_types() {
    let string1 = "somnambulance".to_string();
    // string1 所有权转移 此后便处于·未初始化·状态 不可以🙅再被访问
    let string2 = string1;
    let _ = string2;

    // copy 类型是例外 赋值会出发copy行为 而且源不会处于未初始状态 ！仍旧可访问
    let num1: i32 = 36;
    let num2 = num1;
    println!("{num1},{num2}");
}

#[test]
fn udt_copy() {
    // 派生宏Copy需要其成员都是可copy的
    #[derive(Copy, Clone)]
    struct Label {
        number: u32,
        // name: String ,//  这个就不行了
    }
    fn print(l: Label) { println!("STAMP: {}", l.number); }
    let l = Label { number: 3 };
    print(l);
    println!("My label number is: {}", l.number);
}