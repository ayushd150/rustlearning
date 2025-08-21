// fn main() {
//     println!("Hello, world!");
//     let x: i32 = -5;
//     let y: i 32 = 1000;
//     let z: f32 = 1000.001;
// }


// fn main(){
//     let sentence: String = String::from("my name is ayush");
//     let first_word: String = get_first_word(sentence);
//     print!("first word: {}", first_word)
// }

// fn get_first_word(sentence: String) -> String{
//     for char in sentence.chars() {
//         if char == ' ' {
//             break;
//         }
//     }
// }

// fn main(){
//     let mut x: String = String::from("hi there");
//     x=2;
//     println!("{}", x)
// }

// fn main(){
//     let my_string: String = String::from("helllo");
//     takes_ownership(some_string: my_string);
//     println!("{}", my_string); //this line causes compilTION ERRor as ownership has been removed
// }

// fn takes_ownership(some_string: String){
//     println!("{}", some_string); //'some_string now owns the data
// }


// enum Shape{
//     Rectangle(f64, f64),
//     Circle(f64),
// } 
// //enums let u enumerate over various variants of a specific type

// fn main(){
//     let my_shape = Shape::Rectangle(1.0,2.0);
//     calculate_area(shape:rect)
//     let circle = Shape::Circle(1.0);
//     calculate_area(shape:circle);
// }

// fn calculate_area(shape: Shape){
//     match Shape{
//         Shape::Rectangle(a,b) =>a*b
//         Shape::Circle(r)=> 3.14*r*r;
//     }
// }

use std::fs::read_to_string

fn main(){
    let greeting_file_result = fs::read_to_string("hello.txt");
    match greeting_file_result{
        Ok(file_content)=>{
            println!("file read successfully: {:?}", file_content);
        },
        Err(error)=>{
            println!("Failed to read file: {:?}", error); 
        }
    }
}