fn main() {
    let mut x = 7;
    println!("We have {x} squirrels");
    x = 6;
    println!("Squirrel eat {x} acorns");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    {
        let x = 101;
        println!("Inner shadowing in cruly brackets {x}")
    }

    let x = 5;
    println!("Shadowing suqirel is {x}");

    let cat = '😻';
    println!("what is the cat? {cat}");

    let chip = '🐿';
    println!("Show me some rodent! {chip}");

    let testing_tuble: (char, u64, f32) = ('🐿', 10086, 32.196);

    let (i,j,k) = testing_tuble;

    println!("The rat is in {i}");
    let unit_tuple : () = ();

    let array1 = [1,2,3,4,5];
    let array2: [i64; 5];
    let array3 = [320; 5];

    let out_of_index = array1[6];

    println!("Lets try a non-exist index {out_of_index}");

}
