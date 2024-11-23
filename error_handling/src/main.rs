use std::{error::Error, fs::File, io::ErrorKind};

fn main() -> Result<(), Box<dyn Error>> {
    // let vec1 = vec![1,2];
    // println!("{}", vec1[15]);
    // panic!("how to use it?");

    let data_file = File::open("src/data.txt");

    let opened_file = match data_file {
        Ok(content) => content,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            other_error => {
                panic!("Problem opening the file: {other_error:?}");
            }
        },
    };

    println!("{opened_file:?}");

    // let data_file1 = File::open("data.txt").unwrap();
    let data_file1 = File::open("data.txt").expect("let's see what can we get here");

    // Using types to conduct validation
    pub struct Guess {
        value: i32,
    }

    impl Guess {
        pub fn new(v: i32) -> Guess {
            if v < 1 || v > 100 {
                panic!("Guess value must be 1-100, got {v}");
            }
            Guess { value: v }
        }
        pub fn value(&self) -> i32 {
            self.value
        }
    }

    Ok(())
}
