pub mod hosting;

mod serving {
    use Appetizer::Soup;

    use crate::front_of_house::hosting::Appetizer;

    pub fn take_order() {
        let a = Soup;
    }

    fn serve_order() {}

    fn take_payment() {}
}
