fn main() {
    let v = vec![0, 1, 2, 3, 4, 5];

    let first: &i32 = &v[0];
    println!("The first element is {}", first);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
}
