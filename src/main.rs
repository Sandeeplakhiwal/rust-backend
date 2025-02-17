use std::{f64::consts::PI, fs};


fn get_first_word(str: String)->String {
    let mut first_word = String::from("");
    for c in str.chars() {
        first_word.push(c);
        if c == ' ' {
            break;
        }
    };
    return first_word
}

fn do_sum(a:i32, b:i32)-> i32 {
    return a+b
}

fn take_ownership(some_string: String){
    println!("{}", some_string);
}

fn borrow_variable(str: &String) {
    println!("Borrowed string: {}", str);
}

fn update_borrwed_string(some_string: &mut String) {
    some_string.push_str(", New Word");
}

struct User {
    name: String,
    email: String,
    age: u8,
    active: bool
}

struct Rect {
    height: u32,
    width: u32
}

impl Rect {
    fn area(&self)->u32 {
        return self.height * self.width;
    }
}

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West
}

fn move_around(direction: Direction) {
    println!("I am moving towards {:?}", direction);
}

enum Shape {
    Circle(f64),
    Square(f64),
    Rectangle(f64, f64)
}

fn calculate_area(shape: Shape)->f64 {
    let ans = match shape {
        Shape::Circle(radius) => PI * radius*radius,
        Shape::Square(side_length) => side_length * side_length,
        Shape::Rectangle(width, height) => width*height
    };

    return ans;
}

fn main() {
    println!("Hello World");

    let x:i8 = 6;
    let y = 4;
    let z = 67.8;

    println!("x+y: {}, y: {}, z: {}", x+y, y, z);

    let is_male = false;

    let is_above_18 = true;

    if is_male {
        println!("You are male")
    } else {
        println!("You are not a male")
    }

    if is_male && is_above_18 {
        println!("You are a legal man")
    }

    // let greet = "Hello World";
    let mut greet = String::from("Hello World");

    greet.push_str(", how are you?");

    println!("Greet: {}", greet);

    if let Some(first_letter) = greet.chars().nth(0){
        println!("First letter: {}", first_letter);
    }else {
        println!("String is empty");
    }

    for i in 1..100 {
        print!("{}, ", i)
    }

    let intro = String::from("My name is Sandeep");

    let intro2 = String::from("how are you?");
      
    let combined_intro = format!("{}, {}", intro, intro2);

    println!("\nCombinedIntro: {}", combined_intro);

    // let first_word = get_first_word()
    let first_word = get_first_word(greet);

    println!("First Word is: {}", first_word);

    let sum_res = do_sum(5, 4);

    println!("Sum of 5 & 4 is: {}", sum_res);

    // Ownership

    let my_string = String::from("This is my string");

    // let my_string2:i32 = 55;

    take_ownership(my_string.clone());

    println!("After borrowed {}", my_string);

    // Borrow and references
    let mut s1 = String::from("Hello");

    let s2 = &s1;

    println!("s1: {}", s1);
    println!("s2: {}", s2);

    borrow_variable(&s2);

    println!("s2 after borrow: {}", s2);

    update_borrwed_string(&mut s1);

    println!("s1 after update borrowed string: {}", s1);

    /* Struct */
    let user1 = User {
        active: true,
        name: String::from("Sandeep Lakhiwal"),
        email: String::from("sandeeplakhiwal98@gmail.com"),
        age: 20
    };

    println!("User1=> name: {:?}, email: {:?}", user1.name, user1.email);

    let rect = Rect {
        height: 30,
        width: 25
    };

    println!("The area of the rectangular is {:?}", rect.area());

    /* Enums */
    move_around(Direction::North);

    /* Enums with values */
    let circle = Shape::Circle(5.0);
    let square = Shape::Square(4.0);
    let rectangle = Shape::Rectangle(3.0, 6.0);
    

    let area_of_square = calculate_area(square);

    println!("Area of square: {:?}", area_of_square);


    /* Error handling */
    let res = fs::read_to_string("src/example.txt");

    match res {
        Ok(content) => {
            println!("File content: {}", content);
        }
        Err(err) => {
            println!("Error: {}", err)
        }
    }

}



// use std::{collections::HashMap, fs};
// use chrono::{Local, Utc};
// use rand::{Rng, thread_rng};

// struct User {
//     name: String,
//     email: String
//     age: u8,
//     active: bool
// }

// struct Rect {
//     height: u32,
//     width: u32
// }

// impl Rect {
//     fn area(&self) -> u32 {
//         return self.height * self.width
//     }
// }

// /* Enums */
// // enum Direction {
// //     North,
// //     // South,
// //     // East,
// //     // West
// // }

// // fn move_around(direction: Direction) {
// //     // println!("I am moving towards {:?}", direction);
    
// // }

// /* Enums with values */
// enum Shape {
//     Circle(f64),
//     Square(f64),
//     Rectangle(f64, f64)
// }

