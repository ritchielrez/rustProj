fn test() {
    let x = 5;

    let y = {
        let x = 12;
        x + 1
    };

    println!("X: {:?}", x);
    println!("Y: {:?}", y);
}

fn main() {
    let x = 185;

    // Named arguments
    println!("{num}", num = x);

    let spaces = "  ";
    println!("Spaces: {}", spaces);

    let spaces = spaces.len();
    println!("Spaces: {}", spaces);

    // Tuple types () brackets
    let tuple: (i32, f32, bool, &str) = (23, 23.34, true, "Test");
    println!("Tuple: {:?}", tuple);
    println!("Tuple 1st Index: {:?}", tuple.0);

    // Array types
    let arr: [isize; 5] = [0,1, 2, 3, 4];
    println!("Array: {:?}", arr);

    // Functions
    test();
}

