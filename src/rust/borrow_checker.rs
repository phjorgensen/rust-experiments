#[derive(Debug)]
struct Item {
    count: usize,
}

pub fn main() {
    // let mut item = Item { count: 1 };
    // println!("{:?}", item);
    // add_one(&mut item);
    // println!("{:?}", item);

    // let mut items = vec![Item { count: 0 }, Item { count: 1 }, Item { count: 2 }];
    // let first = items.first_mut();
    // println!("{:?}", first);
    // print_all(&items);
    // println!("{:?}", first);

    // let mut items2 = vec![Item { count: 0 }, Item { count: 1 }, Item { count: 2 }];
    // let first2 = items2.get_mut(0);
    // let second2 = items2.get_mut(1);
    // println!("{:?}", second2);

    // let new_item = Item { count: 2 };
    // println!("{:?}", new_item);
    // let new_item = add_to_count(new_item, 9);
    // println!("{:?}", new_item);

    // let items = vec![1, 2, 3]
    //     .iter()
    //     .map(|x| x + 1);

    // println!("{:?}", items);
}

fn add_one(item: &mut Item) {
    println!("add_one");
    item.count += 1;
}

fn print_all(items: &Vec<Item>) {
    println!("print_all");
    items.iter().for_each(|item| println!("{:?}", item))
}

fn add_to_count(mut item: Item, amount: usize) -> Item {
    item.count += amount;
    return item;
}