// fn calculate_area(shape: Shape) -> f64 {
//     // Pattern matching in enums
//     let ans = match shape {
//         Shape::Circle(radius) => std::f64::consts::PI * radius*radius,
//         Shape::Square(side_length) => side_length*side_length,
//         Shape::Rectangle(width, height) => width*height
//     };

//     return  ans;
// }

// /* Error handling */

// // enum Result<T, E> {
// //     Ok(T),
// //     Err(E)
// // }

// /* Option Enum */

// // pub enum Option<T> {
// //     None,
// //     Some(T)
// // }


// fn main() {
//     println!("Hello, world!");
    
//     let x: i8 = 6;
//     let y = 4;
//     let z = 67.8;

//     println!("x: {}, y: {}, z: {}", x+y, y, z);

//     let is_male = true;
//     let is_above_18 = true;

//     if is_male {
//         print!("You are a male.\n")
//     } else {
//         print!("You are not a male.\n")
//     }

//     if is_male && is_above_18 {
//         print!("You are a legal man.\n")
//     }

//     let greet = String::from("Hello");

//     println!("{} Harsh", greet);

//     // let first_letter = greet.chars().nth(n)

//     // for i in 1..100{
//     //     println!("{} ", i);
//     // }


//     let intro = String::from("My name is Sandeep,");

//     let intro2: String = String::from("how are you?");

//     let combined_intro = format!("{} {}", intro, intro2);

//     println!("CombinedIntro: {}", combined_intro);

//     let first_word = get_first_word(intro);

//     println!("{} ", first_word);

//     println!("Stop");

//     let a:i32 = 4;
//     let b:i32 = 5;

//     let sum_res: i32 = do_sum(a, b);

//     println!("Sum of {} and {} is: {}", a, b, sum_res);

//     update_string();

//     /* Ownership */

//     let my_string:String = String::from("This is my string");
//     let my_string2:i32 = 55;

//     take_ownership(my_string.clone());
//     take_ownership2(my_string2);

//     println!("{}", my_string);
//     println!("{}", my_string2);


//     /* Borrow and references */
//     let mut s1:String = String::from("Hello");
//     // let s2 = &s1; // s2 Borrows Hello for now

//     // println!("{}", s1);
//     // println!("{}", s2)

//     // borrow_variable(&s1);

//     update_borrwed_string(&mut s1);

//     println!("After borrowd: {}", s1);

//     /* Struct */
//     let user1 = User {
//         active: true,
//         name: String::from("Sandeep Lakhiwal"),
//         email: String::from("sandeeplakhiwal98@gmail.com"),
//         age: 20
//     };

//     println!("User1=> name:{:?}, email:{:?}, age:{:?}, isActive:{:?}", user1.name, user1.email, user1.age, user1.active);

//     let rect = Rect {
//         height: 30,
//         width: 45
//     };

//     println!("The area of the rectangular is {}", rect.area() );

//     /* Enums */ 

//     // move_around(Direction::North);

//     /* Enums with values */
//     let circle = Shape::Circle(5.0);
//     let square = Shape::Square(4.0);
//     let rectangle = Shape::Rectangle(3.0, 6.0);

//     let area_of_square = calculate_area(square);
//     let area_of_circle = calculate_area(circle);
//     let area_of_rectangle = calculate_area(rectangle);

//     println!("Areas of circle is {}, square is {} and rectangle is {}.", area_of_circle, area_of_square, area_of_rectangle);

//     /* Error Handling */

//     // There is a function that can error out/stop the thread
//     let res = fs::read_to_string("/example.txt");

//     match res {
//         Ok(content) => {
//             println!("File content: {}", content);
//         },
//         Err(err ) => {
//             println!("Error: {}", err);
//         }
//     }

//     let name_of_someone = String::from("Nilam");

//     match find_fisrt_a(name_of_someone) {
//         Some(index) => println!("The letter 'a' is found at index: {}", index ),
//         None => println!("The letter 'a' is not found in the string.")
//     }


//     /* Generate a random number using Rand package/crate/library */
//     let mut rng = thread_rng();
//     let n:u32 = rng.gen();
//     println!("Random number: {}", n);

//     let now = Utc::now();
//     println!("Current date and time in UTC: {}", now);

//     let local = Local::now();
//     println!("Current date and time in local: {}", local);




//     /* Vectors */
//     let mut vec = Vec::new();
//     vec.push(1);
//     vec.push(2);
//     vec.push(3);
//     vec.push(4);
//     vec.push(5);

//     println!("Vec is: {:?}", vec);

//     let got_ans = vector_with_even_values(Vec::new());

//     println!("Got Ans: {:?}", got_ans);

//     // println!("{}", is_even(50));

//     println!("{:?}", even_filter(&vec));

//     println!("{:?}", vec);

//     // initializing vector using vec macros
//     // let vec_1 = vec![9, 8, 7, 6, 5, 4];

//     // Hashmaps
//     let mut users_ages = HashMap::new();

//     users_ages.insert(String::from("Sandeep"), 20);
//     users_ages.insert(String::from("Nitin"), 22);

