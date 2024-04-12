// rustbook 7.2

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

fn deliver_order() {}

mod back_of_house {

    pub struct Breakfast {  // we can make structs public..
        pub toast: String,  // .. then make fields public case-by-case
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

    pub enum Appetiser { // in contract to structs, a public enum..
        Soup,  // .. has all its variants public
        Salad,
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();  // we can use super like '..' in a filepath
    }

    fn cook_order() {}
}

// we can also bring crates into scope without going through module tree paths
use crate::front_of_house::hosting;
// this only brings it into scope for where we currently are
// e.g., this won't work
// mod example_scope {
//     hosting::add_to_waitlist();
// }

// we can make a 'use' public facing as well, to shorten calls via the API
// e.g., the below shortens public calls to our lib from
// restauraunt::front_of_houes::hosting::add_to_waitlist()
// to restaurant::hosting::add_to_waitlist()
pub use crate::front_of_house::hosting;

// we can write a public function that calls these modules
pub fn eat_at_restau () {
    // we can refer with absolute paths
    // the crate keyword works as front_of_house is in the same crate as eat_at_restau
    crate::front_of_house::hosting::add_to_waitlist();

    // or relative paths
    front_of_house::hosting::add_to_waitlist();

    // order a breakfast in the summer with rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // change the toast
    meal.toast = String::from("White");
    println!("I'd like {} toast please", meal.toast);

    // the fruit is private, so this won't work
    // meal.seasonal_fruit = String::from("Apple");

    // we can use all variants from a public enum
    let starter1 = back_of_house::Appetiser::Soup;
    let starter2 = back_of_house::Appetiser::Salad;

    // brought into scope with use
    // note we could have just added add_to_waitlist to scope
    // but typically we include the parent module to indicate that its not defined in scope
    hosting::add_to_waitlist();
}