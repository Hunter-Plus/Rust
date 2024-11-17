fn main() {
    //Vector
    // vec init
    let mut vec1: Vec<i32> = Vec::new();
    let vec2 = vec![1, 2, 3];
    // vec update
    vec1.push(1);

    // vec visit
    let vec_3: &i32 = &vec2[2];
    let vec_3: Option<&i32> = vec2.get(2);

    // iterating (protected by borrowing checks)
    let mut vec3 = vec![4, 5, 6];
    for i in &vec3 {
        println!("{i}");
    }
    for i in &mut vec3 {
        *i -= 1;
    }

    // Vector + Enums
    enum RodentInfo {
        Legs(i32),
        Name(String),
        Height(f64),
    }
    let vec4 = vec![
        RodentInfo::Legs(4),
        RodentInfo::Legs(3),
        RodentInfo::Name(String::from("Guagua")),
        RodentInfo::Height(5.5),
    ];

    //String
    // String creating
    let mut str1 = String::new(); // empty String
    let str2 = "hello, squirrel".to_string(); // any type with Display trait implemented
    let mut str3 = String::from("Hi, prairie dog!");
    // updating: appending
    str3.push_str("You are stupid!");
    str3.push('!');

    // '+' == add(self, s: &str)
    let str4 = String::from("first");
    let str5 = String::from("sign");
    let str45 = str4 + &str5; // str4 will be invalid after this
                              // format!, macro doesn't take ownerships
    let str6 = String::from("9");
    let str7 = String::from("10");
    let str8 = String::from("23");
    let str678 = format!("brand new: {str6}-{str7}-{str8}");
    println!("{str678}!");

    // Slicing
    let str9 = &str678[0..5];
    println!("{str9}!"); // the

    //iterating
    let str10 = String::from("今天ssks");
    for c in str10.chars() {
        println!("{c}");
    }
    for b in str10.bytes() {
        println!("{b}");
    }

    //HashMAp
    // Creating
    use std::collections::HashMap;
    let mut squirrel_counting = HashMap::new();
    squirrel_counting.insert(String::from("Red"), 5);
    squirrel_counting.insert(String::from("Black"), 3);

    let squirrel_type = String::from("Red");
    let number = squirrel_counting
        .get(&squirrel_type)
        .copied()
        .unwrap_or(1000);
    println!("We have {number} Red squirrel counted!");
    for (k, v) in &squirrel_counting {
        println!("{k},{v}.");
    }
    // Updating
    squirrel_counting.insert(String::from("Red"), 100);
    squirrel_counting.entry(String::from("Grey")).or_insert(15);
    squirrel_counting
        .entry(String::from("Black"))
        .or_insert(1000);
    let white_number = squirrel_counting.entry(String::from("White")).or_insert(0);
    *white_number += 1;
    for (k, v) in &squirrel_counting {
        println!("{k},{v}.");
    }
}
