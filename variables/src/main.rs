fn main() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;

    println!("The value of x is: {}", x);

    let y = 2.0;
    let z: f32 = 3.0;
    println!("{} + {} is: {}", y, z, y + z);

    // tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // Two ways of destructuring:
    // let (x, y, z) = tup;
    let y2 = tup.1;
    println!("The value of y is: {}", y2);

    let a2: [i32; 5] = [1, 2, 3, 4, 5];
    let a3 = [3; 5];
    let first = a2[0];
    let second = a2[1];
}
