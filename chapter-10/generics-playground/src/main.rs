struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.5, y: 4.7 };

    println!("Interger struct: x {}, y {}.", integer.x, integer.y);
    println!("Interger float: x {}, y {}.", float.x, float.y);
}
