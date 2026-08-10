pub(crate) fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

fn deliver_order() {}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    //super is used here like .. in file navigation
    //gives access to the parent scope for code that his cupled closely
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }
    fn cook_order() {}

    pub enum Appetizer {
        Soup,
        Salad,
    }
}
//mod front_of_house {
//    pub mod hosting {
//        pub fn add_to_waitlist() {}
//        fn seat_at_table() {}
//    }
//    mod serving {
//        fn take_order() {}
//        fn serve_order() {}
//        fn take_payment() {}
//    }
//}

//Here we extract the front of house module into its own file

mod front_of_house;

//use only brings into the scope use is in
//ie if we moved the eat at restaurant function into a customer module the use line would be out of
//scope
use crate::front_of_house::hosting;
pub fn eat_at_restaurant() {
    //absolute path
    //crate::front_of_house::hosting::add_to_waitlist();

    //can shorten absolute path with use keyword
    hosting::add_to_waitlist();

    //order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    //change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("Id like {} toast please", meal.toast);

    //this line wont compile if uncommented
    //were not allowed to see or modify seasonal fruit
    //meal.seasonal_fruit = String::from("blueberries");

    //enums are most useful being defined as public
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

//
