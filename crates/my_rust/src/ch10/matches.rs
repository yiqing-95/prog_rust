#[test]
fn test_switch_int() {
    let count_rabbits = 3;
    // match meadow.count_rabbits() {
    match count_rabbits {
        0 => {} // nothing to say
        1 => println!("A rabbit is nosing around in the clover."),
        n => println!("There are {} rabbits hopping about in the meadow", n),
    }

    // Other literals can be used as patterns too, including Booleans, characters, and even strings:
    // let calendar = match settings.get_string("calendar") { "gregorian" => Calendar::Gregorian,
    //     "chinese" => Calendar::Chinese,
    //     "ethiopian" => Calendar::Ethiopian,
    //     other => return parse_error("calendar", other), };
}

#[test]
fn test_catchall() {
    // underscore _ as a pattern, the wildcard pattern:

    // let caption = match photo.tagged_pet() {
    //     Pet::Tyrannosaur => "RRRAAAAAHHHHHH",
    //     Pet::Samoyed => "*dog thoughts*",
    //     _ => "I'm cute, love me", // generic caption, works for any pet
    // };
}

#[test]
fn match_tuples() {
    fn describe_point(x: i32, y: i32) -> &'static str {
        use std::cmp::Ordering::*;
        match (x.cmp(&0), y.cmp(&0)) {
            (Equal, Equal) => "at the origin",
            (_, Equal) => "on the x axis",
            (Equal, _) => "on the y axis",
            (Greater, Greater) => "in the first quadrant",
            (Less, Greater) => "in the second quadrant",
            _ => "somewhere else",
        }
    }
}

fn match_struct() {
    // match balloon.location {
    //     Point { x: 0, y: height } =>
    //         println!("straight up {} meters", height),
    //     Point { x: x, y: y } =>
    //         println!("at ({}m, {}m)", x, y),
    // }

    // match a large struct
    // Some(Account { name, language, .. }) => language.show_custom_greeting(name),
}

fn match_array() {
    fn hsl_to_rgb(hsl: [u8; 3]) -> [u8; 3] {
        match hsl {
            [_, _, 0] => [0, 0, 0],
            [_, _, 255] => [255, 255, 255],
            // ...
            [_, _, _] => [255, 255, 255],
        }
    }
}

fn match_slice() {
    fn greet_people(names: &[&str]) {
        match names {
            [] => { println!("Hello, nobody.") }
            [a] => { println!("Hello, {}.", a) }
            [a, b] => { println!("Hello, {} and {}.", a, b) }
            [a, .., b] => { println!("Hello, everyone from {} to {}.", a, b) }
        }
    }
}

// ref patterns borrow parts of a matched value. & patterns match references.
fn ref_pattern() {
    // match account {
    //     Account { ref name, ref language, .. } => {
    //         ui.greet(name, language);
    //         ui.show_settings(&account); // ok }
    //     }

    //
    // match line_result {
    //     Err(ref err) => log_error(err), // `err` is &Error (shared ref)
    // Ok(ref mut line) => { // `line` is &mut String (mut ref)
    //     trim_comments(line); // modify the String in place
    //     handle(line);
    // }
}

fn match_guards(){
    // match point_to_hex(click) {
    //     None => Err("That's not a game space."), Some(hex) if hex == current_hex =>
    //         Err("You are already there! You must click somewhere else"),
    //     Some(hex) => Ok(hex)
    // }
}

fn  multiple_possibilities(){

    // let at_end = match chars.peek() { Some(&'\r') | Some(&'\n') | None => true, _ => false,
    // };

    // match next_char {
    //     '0'..='9' => self.read_number(),
    //     'a'..='z' | 'A'..='Z' => self.read_word(), ' ' | '\t' | '\n' => self.skip_whitespace(),
    // _ => self.handle_punctuation(),
    // }
    // Rust does not (yet) permit the use of end-exclusive ranges like 0..100 in patterns.
}