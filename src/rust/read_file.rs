use std::fs::read_to_string;

pub fn read_every_line() {
    let file = read_to_string("lines").unwrap();

    file.lines().for_each(|line| println!("{}", line));
}

pub fn read_every_other_line() {
    let file = read_to_string("lines").unwrap();

    file.lines()
        .enumerate()
        .filter(|(idx, _)| idx % 2 == 0)
        .for_each(|(_, line)| println!("{}", line));
}

pub fn read_every_other_line_skip() {
    let file = read_to_string("lines").unwrap();

    file.lines()
        .enumerate()
        .filter(|(idx, _)| idx % 2 == 0)
        .skip(2)
        .take(2)
        .for_each(|(_, line)| println!("{}", line));
}

