mod back_of_house;
mod front_of_house;

pub use crate::front_of_house::hosting;

pub use crate::back_of_house::cook;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();

    cook::order();
}