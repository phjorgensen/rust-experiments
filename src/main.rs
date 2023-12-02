pub mod rust;
use rust::add_one;
use rust::borrow_checker;
use rust::enums;
use rust::iterator;
use rust::optional;
use rust::optional_exersice;
use rust::read_file;
use rust::read_numbers;
use rust::traits;

fn main() {
    // add_one::with_iter();
    // add_one::other_things_with_iter();
    // read_file::read_every_line();
    // read_file::read_every_other_line();
    // read_file::read_every_other_line_skip();
    // enums::print_enum_values();
    // enums::use_impl();
    // optional::secure_types_example_1();
    // optional::secure_types_example_2();
    // optional::secure_types_example_3();
    // println!("{}", optional_exersice::practice(vec![1, 2, 3], 0));
    // println!("{}", optional_exersice::practice(vec![1, 2, 3], 1));
    // println!("{}", optional_exersice::practice(vec![1, 2, 3], 2));
    // println!("{}", optional_exersice::practice(vec![1, 2, 3], 3));
    // println!("{}", optional_exersice::practice(vec![1, 2, 3], 4));
    // read_numbers::read_numbers();
    // borrow_checker::main();
    traits::main();
    // iterator::main();
}
