fn main() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("{THREE_HOURS_IN_SECONDS}");

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let spaces = "   ";
    println!("The value of spaces is: {spaces}");
    let spaces = spaces.len();
    println!("The value of spaces is: {spaces}");

    // addition
    let sum = 5 + 10;
    println!("The value of sum is: {sum}");

    // subtraction
    let difference = 95.5 - 4.3;
    println!("The value of difference is: {difference}");

    // multiplication
    let product = 4 * 30;
    println!("The value of product is: {product}");

    // division
    let quotient = 56.7 / 32.2;
    println!("The value of quotient is: {quotient}");
    let truncated = -5 / 3; // Results in -1
    println!("The value of truncated is: {truncated}");

    // remainder
    let remainder = 43 % 5;
    println!("The value of remainder is: {remainder}");

    // Tuple
    let _tup: (i32, f64, u8) = (500, 6.4, 1);
    // Array
    let _a: [i32; 5] = [1, 2, 3, 4, 5];
}
