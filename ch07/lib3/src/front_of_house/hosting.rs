pub enum Appetizer {
    Soup,
    Salad,
}

pub fn add_to_waitlist() {
    seat_at_table();
    super::serving::take_order();
    self::seat_at_table();

    let a = Appetizer::Soup;
    let b = Appetizer::Salad;
}

fn seat_at_table() {}
