struct Squirrel {
    legs:i32,
    name: String,
    color: String,
    whisker: bool,
}

// tuple struct
struct Color(i32,i32,i32);

// unit-like struct
struct Stupid;

fn make_a_squirrel (name:String, color:String) -> Squirrel{
    Squirrel {
        whisker: true,
        legs: 4,
        // name: name,
        // color: color,
        // using shorthand
        name,
        color,
    }
}

fn main() {
    let mut rodent1 = Squirrel {
        whisker: true,
        name: String::from("King"),
        legs: 4,
        color: String::from("Red"),
    };

    rodent1.whisker = false;

    let rodent2 = Squirrel{
        color: String::from("Black"),
        name: String::from("Prairie"),
        // whisker: rodent1.whisker,
        // legs: rodent1.legs,
        ..rodent1
    };
    // some fields of rodent 1 will be invalid as the ownerships has been moved

    let black = Color(0,0,0);
    let squirrel_iq = Stupid;
}
