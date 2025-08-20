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

fn main(){
    let my_string: String = String::from("helllo");
    takes_ownership(some_string: my_string);
    println!("{}", my_string); //this line causes compilTION ERRor as ownership has been removed
}

fn takes_ownership(some_string: String){
    println!("{}", some_string); //'some_string now owns the data
}