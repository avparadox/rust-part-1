fn main() {
    println!("Hello, Aditya!");

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

    let result = do_sum(3, 4);
    println!("Total Result is: {}", result);
}


// Functions 
fn do_sum(a: i32, b: i32) -> i32 {
	return a + b;
}


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