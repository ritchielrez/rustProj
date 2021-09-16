fn main() {
    let mut x = 185;

    // Named arguments
    println!("{num}", num = x);

    let spaces = "  ";
    println!("Spaces: {}", spaces);

    let spaces = spaces.len();
    println!("Spaces: {}", spaces);
    
    // Tuple types
    let tuple: (i32, f64, bool) = (500, 23.34, true);
    println!("Tuple: {:?}", tuple);

    // Array types
    let array: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("Array: {:?}", array);
}
