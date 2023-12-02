use std::collections::{HashMap, HashSet};

pub fn with_iter() {
    let data = vec![1, 2, 3];
    let list: Vec<u8> = data.iter().map(|x| x + 1).collect();
    println!("{:?}", list);
}

pub fn other_things_with_iter() {
    let data = vec![1, 2, 3];
    let mut list = data.iter().map(|x| x + 1);

    let mut new_vector = vec![];

    while let Some(x) = list.next() {
        new_vector.push(x);
    }

    println!("{:?}", list);

    let joined: String = vec!["this", "is", "a", "test"].into_iter().collect();
    println!("{:?}", joined);

    let hash_set: HashSet<isize> = vec![1, 2, 3].into_iter().collect();
    println!("{:?}", hash_set);

    let hash_map: HashMap<&str, usize> = vec!["this", "is", "a", "test"]
        .into_iter()
        .enumerate()
        .map(|(idx, item)| (item, idx))
        .collect();
    println!("{:?}", hash_map);
    println!("{:?}", hash_map.get("test"));

    let total: i8 = vec![1, 2, 3].iter().sum();
    println!("{}", total);
}
