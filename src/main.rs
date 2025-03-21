use std::io;

// Structs

// struct User {
//     name: String,
//     age: u32,
//     email: String,
//     active: bool
// }

fn main() {
    println!("Hello, Aditya!");
    let mut input1 = String::new();
    let mut input2 = String::new();
    
    println!("Enter the first number:");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    
    println!("Enter the second number:");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    
    let num1: f64 = input1.trim().parse().expect("Please enter a valid number");
    let num2: f64 = input2.trim().parse().expect("Please enter a valid number");
    
    let sum = num1 + num2;
    println!("The sum is: {}", sum);

    // let user = User {
    //     name: String::from("Aditya"),
    //     age: 22,
    //     email: String::from("aditya@av.com"),
    //     active: true
    // };

    // println!("Name: {}", user.name);
    // println!("Email: {}", user.email);
    // println!("Age: {}", user.age);
    // println!("Active Status: {}", user.active);

    // Signed & Unsigned Ints

    // let x: i8 = -4;
    // let x1 = -4;
    // let y: u8 = 101;
    // // by default types of +ve ints are given i32. 
    // // if it is a positive number and u know it surely, make it u8 or whaterver.
    // let y2 = 101;
    // let z: f32 = -95.5;

    // println!("x: {},",x);
    // println!("x1: {},",x1);
    // println!("y: {},",y);
    // println!("y2: {},",y2);
    // println!("z: {},",z);

    // Muts & Loops

    // let mut x: i32 = 0;
    
    // for _i in 0..100000 {
    //    x =  x +  10000;
    // }

    // println!("x: {}", x)

    // Booleans

    // let male = true;
    // let age_18  = true;

    // if male {
    //     println!("He is a male")
    // }else{
    //     println!("You are not a male ")
    // }

    // if male && age_18 {
    //     println!("He is a male having an age of 18")
    // }else {
    //     println!("There is some issue")
    // }

    // Strings, Patter Matching, Unwrap, Option<char>

    // let greeting = String::from("Hello Sir");
    // println!("{}", greeting);

    // let char1 = greeting.chars().nth(100);

    // Gives Build error
    // println!("{}", char1);

    // Gives runtime error if not exist
    // println!("char1: {}", char1.unwrap())

    // Checks & gives either ans or simply print a normal statement. No errors posted.
    // match  char1 {
    //     Some(c) => println!("{}", c),
    // None => println!("No character at that index "),
    // }

    // Conditionals
    
    // let is_even = false;

    // if is_even {
    //     println!("The number is even")
    // } else {
    //     println!("The number is odd")
    // }

    // Loops

    // Note -  With just print you will get a %, it means the program has ended and returned the control to the shell.
    
    // for i in 0..10{
    //     // print!("{}\n", i)
    //     // print!("{}", i) -> gives %
    //     println!("{}", i)
    // }

    // let sentence = String::from("hello world, i hope everyone is doing well.");
    // let first_word = get_first_word(sentence);
    // if first_word == ""{
    //     println!("There are initial spaces")
    // } else {
    //     println!("This is the first word of the sentence: {}", first_word)
    // }

    // let result = do_sum(3, 4);
    // println!("Total Result is: {}", result);

    // Pattern Matching Prac

    // let x: i8 = 44;
    
    // match x {
    //     1 => println!("Value is 1"),
    //     2 => println!("It is 2"),
    //     _ => println!("Something is fishy"),    
    //     // Default case is important as it helps in covering all the edge cases for the pattern matching. Basically you need to return something or else it gives an error. so _ is important.      
    // }

    // Stack & Heap Chapter
        // stack_fn();   // Call the function that uses stack memory
        // heap_fn();    // Call the function that uses heap memory
        // update_string();  // Call the function that changes size of variable at runtime
    
    // Ownerships - Clones

    // let s1 = String::from("Hello");
    // let s2 = s1.clone();
    // println!("{} S1, {:p}", s1, s1.as_ptr());
    // println!("{} S2, {:p}", s2, s2.as_ptr());

    // let real_string = String::from("Hello World");
    // let mystring3 = take_ownership(real_string);
    // // This below line may give an error as heap value of real string has been passed to mystring.
    // println!("{} from mystring3", mystring3);

    //simple string init in main's stack frame -> no mut needed
    // let real = String::from("Hello");
    // // fn calling & transferring address of the heap -> temp purpose 
    // borrow_variable(&real);
    // // above one is compiled so now we got our heap value back -> print it
    // println!("{} from real", real);
    // // same as above -> both
    // let s = &real;
    // println!("{} from s", s);

    // References

    // Mutable Reference
    // let mut s1 = String::from("Hello");
    // // Passing a mutable reference
    // update_word(&mut s1);
    // println!("{}", s1)

    // learn example
    // let mut s1 = String::from("Hello");
    // let s2 = &mut s1;
    // s2.push_str(" World");
    // // Gives error below to save memory issues
    // // println!("{}", s1);
    // println!("{}", s2);


}

// fn update_word(s: &mut String){
//     s.push_str(" World");
// }

// accept string & send string back to main stack frame
// fn borrow_variable(mystring: &String) -> &String{
//     // simple print
//     println!("{} from &String", mystring);
//     // crashing the pointer
//     return mystring;
// }

// fn stack_fn() {
//     // Declare a few integers on the stack
//     let a = 10;
//     let b = 20;
//     let c = a + b;
//     println!("Stack function: The sum of {} and {} is {}", a, b, c);
// }

// fn heap_fn() {
//     // Create a string, which is allocated on the heap
//     let s1 = String::from("Hello");
//     let s2 = String::from("World");
//     let combined = format!("{} {}", s1, s2);
//     println!("Heap function: Combined string is '{}'", combined);
// }

// fn update_string() {
//     // Start with a base string on the heap
//     let mut s = String::from("Initial string");
//     println!("Before update: {}", s);
//     println!("Capacity: {}, Length: {}, Pointer: {:p}", s.capacity(), s.len(), s.as_ptr());
    
//     // Append some text to the string
//    for _ in 0..100{
//     s.push_str(" and some additional text \n");
//     // println!("After update: {}", s);
//     println!("Capacity: {}, Length: {}, Pointer: {:p}", s.capacity(), s.len(), s.as_ptr());
//    }
// }


// Functions 
// fn do_sum(a: i32, b: i32) -> i32 {
// 	return a + b;
// }


// Function - To search the first word in the sentence
// creating function which takes sentence as input of type String & also returns a String
// fn get_first_word(sentence:  String) -> String{
//     // having a variable ans which is mutable used to store the characets after looping
//     let mut ans = String::from("");
//     // simple for loop on the input take (sentence)
//     for char in sentence.chars(){
//         // checking if there are initial spaces, if yes, we break
//         if char == ' '{
//             break;
//         }
//         // if no spaces, we are converting char to string & back & push it to the variable {ans}. -> My understanding.``
//         // Converts the char to a String and appends it to the variable `ans`. -> GPT Says
//         ans.push_str(char.to_string().as_str());
//     }
//     // returning the ans whatever output we get.
//     return  ans;
// }