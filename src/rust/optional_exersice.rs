pub fn practice(list: Vec<usize>, index: usize) -> usize {
    return list.get(index).unwrap_or(&index) * 5;
    // return match list.get(index) {
    //     Some(item) => item * 5,
    //     None => index * 5,
    // };
}
