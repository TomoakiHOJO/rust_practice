mod models;

use models::{Somecode, Othercode, print_data};

fn main() {
    let code = Somecode::new(10, 20);
    let other = Othercode::new(3.14, 2.71);
    print_data(&code);
    print_data(&other);
}