fn main() {
    // Declare Vector
    let v = vec![0, 1, 2, 3, 4, 5];

    // Declare variable to get the first element of &v
    let first: &i32 = &v[0];
    println!("The first element is {}", first);

    // -match- can be used to access a vector element in a safe manner 
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // Printing all elements from the vector &v
    for i in &v {
        println!("{}", i);
    }
}
