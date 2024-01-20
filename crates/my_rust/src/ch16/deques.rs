use std::collections::VecDeque;

#[test]
fn test_ops() {
    let mut my_deck = VecDeque::new();

    my_deck.push_front(1);

    my_deck.push_back(100);

    println!("{my_deck:?}");

    assert_eq!(my_deck.len(), 2);

    let front = my_deck.pop_front();
    println!("front : {}", front.unwrap());

    let back = my_deck.pop_back();
    println!("back : {}", back.unwrap());

    my_deck.push_front(200);
    my_deck.push_front(300);
    my_deck.push_front(400);

    let f = my_deck.front();
    let b = my_deck.back();
    println!("front : {} , back: {}", f.unwrap(), b.unwrap());

    // 同理 deque.back_mut() ...
    let mut f = my_deck.front_mut().unwrap();
    *f = 2000;
    println!("front : {}", my_deck.front().unwrap());

    //
    let my_vec = Vec::from(my_deck);
    println!("{:?}", my_vec);
    // 下面这个操作会比较快 因为底层结构直接可复用
    let my_deque = VecDeque::from(my_vec
    );
    println!("{:?}", my_deque);

    //
    let v = VecDeque::from(vec![1, 2, 3, 4]);
}