fn longest_str<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// generic version
use std::fmt::Display;
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// lifetimes in struct
struct Animal<'a> {
    tail: &'a str,
}
// lifetimes in methods
impl<'a> Animal<'a> {
    fn barking(&self, tone: &str) -> &str {
        println!("Attention please: {tone}");
        self.tail
    }
}

fn main() {
    let string1 = String::from("prairie_dog");
    {
        let string2 = String::from("squirrel");

        let longer_animal = longest_str(&string1, &string2);
        println!("The longer one is {longer_animal}");
    }

    // cannot complie
    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is {result}");

    let s: &'static str = "Eternal squirrel king!";
}
