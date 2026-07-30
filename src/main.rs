use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    //ch_1();
    //ch_2();
    //ch_3();
    ch_4();
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
fn ch_4() {
    str_lit();
    str_rep();
    scope_assignment();
    copies();
    ownership();
    //imut_s is a string literal, the string is hardcoded into the binary output
    //string literals are fast becuase their size is fixed and they can be stored on the stack
    //s a String type, contents heap which increases overhead but allows its size to vary
    //This also introduces memory management issues IE memory needs to be allocated and returned
    //Certain languages use a garbage collecter to deal with this issue, rust uses ownership
    //Ownership is a set of rules that garuntee memory is freed correctly using scope
    fn str_lit() {
        let _imut_s = "hello";
        let mut s = String::from("hello");
        s.push_str(", world!");
        println!("{s}");
    }

    //s1 and s2 are both pointers on the stack
    //they point to data associated with the string
    //the pointer, length and capacity
    //Each letter of hello is stored on the heap and is NOT copied
    fn str_rep() {
        let s1 = String::from("hello");
        let s2 = s1;
        //println!("{s1}")  will error becuase s1 has been freed
        //This is called a move instead of a shallow copy because the first value is freed
    }
    //s with "hello" is overwritten after "ahoy"
    fn scope_assignment() {
        let mut s = String::from("hello");
        s = String::from("ahoy");
        println!("{s}, world!");
    }

    fn copies() {
        //integers size are known at compile time so they are stored on the stack
        //certain type like integers have "copy trait" dont move and instead are copied
        let x = 5;
        let y = x;
        println!("x={x},y={y}");

        let s = String::from("hello");
        //takes_ownership(s);
        let x = 5;
        //makes_copy(x); here x is does not move into the function and can be used after
    }

    fn ownership() {
        let s1 = gives_ownership(); // moves its return value to s1
        let s2 = String::from("hello");
        let s3 = takes_and_gives_back(s2); //s2 is moved into takes_and_gives_back, moves its return
        //value into s3
    }
    fn gives_ownership() -> String {
        let some_string = String::from("yours"); //some_string comes into scope and is returned the
        //calling function
        some_string
    }
    fn takes_and_gives_back(a_string: String) -> String {
        a_string
    }

    //Using tuples can allow using values with returning ownership but isnt ideal
    //fn tuples() {
    //    let s1 = String::from("hello");
    //    let (s2, len) = calculate_len(s1);
    //    println!("The length of '{s2}' is {len}");
    //}
    //fn calculate_len(s: String) -> (String, usize) {
    //    let length = s.len();
    //    (s, length)
    //}

    //the & are references, allow refering to a value without ownernship
    //since the calculate function returns only the len, so would go out of scope
    //since s is passed as reference the calculate function doesnt take ownership
    //calculate is able to use s but s is not dropped after the function ends
    //This is called borrowing
    fn better() {
        let s1 = String::from("hello");
        let len = calculate_len(&s1);
        println!("The length of '{s1}' is {len}");
    }
    fn calculate_len(s: &String) -> usize {
        s.len()
    }

    //will error without mut becuase references are immutable unless mut is explicitly passed
    //if mut is passed there can only be one reference to that mutuable reference
    fn borrow_error() {
        let mut s = String::from("hello");
        change(&mut s);
    }
    fn change(s: &mut String) {
        s.push_str(", world");
    }

    //all ok here becuase r1 and r2 in different scopes
    fn mulitple_muts() {
        let mut s = String::from("hello");
        {
            let r1 = &mut s;
        }
        let r2 = &mut s;
    }

    fn multiple_muts_2() {
        let mut s = String::from("hello");
        let r1 = &s;
        let r2 = &s;
        println!("{r1} and {r2}");
        let r3 = &mut s; //no problem since r1 and r2 have been used and wont be again
    }

    //here we try to return a reference to s which is dropped after dangle runs
    //this normally would result in a dangling reference but rust wont allow it

    //fn danglers() {
    //    let ref_to_nothing = dangle();
    //}
    //fn dangle() -> &String {
    //    let s = String::from("hello");
    //    &s
    //}

    //this code will not error but isnt ideal becuase the index returned from first_word is
    //disconnected to the state of s
    fn slices() {
        let mut s = String::from("hello world");
        let word = first_word(&s);
        s.clear();
    }
    //fn first_word(s: &String) -> usize {
    //    let bytes = s.as_bytes();
    //    for (i, &item) in bytes.iter().enumerate() {
    //        if item == b' ' {
    //            return i;
    //        }
    //    }
    //    s.len()
    //}

    fn first_word(s: &String) -> &str {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        &s[..]
    }
}
