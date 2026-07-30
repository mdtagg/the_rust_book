use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    //ch_1();
    //ch_2();
    ch_3();
}

fn ch_1() {
    println!("hello world");
}
fn ch_2() {
    fn guessing_game() {
        println!("Guess a number!");
        let secret_number = rand::thread_rng().gen_range(1..=100);
        println!("Your secret number is {secret_number}");
        loop {
            let mut guess = String::new();
            io::stdin().read_line(&mut guess).expect("Cannot read line");
            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too Big!"),
                Ordering::Equal => {
                    println!("You win!");
                    break;
                }
            }
        }
    }
    guessing_game()
}
fn ch_3() {
    fn variables() {
        //let mut x = 5;
        //println!("The value of x is: {x}");
        //x = 6;
        //println!("The value of x is: {x}");

        // constants MUST have a type annotation
        // They are valid for the entire time a program runs in their scope
        //const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

        //let allows us to "shadow" => change variable without changing name
        //In the {} the scope is changed so the line will read 6 then 12
        let x = 5;
        let x = x + 1;
        {
            let x = x * 2;
            println!("The value of x is: {x}")
        }
        println!("The value of x is: {x}");

        //here guess must have a type annotation
        //rust is statically typed (compiler must know types at compile time)
        let _guess: u32 = "42".parse().expect("Not a number");

        //let tup: (i32, f64, u8) = (500, 6.4, 1);
        //let (x, y, z) = tup;
        //println!("The value of y is: {y}");

        let x: (i32, f64, u8) = (500, 6.4, 1);

        let _five_hundred = x.0;
        let _six_point_four = x.1;
        let _one = x.2;

        //arrays in rust have  fixed length, vectors dont
        //array data must have the same type
        let _a: [i32; 5] = [1, 2, 3, 4, 5];

        //this array will be [3,3,3,3,3]
        let _a_imp = [3; 5];

        //if a is indexed out of bounds ie a[6] rust will not compile
        //lower level languages will not catch this error and use unsafe memory
    }
    variables();
    fn loops() {
        regular_for_loop();
        nested_loops();
        while_loops();
        for_loop();
        for_range();

        fn regular_for_loop() {
            let mut counter = 0;
            let result = loop {
                counter += 1;
                if counter == 10 {
                    break counter * 2;
                }
            };
            println!("The result is: {result}");
        }
        fn nested_loops() {
            let mut count = 0;
            'counting_up: loop {
                println!("count = {count}");
                let mut remaining = 10;

                loop {
                    println!("remaining = {remaining}");
                    if remaining == 9 {
                        break;
                    }
                    if count == 2 {
                        break 'counting_up;
                    }
                    remaining -= 1;
                }
                count += 1;
            }
            println!("End count = {count}");
        }
        fn while_loops() {
            let mut count = 0;

            while count != 3 {
                count += 1;
            }
        }
        fn for_loop() {
            let a = [10, 20, 30, 40, 50];

            for element in a {
                println!("the value is: {element}");
            }
        }
        //rust users almost always default to a for loop over a while
        //while loops require an index which can be unsafe
        fn for_range() {
            for number in 1..4 {
                println!("{number}");
            }
        }
    }
    loops();
    fn branches() {
        one();
        two();
        three();

        fn one() {
            let number = 3;

            if number < 5 {
                println!("True");
            } else {
                println!("False");
            }
        }

        fn two() {
            let number = 3;
            //rust does not coerce values in if statments to be boolean
            //if number {
            //    println!("number is equal to 3");
            //}
            if number != 0 {
                println!("number is equal to 3");
            }
        }

        //the types in this kind of conditional must match
        fn three() {
            let condition = true;
            let number = if condition { 5 } else { 6 };
            println!("The value of number is: {number}");
        }
    }
    branches();
}
