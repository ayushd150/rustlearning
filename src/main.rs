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

// use std::fs::read_to_string  //import from seperate library

// fn main(){
//     let greeting_file_result = fs::read_to_string("hello.txt");
//     match greeting_file_result{
//         Ok(file_content)=>{
//             println!("file read successfully: {:?}", file_content);
//         },
//         Err(error)=>{
//             println!("Failed to read file: {:?}", error); 
//         }
//     }
// }

// fn main(){
//     let index = find_first_a(String::from("lovely")); //double quotes - string literal, single quotes - char literal
//     if index==-1{
//         println!("a is not found")
//     }
//     else{
//         println!("index is {}", index);
//     }
// }

// fn find_first_a (s: String) -> Option<i32>{
//     for (index,char) in s.chars().enumerate(){
//         if char=='a'{
//             return Some(index as i32);
//         }
//     }

//     return None;
// } //option means either some value or none

// why strings stored on heap :-
// they are large and their size can change at runtime, size of stack frame needs to be fixed

// rust memory management - own ownership model for memory management, safe for memory errors

//23/08
//vectors
fn main(){
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    println!("{}", vec); //[1,2]
}

fn even_filter(vec:Vec<i32>) -> Vec<i32> {
    let mut new_vec = Vec::new();
    for val in vec{
        if val % 2==0{
            new_vec.push(val);
        }
    }
    return new_vec
}

// fn main(){
//     let numbers = vec![1,2,3]
// }
//initialising by macro

//hashmaps- stores key value pair

use std::{collection::HashMap, collections::HashMap};

fn group_values_by_keys(vec:Vec<(String, i32)>) -> HashMap<String, i32>{
    let mut hm = HashMap::new();
    for (key, value) in vec{
        hm.insert(key, value);
    }
    return hm;
}


fn main(){
    let input_vec = vec![(String::from("ayush"), 22), (String::from("harish"), 23)];
    let hm = group_values_by_keys(input_vec);
    println!("{:?}", hm)
}

fn main(){
    let nums = vec![1,2,3]
    let iter = nums.iter()
    for value in iter{
        println!("{}", value)
    }
}