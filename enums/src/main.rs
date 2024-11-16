use std::f64::consts::E;

fn main() {
    #[derive(Debug)]
    enum SquirrelColor {
        Red(i32),
        Gray(i32),
        Black,
        Mixed(String),
        None,
    }

    struct Squirrel {
        legs: i32,
        color: SquirrelColor,
        name: String,
    }

    let red_color = SquirrelColor::Red;
    let gray_color = SquirrelColor::Gray;
    impl SquirrelColor {
        fn print_squirrel_color(&self) {
            match self {
                SquirrelColor::Red(..) => println!("Red with value {self:?}"),
                SquirrelColor::Gray(value) => println!("Gray with value {value}"),
                Self::Black => println!("BLACK!"),
                SquirrelColor::Mixed(_) => println!("{self:?}"),
                other => println!("Other！"),
            }
        }
    }

    let squirrel_king = Squirrel {
        legs: 4,
        color: SquirrelColor::Red(10),
        name: String::from("Guagua"),
    };

    let color_red = SquirrelColor::Red(12);
    color_red.print_squirrel_color();
    let color_mixed = SquirrelColor::Mixed(String::from("black and white"));
    color_mixed.print_squirrel_color();
    let color_gray = SquirrelColor::Gray(7);
    color_gray.print_squirrel_color();

    // Option

    let some_squirrel = Some(squirrel_king);
    let some_legs = Some(4);
    let some_number = Some(100);
    let no_legs: Option<i32> = None;

    fn add_leg(legs: Option<i32>) -> Option<i32> {
        match legs {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
    let one_more_leg = add_leg(some_legs);
    println!("{one_more_leg:?}");

    // if let
    match some_legs {
        Some(i) => println!("we have {i} legs"),
        _ => println!("ELSE"),
    }
    if let Some(i) = some_legs {
        println!("we have {i} legs, simple way");
    } else {
        println!("ELSE");
    }
}
