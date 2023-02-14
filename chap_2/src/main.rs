// ! 2.1  The stack, the heap, and pointers

// * str& and String are UTF-8

// fn main() {
//     let _name = "자우림"; // This is a Korean name. No problem, because a &str is UTF-8.
//     let _other_name = String::from("Adrian Fahrenheit Țepeș"); // Ț and ș are no problem in UTF-8.

//     println!("{}", _other_name)
// }

//* Even emojis are fair game
// fn main() {
//     let name = "😂";
//     println!("My name is actually {}", name);
// }

//* str is dynamic in size
// fn main() {

//     println!("A String is always {:?} bytes. It is Sized.", std::mem::size_of::<String>()); // std::mem::size_of::<Type>() gives you the size in bytes of a type
//     println!("And an i8 is always {:?} bytes. It is Sized.", std::mem::size_of::<i8>());
//     println!("And an f64 is always {:?} bytes. It is Sized.", std::mem::size_of::<f64>());
//     println!("But a &str? It can be anything. '자우림' is {:?} bytes. It is not Sized.", std::mem::size_of_val("자우림")); // std::mem::size_of_val() gives you the size in bytes of a variable
//     println!("And 'Adrian Fahrenheit Țepeș' is {:?} bytes. It is not Sized.", std::mem::size_of_val("Adrian Fahrenheit Țepeș"));
// }

// * If you remove the & it will break the script, rust NEEDS to know the value of that strin g at compile time
// fn main() {
//     let my_name: &str = "My name"; // ⚠
//     println!("{}", my_name)
// }

// * There are lots of ways to make strings. Here is the format! macro
// fn main() {
//     let my_name = "Billybrobby";
//     let my_country = "USA";
//     let my_home = "Korea";

//     let together = format!("I am {my_name} and I come from {my_country} but I live in {my_home}.");

//     println!("{}", together)
// }

//*  Here is the .into - this will err out you need to call a type
// mystring:&str OR my_string:String will do the trick
// fn main(){
//     let my_string = "This is a string".into();
//     println!("{}", my_string);
// }

// *  Const and static - const sits outside the main

// const NUMBER_OF_MONTHS: u32 = 12;

// fn print_months() { // This function takes no input!
//     println!("Number of months in the year: {NUMBER_OF_MONTHS}");
// }

// fn main() {
//     print_months();
// }

// *More on references

// fn return_str() -> &String {
//     let country = String::from("Austria");
//     let country_ref = &country;
//     country_ref // ⚠
// }

// fn main() {
//     let country = return_str();
// }

// * 2.5 Mutable References

//* Example 1
// fn main() {
//     let mut my_number = 8; // don't forget to write mut here!
//     let num_ref = &mut my_number;
//     println!("{num_ref}")
// }

//* Example 2
// fn main() {
//     let mut my_number = 8;
//     let num_ref = &mut my_number;
//     *num_ref += 10; // Use * to change the i32 value.
//     println!("{}", my_number);

//     let second_number = 800;
//     let triple_reference = &&&second_number;
//     println!(
//         "Second_number = triple_reference? {}",
//         second_number == ***triple_reference
//     );
// }

//* Example 3 - This breaks
// fn main() {
//     let mut number = 10;
//     let number_ref = &number;
//     let number_change = &mut number;
//     *number_change += 10;
//     println!("{}", number_ref); // ⚠
// }

// *Example 4 - But this works
// fn main() {
//     let mut number = 10;
//     let number_change = &mut number; // create a mutable reference
//     *number_change += 10; // use mutable reference to add 10
//     let number_ref = &number; // create an immutable reference
//     println!("{}", number_ref); // print the immutable reference
// }

//* 2.6 Shadowing
// *Example 1
// fn main() {
//     let country = String::from("Austria");
//     let country_ref = &country;
//     let country = 8;
//     println!("{}, {}", country_ref, country);
// }

// *Example 2
fn main() {
    let country = String::from("Austria"); // Now we have a String called country
    let country_ref = &country; // country_ref is a reference to this data. It's not going to change
    let country = 8; // Now we have a variable called country that is an i8. But it has no relation to the other one, or to country_ref
    println!("{}, {}", country_ref, country); // country_ref still refers to the data of String::from("Austria") that we gave it.
}
