mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // 絶対
    crate::front_of_house::hosting::add_to_waitlist();

    // 相対
    front_of_house::hosting::add_to_waitlist();
}
