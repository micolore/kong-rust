use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    test_fn(32);
}

fn test_fn(x: u32) -> u32 {
    println!("x is {}", x);
    x + 1
}

fn _test_var() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    const DAY_HOUR: u32 = 24;

    println!("The day of x is: {}", DAY_HOUR);

    let _y = 1;
    let _y = "123";
    let _y = true;

    println!("The value of y is: {}", _y);

    let mut _z = 1;

    // _z = "123"; //  ^^^^^ expected integer, found `&str`


    println!("The value of z is: {}", _z);
}

fn _guess_num_game() {
    println!("guess  the number!");

    loop {
        println!("Pls input you guess!");

        let rand_number = rand::thread_rng().gen_range(1..101);

        // println!("rand number is: {}", rand_number);

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("failed to read line!");

        let guess: u32 = match guess.trim().parse()
        {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("you guessed: {}", guess);

        match guess.cmp(&rand_number) {
            Ordering::Less => println!("too small!"),
            Ordering::Greater => println!("too big!"),
            Ordering::Equal => {
                println!("wow you win!");
                break;
            }
        }
    }
}

