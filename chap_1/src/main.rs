//*Example 1
// fn main() {
//     let mut my_name: String = "Dave".to_string();
//     my_name.push('!');
//     println!/*test*/("{}", my_name);
// }


//* Example 2
// fn main() {
//     let first_letter = 'A';
//     let space = ' '; // A space inside ' ' is also a char
//     let other_language_char = 'Ꮔ'; // Thanks to Unicode, other languages like Cherokee display just fine too
//     let cat_face = '😺'; // Emojis are chars too

//     println!("{first_letter} {space} {other_language_char} {cat_face}")
// }

//* Example 3
// fn main(){
    //?This will print d because that is the char in place 100
//     let my_number= 100;

//     println!("{}", my_number as u8 as char)
// }

//* Example 4 Casting errors
// fn main() {
//     let my_number = 600;
//     println!("{}", my_number as u8);
// }

//* Example 5 - fixed casting
// fn main() {
//     let my_number: u8 = 100; //  change my_number to my_number: u8
//     let second_num = 10;
//     println!("{}", my_number as char);
//     println!("{}", my_number as char);
// }

//* Example 6 - Getting the len down
// fn main() {
//     println!("Size of a char: {}", std::mem::size_of::<char>()); // 4 bytes
//     println!("Size of string containing 'a': {}", "a".len()); // .len() gives the size of the string in bytes
//     println!("Size of string containing 'ß': {}", "ß".len());
//     println!("Size of string containing '国': {}", "国".len());
//     println!("Size of string containing '𓅱': {}", "𓅱".len());
// }

// //* Example 7 - Show the bytes
// fn main() {
//     println!("{:?}", "a".as_bytes());
//     println!("{:?}", "ß".as_bytes());
//     println!("{:?}", "国".as_bytes());
//     println!("{:?}", "𓅱".as_bytes());
// }

//* Example 8 - show the char count
// fn main() {
//     let slice = "Hello!";
//     println!("Slice is {} bytes and also {} characters.", slice.len(), slice.chars().count());
//     let slice2 = "안녕!";
//     println!("Slice2 is {} bytes but only {} characters.", slice2.len(), slice2.chars().count());
// }

// * Example 9 - Intro to floats

// fn main() {
//     let my_float: f64 = 5.0;
//     let my_other_float: f32 = 8.5;

//     let _third_float = my_float + my_other_float as f64;
//     println!("{_third_float}")

// }


// * Example 10 - Make a function
fn number() -> i32{
    8
}

fn main(){
    println!("Hello, world number {}!", number());
}