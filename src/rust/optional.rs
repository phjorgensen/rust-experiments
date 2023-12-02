struct Custom {
    age: usize,
    name: String,
}

enum Item {
    Number(usize),
    String(String),
    MyCustom(Custom),
}

pub fn secure_types_example_1() {
    let mut items: Vec<Item> = vec![];
    append(&mut items);

    // Not allowed, since items is array of usize, not Item
    // let mut items: Vec<usize> = vec![];
    // append(&mut items);
}

fn append(items: &mut Vec<Item>) {
    items.push(Item::String("Hello, Fem!".to_string()));
}


pub fn secure_types_example_2() {
    // let foo = Some(5);
    // let foo: Option<String> = None;

    // if Some(x) = foo {
    //     println!("Is string")
    // }
}

pub fn secure_types_example_3() {
    // println!("{}", ensure_number(0));
    // println!("{}", ensure_number(10));
    // println!("{}", ensure_number(None));

    // println!("{}", handle_number(0));
    // println!("{}", handle_number(10));
    // println!("{}", handle_number(None));
}

fn ensure_number(num: Option<usize>) -> usize {
    return num.unwrap_or(0) * 5;
}

fn handle_number(num: Option<usize>) -> Option<usize> {
    return Some(num? * 5);
}
