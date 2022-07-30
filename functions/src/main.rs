fn main() {
    println!("Hello, world!");

    another_function(117, 34);

    let y = expression_example();
    println!("The value of y is: {}", y);
}

fn another_function(x: u32, y: u32) {
    println!("The value of x is: {} and the value of y is: {}", x, y);
}

fn expression_example() -> u32 {
    let _x = 5;

    {
        let x = 3;
        x + 1
    }
}
