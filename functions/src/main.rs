fn main() {
    println!("Hello, world!");

    another_function(117, 34);

    let y = expression_example();
    println!("The value of y is: {}", y);

    let y_plus_three = add_three(y);
    println!("The value of y_plus_three is: {}", y_plus_three);
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

fn add_three(num: u32) -> u32 {
    let result = num + 3;

    return result;
}
