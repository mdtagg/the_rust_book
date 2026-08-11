pub(crate) fn main() {
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
        let _s2 = s1;
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

        let _s = String::from("hello");
        //takes_ownership(s);
        let _x = 5;
        //makes_copy(x); here x is does not move into the function and can be used after
    }

    fn ownership() {
        let _s1 = gives_ownership(); // moves its return value to s1
        let s2 = String::from("hello");
        let _s3 = takes_and_gives_back(s2); //s2 is moved into takes_and_gives_back, moves its return
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
            let _r1 = &mut s;
        }
        let _r2 = &mut s;
    }

    fn multiple_muts_2() {
        let mut s = String::from("hello");
        let r1 = &s;
        let r2 = &s;
        println!("{r1} and {r2}");
        let _r3 = &mut s; //no problem since r1 and r2 have been used and wont be again
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
        let _word = first_word(&s);
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
