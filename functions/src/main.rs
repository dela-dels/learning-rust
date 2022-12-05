fn main() {
    println!("Hello, world!");

    person_details("James", 12);

    let total = add_two_numbers(5, 1);

    println!("Summation is: {}", total);
}

//Function with parameters and their types
fn person_details(name: &str, age: u8) {
    println!("Name is: {} and Age is: {}", name, age);
}

//Function with parameters, their types and also a return type for the function
fn add_two_numbers(first_number: u8, second_number: u8) -> u8 {
    first_number + second_number
}
