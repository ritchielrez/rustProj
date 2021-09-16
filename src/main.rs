fn main() {
    let mut x = 185;

    // Named arguments
    println!("{num}", num = x);

    let spaces = "  ";
    println!("Spaces: {}", spaces);

    let spaces = spaces.len();
    println!("Spaces: {}", spaces);
}
