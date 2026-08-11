pub(crate) fn main() {
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }
    //the instance must be either entirly mutable or not
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@someone.com");

    fn build_user(email: String, username: String) -> User {
        User {
            active: true,
            username,
            email,
            sign_in_count: 1,
        }
    }
    //user1 become invalid after delcaring user2 becuase of the username property
    //username is a string which doesnt have the copy trait, email is a new string
    //and active/sign in count are bool and in types with the copy trait
    let _user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    //tuple structs

    //tuple structs are different types even if they hold the same values
    //to destructure them you must include the tuple struct type
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let _black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let Point(_x, _y, _z) = origin;

    //the problem with this code is that its unclear that width and height are related
    //also our function is supposed to calculate one rectangle area but has 2 parameters
    fn create_rectangle() {
        let width1 = 30;
        let height1 = 50;

        println!(
            "The area of the rectangle is {} square pixels.",
            area(width1, height1)
        );
    }
    fn area(width: u32, height: u32) -> u32 {
        width * height
    }
    //create_rectangle();

    struct Rectangle {
        height: u32,
        width: u32,
    }
    fn rectangle_refactor() {
        let rect1 = Rectangle {
            height: 30,
            width: 50,
        };
        println!(
            "The area of the rectangle is {} square pixels.",
            area_refactor(&rect1)
        );
    }
    fn area_refactor(rectangle: &Rectangle) -> u32 {
        rectangle.height * rectangle.width
    }
    rectangle_refactor();

    //println!(rect1) will error becuase rust does not know which format to output the struct as
    fn debugging_structs() {
        #[derive(Debug)]
        struct Rectangle {
            height: u32,
            width: u32,
        }
        fn debug() {
            let rect1 = Rectangle {
                width: 30,
                height: 50,
            };
            //:? tells println! to to use an output format called debug the # makes it object form
            println!("rect1 is {rect1:#?}")
        };
        debug();
    };
    //debugging_structs();

    fn other_debug_methods() {
        #[derive(Debug)]
        struct Rectangle {
            height: u32,
            width: u32,
        }
        let scale = 2;
        let rect1 = Rectangle {
            width: dbg!(30 * scale),
            height: 50,
        };
        dbg!(&rect1);
    }
    //other_debug_methods();

    //Methods

    fn method_syntax() {
        #[derive(Debug)]
        struct Rectangle {
            width: u32,
            height: u32,
        }
        impl Rectangle {
            fn area(&self) -> u32 {
                self.width * self.height
            }
        }
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };
        println!(
            "The area of the rectabnle is {} square pixels",
            rect1.area()
        );
    };
    method_syntax();
}
