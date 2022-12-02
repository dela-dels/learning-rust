// fn main() {
//     //let x = 5;
//     let mut x = 5; //without explicitly saying we want x to be mutable, we get a compile error
//     println!("The value of x is: {x}");
//     x = 6;
//     println!("The value of x is: {x}");

// }

//Shadowing
// fn main() {
//     let x = 5;

//     let x = x + 1;

//     //Inner scope
//     {
//         let x = x * 2;
//         println!("The value of x in the inner scope is: {x}");
//     }
//     println!("The value of x is: {x}");
// }

// fn main() {
//     // addition
//     let sum = 5 + 10;
//     println!("Summation results: {sum}");

//     // subtraction
//     let difference = 95.5 - 4.3;
//     println!("Subtraction results: {difference}");

//     // multiplication
//     let product = 4 * 30;
//     println!("Product results: {product}");

//     // division
//     let quotient = 56.7 / 32.2;
//     println!("Division results: {quotient}");

//     let floored = 2 / 3; // Results in 0
//     println!("Floored results: {floored}");

//     // remainder
//     let remainder = 43 % 5;
//     println!("Remainder results: {remainder}");
// }

//Character types
// fn main() {
//     let c: char = 'c';

//     println!("{c}");
// }

//Compound Types (Tuples)
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The values in the tuple are: x: {}, y: {}, z: {}", x, y, z);

    println!("Index 0 of tuple is: {}", tup.0);

    println!("This is the tuple {:?}", tup);

    let cars = ["Volvo", "Toyota", "Mercedes", "Ferrari"];

    println!("The first element in the cars array is  {}", cars[0]);

    let days_of_the_week: [&str; 7] = [
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
        "Sunday",
    ];

    println!("First day of the week is {}", days_of_the_week[0]);
}
