use std::io;

fn main() {
    let mut temp = String::new();
    let mut units = String::new();
    println!("Enter temperature value:");
        
    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");

    println!("Enter units:");
    io::stdin()
        .read_line(&mut units)
        .expect("Failed to read line");


    let temp: i32 = temp.trim()
        .parse()
        .expect("Can't parse numeric value");
    let units = units.trim();
    
    if units == "F" {
        println!("{temp}°F is {}°C", fahrenheit2celsius(temp));
    } else if units == "C" {
        println!("{temp}°C is {}°F", celsius2fahrenheit(temp));
    }
}

fn fahrenheit2celsius(t: i32) -> i32 {
    (t - 32) / (9 / 5)
}

fn celsius2fahrenheit(t: i32) -> i32 {
    t * (9 / 5) + 32
}



