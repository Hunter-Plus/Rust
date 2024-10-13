fn main() {
    println!("Hello, world!");

    function1(5, "squirrels");

    let y = {
        let z = 211;
        z - 100
    };

    println!("y is from expression {y}");

    let n = return_n_plus_one(76);

    println!("n+1 is {n}");
}

fn function1( number : i32, animal: &str) {
    println!("where can we find {number} {animal}?");
}

fn return_n_plus_one(n:i32)-> i32{
    n+1;
    12
}