//     println!("{:?}", users_ages);

//     let first_user_age = users_ages.get("Sandeep");

//     match first_user_age {
//         Some(age) => println!("Age of first user is {}.", age),
//         None => println!("Given name user doesn't exists in users array.")
//     }

//     // println!("Age of Sandeep is {}", users_ages["Sandeep"]);

//     // Tuple vectors into structs
//     let tuple_vec = vec![(String::from("Sandeep"), 20), (String::from("Nitin"), 20)];

//     let mut tuple_to_hashmap_res = group_values_by_keys(tuple_vec);

//     println!("Tuple to hashmap res {:?}", tuple_to_hashmap_res);

//     /* Iterator */
//     // let iter = tuple_to_hashmap_res.iter();
//     let iter = tuple_to_hashmap_res.iter_mut();

//     for (_key, value) in iter {
//         *value = *value + 1;
//     }

//     println!("Updated TopleToHashmap: {:?}", tuple_to_hashmap_res);

//     let mut vec_iter = vec.iter_mut();

//     while let Some(val) = vec_iter.next() {
//         println!("{}", val);
//     }

//     // .into_iter => takes ownership

//     // Cunsuming iterator
//     let v1 = vec![1, 2, 3];
//     let v1_iter = v1.iter();

//     // let sum:i32 = v1_iter.sum();

//     // println!("Sum of v1 is: {}", sum);

//     // println!("{:?}", v1);

//     // Iterator adaptor
//     let v1_iter_2 = v1_iter.map(|x| x+1);

//     // for x in v1_iter_2 {
//     //     println!("😊 {}", x);
//     // }

//     let v1_iter_3 = v1_iter_2.filter(|x| *x%2 == 0);

//     for x in v1_iter_3 {
//         println!("Check: {}", x)
//     }

//     let v2 = vec![5, 6, 7];
//     let v2_iter = v2.iter().filter(|x| *x%2 == 0).map(|x| x*2);
//     let v2_vec:Vec<i32> = v2_iter.collect();
//     println!("v2_iter {:?}", v2_vec);
// }

// fn group_values_by_keys(vec: Vec<(String, i32)>) -> HashMap<String, i32> {
//     let mut hm = HashMap::new();
//     for (key, value) in vec {
//         hm.insert(key, value);
//     }
//     return hm;
// }

// fn even_filter(vec: &Vec<i32>) -> Vec<i32> {
//     let mut res  = Vec::new();
//     for item in vec {
//         if is_even(item) {
//             res.push(*item);
//         }
//     }
//     return res;
// }

// fn is_even(number: &i32) -> bool {
//     let res = number % 2 == 0;
//     return res;
// }

// fn vector_with_even_values(v: Vec<u32>) -> Vec<u32> {
//     let mut new_vector = v;
//     for item in 1..50 {
        
//         if item%2 == 0 {
//             new_vector.push(item);      
//         };
        
//     }
//     return  new_vector;
// }

// fn find_fisrt_a(s: String) -> Option<i32> {
//     for (index, character) in s.chars().enumerate() {
//         if character == 'a' {
//             return Some(index as i32);
//         }
//     }
//     return None;
// }

// fn get_first_word(sentence: String) -> String {
//     let mut ans:String = String::from("");
//     for c in sentence.chars() {
//         // println!("Char {}", c);
//         ans.push(c);
//         if c == ' ' {
//             break;
//         }
//     }
//     return  ans;
// }

// fn do_sum(a:i32, b:i32) -> i32 {
//     return  a+b;
// }

// fn update_string() {
//     let mut str1: String = String::from("Sandeep");

//     println!("Before update: {}", str1);
//     println!("Before => Capacity: {}, Length: {}, Pointer: {:p}", str1.capacity(), str1.len(), str1.as_ptr());

//     str1.push_str(" Lakhiwal");    
//     println!("After => Capacity: {}, Length: {}, Pointer: {:p}", str1.capacity(), str1.len(), str1.as_ptr());

// }

// fn take_ownership(some_string: String) {
//     println!("{}", some_string)
// }

// fn take_ownership2(some_string: i32) {
//     println!("{}", some_string)
// }

// // fn borrow_variable(some_string: &String) {
// //     println!("Borrwed in function: {}", some_string);
// // }

// fn update_borrwed_string(some_string: &mut String) {
//     some_string.push_str(" World");
// }


// // Integers
// /* 
// 1. Signed Integers = positive or negative
//     i8 = 8bit integer = _ _ _ _ _ _ _ _ => -2**7 to 2**7 - 1
//     i16
//     i32
//     i64

// 2. Unsigned Integers = only positive
//     u8
//     u16 = 16bit integer
//     u32
//     u64
// 3. Float Integers = with decimals
// */


// /* 
// Actix = Extremely fast http server
// Serde = Serializing and deserializing data in rust
// Tokio = Asynchronous runtime in rust
// reqwest = Send HTTP requests
// sqlx = Connect to sql database
// chrono = Date-time
