fn main() {
    let prairie_dog = true;
    let squirrel_king = if prairie_dog { 7 } else { 8 };

    if squirrel_king > 7 {
        println!("what I'm saying?")
    } else if squirrel_king > 5 {
        println!("Good good!");
    } else {
        println!("WTF!?");
    }
    let mut squirrel_num = 0;
    let mut dog_num = 0;

    'counting_animals: loop {
        squirrel_num += 1;
        dog_num += 1;
        if squirrel_num == 100 {
            break;
        };
        loop {
            if dog_num == 7 {
                break 'counting_animals;
            }
            if dog_num < 51 {
                println!("woof woof");
                break;
            }
        }
    }

    println!("big dog {squirrel_num}");

    let a_collection = ["📕", "🐕", "🌲", "😵", "🐟", "☁"];
    let mut iterator_index = 0;
    while iterator_index < a_collection.len() {
        println!("{}", a_collection[iterator_index]);
        iterator_index += 1;
    }

    for emoji in a_collection {
        println!("{emoji}");
    }

    for index in (1..4).rev() {
        println!("{}", a_collection[index]);
    }

}


