fn main() {
    let mut s = String::from("Squirrel!");
    s.push_str("Stupid!");

    // only copy the pointer, the length and the capacity.
    // Making s invalid.
    let mut s1 = s;

    // Deep copy
    let s2 = s1.clone();
    println!("{s1}");

    // the memory allocted for s (now s1) and s2 will be freed when they will be out of scope
    // the call for drop function finish this job

    fn takeing_ownership(s: String) {
        println!("{s}");
    }
    takeing_ownership(s2);
    // s3 will be dropped after this calling

    let i = 3;
    making_copy(i);
    // i32 has a copy implementation so it will make a copy
    fn making_copy(i: i32) {
        println!("{i}");
    }

    let new_s = return_a_ownership();
    let new_s1 = String::from("squirrel");
    let new_s2 = taking_and_returning_ownership(new_s1);

    println!("{new_s}");
    fn return_a_ownership() -> String {
        let new_string = String::from("Rodent");

        new_string
    }

    fn taking_and_returning_ownership(s: String) -> String {
        s
    }

    fn not_take_ownership(s: &String) -> usize {
        s.len()
    } // s will not being dropped here because it's a reference without ownership.

    fn mutable_reference(s: &mut String) {
        s.push_str(" Super Super Stupid!")
    }

    let mut ss = String::from("Big Suqirrel!");

    mutable_reference(&mut ss);

    println!("{ss}");

    //Slices
    // fn return_first_word(s: &String) -> &str {
    fn return_first_word(s: &str) -> &str {
        // Convert String to an array of bytes
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        return &s[..];
    }

    let sss = String::from("Squirrel King");
    let squirrel = &sss[0..8];
    let king = &sss[9..13];

    let who = return_first_word(&sss);
    println!("{who}");
}
