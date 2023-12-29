struct Appellation {
    name: String,
    nicknames: Vec<String>,
}

impl Drop for Appellation {
    // Rust calls Drop::drop on a value before dropping its fields or elements, the value the method receives is always still fully initialized./
    fn drop(&mut self) {
        print!("Dropping {}", self.name);
        if !self.nicknames.is_empty() {
            print!(" (AKA {})", self.nicknames.join(", "));
        }
        println!("");
    }
}

#[test]
fn test_drop() {
    {
        let mut a = Appellation {
            name: "Zeus".to_string(),
            nicknames: vec!["cloud collector".to_string(),
                            "king of the gods".to_string()],
        };
        println!("before assignment");
        a = Appellation { name: "Hera".to_string(), nicknames: vec![] };
        println!("at end of block");
    }
    println!("done!");
}