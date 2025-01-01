use std::thread;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Black,
    White,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn most_stocked(&self) -> ShirtColor {
        let mut num_black = 0;
        let mut num_white = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Black => num_black += 1,
                ShirtColor::White => num_white += 1,
            }
        }

        if num_black > num_white {
            ShirtColor::Black
        } else {
            ShirtColor::White
        }
    }

    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Black, ShirtColor::White, ShirtColor::Black],
    };

    let user1 = Some(ShirtColor::White);
    let giveaway1 = store.giveaway(user1);
    println!("The user with preference {:?} gets {:?}", user1, giveaway1);

    let user2 = None;
    let giveaway2 = store.giveaway(user2);
    println!("Thea user with preference {:?} gets {:?}", user2, giveaway2);

    // Syntax & Annotation
    fn add_one_v1(x: u32) -> u32 {
        x + 1
    }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x| x + 1;
    let add_one_v4 = |x| x + 1;

    println!(
        "Closure without annotation needs to be evaluaded from their usage{}",
        add_one_v3(1)
    );
    println!(
        "Closure without annotation needs to be evaluaded from their usage{}",
        add_one_v4(2)
    );
    // Inference
    let return_itself = |x| x;

    let s = return_itself(String::from("squirrel!"));
    // Will cause complier error (using different types) because the inference can only happen once.
    // let n = return_itself(3);

    // Reference & Ownership
    // immutable reference
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");

    // mutable reference
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let mut borrows_mutably = || list.push(7);
    // println!("Try borrow when there is a mutable reference infered: {list:?}");
    borrows_mutably();
    println!("After calling closure: {list:?}");

    // moving ownership
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();

    // println!("Trying borrowing after the onwership was moved: {list:?}");
}
