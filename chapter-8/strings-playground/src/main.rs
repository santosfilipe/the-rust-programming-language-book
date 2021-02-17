fn main() {
    println!("Welcome to Strings playground!");

/*      for c in "नमस्ते".chars() {
        println!("{}", c);
    } */

    for c in "FiLiPe".chars() {
        if c.is_uppercase() {
            println!("{}", c);
        }
    }
}
