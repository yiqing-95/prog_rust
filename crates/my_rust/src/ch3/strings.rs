use regex::Regex;

fn literals() {
    let speech = "\"Ouch!\" said the well.\n";

    println!("In the room the women come and go,
        Singing of Mount Abora");

    println!("It was a bright, cold day in April, and \
     there were four of us—\
more or less.");

    let default_win_install_path = r"C:\Program Files\Gorillas";
    let pattern = Regex::new(r"\d+(\.\d+)*");
    println!(r###"
This raw string started with 'r###"'.
Therefore it does not end until we reach a quote mark ('"') followed immediately by three pound signs ('###'):
"###);
}

#[test]
fn byte_string() {
    let method = b"GET";
    assert_eq!(method, &[b'G', b'E', b'T']);
}

#[test]
fn strings() {
    let noodles = "noodles".to_string();
    let oodles = &noodles[1..];
    let poodles = "ಠ_ಠ";

    assert_eq!("ಠ_ಠ".len(), 7);
    assert_eq!("ಠ_ಠ".chars().count(), 3);

    // For creating new strings at run time, use String.

    // ## string 创建
    let _error_message = "too many pets".to_string();

    assert_eq!(format!("{}°{:02}′{:02}′′N", 24, 5, 23), "24°05′23′′N".to_string());

    let bits = vec!["veni", "vidi", "vici"];
    assert_eq!(bits.concat(), "venividivici");
    assert_eq!(bits.join(", "), "veni, vidi, vici");

    // &str 参数类型兼容性更好 可以传递String 或者字符串字面量
    fn some_string_fn(s: &str) {}

    // ## 比较操作 ，内容一样就相等 ：相同的字符序列 相同的顺序 不论地址
    assert!("ONE".to_lowercase() == "one");

    assert!("peanut".contains("nut"));
    assert_eq!("ಠ_ಠ".replace("ಠ", "■"), "■_■");
    assert_eq!(" clean\n".trim(), "clean");
    for word in "veni, vidi, vici".split(", ") {
        assert!(word.starts_with("v"));
    }
}