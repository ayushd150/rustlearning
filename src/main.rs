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
// fn main(){
//     let mut vec = Vec::new();
//     vec.push(1);
//     vec.push(2);
//     println!("{}", vec); //[1,2]
// }

// fn even_filter(vec:Vec<i32>) -> Vec<i32> {
//     let mut new_vec = Vec::new();
//     for val in vec{
//         if val % 2==0{
//             new_vec.push(val);
//         }
//     }
//     return new_vec
// }

// fn main(){
//     let numbers = vec![1,2,3]
// }
//initialising by macro

//hashmaps- stores key value pair

// use std::{collection::HashMap, collections::HashMap};

// fn group_values_by_keys(vec:Vec<(String, i32)>) -> HashMap<String, i32>{
//     let mut hm = HashMap::new();
//     for (key, value) in vec{
//         hm.insert(key, value);
//     }
//     return hm;
// }


// fn main(){
//     let input_vec = vec![(String::from("ayush"), 22), (String::from("harish"), 23)];
//     let hm = group_values_by_keys(input_vec);
//     println!("{:?}", hm)
// }

// fn main(){
//     let nums = vec![1,2,3]
//     let iter = nums.iter()
//     for value in iter{
//         println!("{}", value)
//     }
// }


// fn main(){
//     let mut v1 = vec![1,2,3]
//     let v1_iter = v1.iter_mut();
//     for val in v1_iter{
//         *val=*val+1 //*val means derefrence */
//     }
//     println!("{:?}", v1) //You’ll get an error because a vector (Vec<i32>) does not implement Display (the trait behind {}). To fix this, Rust provides another placeholder: {:?}
// It uses the Debug trait, which is meant for debugging / developer output.
// }

//intolter - convert collection to iterator takes ownership of collection
//iter - if u want immutrable ref to inner var adnd dont want to transfer ownership
//itermut - mutable ref to inner var and dont want to transfer ownership
//iterinto - move var to the iterator and dont use if afterwards

//In Rust, an iterator is something that produces items one by one.

// An iterator adaptor is a method that takes one iterator and returns another iterator, usually transforming or filtering the items along the way.

// Importantly: adaptors are lazy → they don’t actually do any work until you call a “consumer” method (like collect, sum, for, etc.).


// fn main() {
//     let nums = vec![1, 2, 3, 4, 5, 6];

//     let result: Vec<i32> = nums.iter()
//                                .filter(|x| *x % 2 == 1) // keep only odd numbers
//                                .map(|x| x * 2)          // double each
//                                .collect();              // turn into new Vec

//     println!("{:?}", result); // [2, 6, 10]
// }


// fn main() {
//     let mut s = String::from("Hello"); // owns heap memory
//     s.push_str(", world!");            // can be mutated
//     println!("{}", s);                 // "Hello, world!"
// }


// fn main(){
//     let word = String::from("hello world");
//     let word2 = &word[0..5];
// //u can have multiple immutable references
// //if u have mutable reference, u cannot have other muttable/immutable references

//     word2.clear();
//     println!("{}", word2);
// }

// fn main() {
//     let mut word = String::from("Hello world");
//     let word2 = find_first_word(word); //Calls find_first_word, moving ownership of word into the function because the parameter type is String (owned). After this line, word in main is no longer usable.
//     println!("{}", word);
// }

// fn find_first_word(word: String) -> &str {
//     let mut index = 0;
//     for (_, i) in word.chars().enumerate() {
//         if i == ' ' {
//             break;
//         }
//         index = index + 1;
//     }
//     return &word[0..index];
// }


// fn main(){
//     let name = String::from("hello world");
//     let mut space_index = 0;
//     for i in name.chars(){
//         if i == ' '{
//             break;
//         }
//         space_index+=1
//     }
//     let ans = &name[0..space_index];
//     println!("Ans is {}", ans);

// }

//use of &name
// name[0..space_index] produces a string slice (&str).

// A slice is not a new string. It’s just a view (pointer + length) into the existing string’s data.

// So ans is a &str that borrows from name.
// Rust would try to move ownership of part of the string out of name.
// But that’s not possible — you can’t partially move out of a String.
// Instead, you borrow (&) to create a reference to a part of it.

//25/08
//fn identity<T>(x: T) -> T {
//     x
// }
// T is a generic type parameter.

// identity works with any type: integers, strings, structs, etc.

trait Summary{
    fn summarize (&self) -> String;
}

struct User{
    name:String,
    age:u32,
}
impl Summary for User{
    fn summarize (&self) -> String {
        return format!("the name is {} and the age is {}", self.name, self.age)
    }
}

fn main(){
    let user = User{
        name: String::from("ayush"),
        age:21,
    };
    println!("{}", user.summarize());
}

//trait bound syntax
//single struct can implement multiple traits
// pub fn notify<T: Summary>(item: T){
//     println!("Breaking news {}", item.summarize());
// }