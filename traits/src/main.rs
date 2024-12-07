use traits::{Greeting, ParirieDog, Squirrel, Tweeting, Warning};

// function only for types with Tweeting trait
pub fn calling(animal: &impl Tweeting) -> String {
    format!("The animal feedbacks: {}", animal.tweet())
}
// it's a syntax sugar for:
pub fn calling_verbose<T: Tweeting>(animal: &T) -> String {
    format!("The animal feedbacks: {}", animal.tweet())
}

fn main() {
    let squirrel_king = Squirrel {
        name: String::from("dandan"),
        legs: 4,
        tone: String::from("ji~~~"),
    };
    let prairie_dogo = ParirieDog {
        name: String::from("King"),
        tail: 1,
        tone: String::from("hengheng"),
    };

    println!("We heard the squirrel barking: {}", squirrel_king.tweet());
    println!("Then the prairie dog says: {}", prairie_dogo.hello());
    println!("Squirrel seems to be happy: {}", squirrel_king.hello());
    println!("Prairie dog then yalling: {}", prairie_dogo.tweet());

    println!(
        "Suddenlly a praditor shown up! Prairie dog: {}",
        prairie_dogo.warn()
    );
    println!("Squirrel king then: {}", squirrel_king.loud_tweet());

    println!("We call the squirrel. {}", calling(&squirrel_king));
}
