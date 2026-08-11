fn main() {
    //structs let you group together data and fields ie Rectangle {height,width}
    //enums let you say data is one of a possible set of values
    enum IpAddr {
        V4(String),
        V6(String),
    }
    let _home = IpAddr::V4(String::from("12.0.0.1"));
    let _loopback = IpAddr::V6(String::from("::1"));

    //enums can have different types and amounts of data associated with them
    enum _IpAddr2 {
        V4(u8, u8, u8),
        V6(String),
    }

    fn _struct_vs_enum() {
        //Same types, difference between structs and enums
        //Cant use the data defined with structs as easily
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32),
        }

        struct QuitMessage;
        struct MoveMessage {
            x: i32,
            y: i32,
        };
        struct WriteMessage(String);
        struct ChangeColorMessage(i32, i32, i32);
    }

    //can define methods on enums
    enum Message {
        _Quit,
        _Move { x: i32, y: i32 },
        Write(String),
        _ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            //method here
        }
    }
    let m = Message::Write(String::from("hello"));
    m.call();

    //How does rust deal with null?
    //rust does not have a null type but does have an Option<T> enum

    let _some_number = Some(5);
    let _some_char = Some('e');
    let _absent_number: Option<i32> = None;

    //this will not compile because rust doesnt know exactly what type y is
    //let x: i8 = 5;
    //let y: Option<i8> = Some(5);
    //let sum = x + y;

    //Option<T> is used to guard against pervasive null use and its errors
    //When using Option<T> you must narrow the cases when the data is null and it is not

    //This is an exmample how how to get data out of a
    #[derive(Debug)]
    enum UsState {
        _Alabama,
        _Alaska,
        California,
    }

    enum Coin {
        _Penny,
        _Nickel,
        _Dime,
        Quarter(UsState),
    }
    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::_Penny => 1,
            Coin::_Nickel => 5,
            Coin::_Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {state:?}!");
                25
            }
        }
    }
    value_in_cents(Coin::Quarter(UsState::California));

    //this pattern is seen a lot in rust
    //match against a variable bind variable to data inside and execute code based on the match
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
    let five = Some(5);
    let _six = plus_one(five);
    let _none = plus_one(None);

    //other is used as a catch all here
    //because match must be exhaustive other can be used to pass type checks
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
        // _ => (), can also be used if we only want code to run in the prior cases
    }
    fn add_fancy_hat() {};
    fn remove_fancy_hat() {};
    fn move_player(num_spaces: u8) {};

    // Using if let to bind a variable if only condition is present for a successful match case
    // this first block is unnessary boilerplate code
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }

    let config_max_2 = Some(3u8);
    if let Some(max) = config_max_2 {
        println!("The maximum is configured to be {max}");
    }
}
