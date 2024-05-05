fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");

    let x = x + 1; // shadowing, new value is 6
    
    {
        let x = x * 2; // shadowing again, new value is 12
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    const THREE_HOURS_IN_SECONDS: i32 = 3 * 60 * 60;
    println!("{}", THREE_HOURS_IN_SECONDS);

    let spaces = "   ";
    println!("spaces follow{spaces}.");
    let spaces = spaces.len(); // creating new variable, we can change the type
    println!("Number of spaces is {spaces}");

    // counter-example, incompatible types:
    // let mut spaces = "   ";
    // spaces = spaces.len();
}