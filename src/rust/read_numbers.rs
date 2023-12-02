pub fn read_numbers() {
    let file_name = std::env::args().nth(1).expect("the file name to passed in");

    let file = std::fs::read_to_string(file_name).expect("unable to read the file to string");

    file.lines().for_each(|line| {
        if let Ok(value) = line.parse::<usize>() {
            println!("{}", value);
        } else {
            println!("Line is not a number");
        }
        // match line.parse::<usize>() {
        //     Ok(line) => println!("{}", line),
        //     Err(_) => println!("Line is not a number"),
        // };
    });
}
