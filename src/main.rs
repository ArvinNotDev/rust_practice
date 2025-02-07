// fn main() {
//     println!("Hello, world!");
//     let mut x = 10;
//     x += 1;
//     println!("{x}");
//     let a = 25;
//     let mut b = a as f64;
//     b *= 2.5;
//     println!("a is {a} and b is {b}");



//     // strings
//     let mut myString = "this is a &str";
//     myString = "a"; // we can change it but we cannot add anything
//     println!("{myString}");
    
//     let mut newString = String::from("this is a String::from string!");
//     newString += "(something new)";
//     println!("{newString}");


//     // arrays (it stores in the stack)

//     let myArray = [1,2,3,4,5]; //all types must be the same
//     let BeautifulArray = [0; 7]; //OUTPUT: [0,0,0,0,0,0,0]

//     // vector (arrays but it stores in the heap)

//     let myVec = vec![1,2,3,4,5,];


//     // tuples

//     let myTuple = ("hi", "my", "age", "is", 20); // tuples can include different data types

    


//     // functions

//     // functions can be inside or outside of the main function
//     // we must say that what types of the variables will come to the function 
//     // for example in this case we want to pass a string so we will
//     // mention that the type of name is &str

//     fn printHello(name: &str){
//         println!("hi {name}")
//     }
//     printHello("arvin");
    
    
//     // we can mention what we will return from function
//     // for example:
//     fn sum(num1: i32, num2: i32) -> i32 {
//         num1 + num2 // now i dont have to place a ";" cause if i do
//                     // it wont return it! 
//                     // if you wont return anything you cant mention
//                     // that what you are going to return...
//         // but if you want to you can use: return something;
//     }
//     println!("2+2={}", sum(2,2)); // OUTPUT: 2+2=4

//     // functions can return multiple things too
//     // for example:
//     fn returning_tuple(num1: i32, num2: i32, num3: i32) -> (i32, i32, i32){
//         return (num1, num2, num3);
//     }

//     let result = returning_tuple(1, 2, 3);
//     println!("{:?}", result); // you cannot print a tuple or array or vector like normal
//                               // remember this syntax, it will format them to something
//                               // it can print! what a unique syntax :)

    



//     // code Blocks
    
//     //example
//     {
//         let a = 0;
//         let s = "string";
//         println!("{}", format!("{a} {s}"));
//         // we cannot access these variables outside of the code Block! ;)
//         // code Blocks can return values too! so Coooooool...
//         // * => we cannot reuse code Blocks!
//         // so code Blocks and functions are the same but in code Blocks you cannot do *
//     }





//     // conditionals

//     let num = 25;
//     let bool = true;

//     if bool{
//         println!("hi");
//     } else {
//         println!("bye");
//     }
    
//     if num == 25 {
//         println!("num is 25! ");
//     } else if num == 0{
//         println!("num is 0!");
//     } else {
//         println!("num is not 25!");
//     }


//     // intresting syntax for one line if else variables!
//     let num = 80;
//     let grade = if num >= 90 {
//         'A'                     // do not place ';'
//     } else if num >= 60 {
//         'B'                     // do not place ';'
//     } else {
//         'C'                     // do not place ';'
//     };



//     // using match for the same case!
//     let mut grade = 'A';
//     match num{
//         90..=100 => grade = 'A',
//         60..=89 => grade = 'B',
//         _ => grade = 'C' // dont place ';' and '_ =>' is named arm and
//                          // its neccessary to put that in the end!
//     }






// //-------------------------------------------------------------------------
    
//     // xxXloopsXxx // ______i love looooooooops______





//     // infinite loop
//     loop {
//         println!("im in the loop");
//         break;
//     } // its an infinite loop but you can go out of it by break

//     // we can Label(naming after it) something like:
//     'label: loop {
//         println!("now im in a loop which has label");

//         // break with label so you can get out of multiple loops with only one break!
//         break 'label;
//     }


//     // we can fill a defined variable using loops
//     // for example:
//     let mut b = 1;
//     let a = loop{
//         b += 1;


//         if b==5{
//             break b;
//         }
//     };
//     // now you can see that in this case when b reached 5, "a" value became 5!
//     println!("a={}", a); // OUTPUT: a=5






//     // for loops
//     let myVec = vec![1,2,3,4,5,6,7];
//     for i in myVec{
//         println!("{}", i);
//     };



//     // while loops
//     let mut num = 10;
//     while num > 0{
//         num -= 1;
//         println!("{}", num);
//     }




//     // getting string input
//     let mut n = String::new();
//     std::io::stdin().read_line(&mut n).expect("failed to get input!..."); // you can now print it but lets see how to cast it...
//     let n: i64 = n.trim().parse().expect("didn't make it!..."); // casted it to integer
//     print!("{n}");




//     // if we define a variable but do not using it, compiler gives us warning, lets fix it:
//     // let mamamia = "hi";
//     // so we will use something like this
//     let _mamamia = "hi";
//     // now the warning is gone!



//     // static is something like a constant variables but...
//     // if you want to store tons of data in a var
//     // static is a better choice!
//     static data: &str = "this is my data and i wont change it...";



// }



// _______________________________________________part-2___________________________________________________

// fn main(){

//     // look at this great example of ref value
//     let n = String::from("22");
//     let b = &n; // here we can use both n.clone() or &n
//     println!("n is {} and b is {}", n, b)

// }
// __________________________________________________________________






// xX Object oriented programming using rust Xx


// in this case we are using "f64" type and remember that you cannot use refrence types like &str
// with same syntax cause then you have to make sure to specify a lifetime like this <'a> 
// if you want to use string its better to use "String" instead of %str, String is an owned type :) 


// example with f64 for a BankAccount class:

// pub struct BankAccount { // in rust we are making instances using structs
//     balance: f64, // Private field
// }

// impl BankAccount {
//     // Public constructor, same as def __init__(): in python
//     pub fn new(initial_balance: f64) -> Self {
//         BankAccount { balance: initial_balance }
//     }

//     // Public method to deposit money
//     pub fn deposit(&mut self, amount: f64) {
//         self.balance += amount;
//     }

//     // Public method to get balance
//     pub fn get_balance(&self) -> f64 {
//         self.balance
//     }
// }

// fn main() {
//     let mut account = BankAccount::new(100.0);
//     account.deposit(50.0);
//     println!("Balance: {}", account.get_balance()); // Output: Balance: 150.0
// }

// \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\


// Example with String for Person class:

// pub struct Person {
//     first_name: String,
//     last_name: String,
//     age: i32
// }

// impl Person {
    
//     pub fn new(initial_first_name: String, initial_last_name: String, initial_age: i32) -> Self{
//         Person {first_name: initial_first_name, last_name:initial_last_name, age: initial_age}
//     }

//     pub fn get_full_name(&self) -> String{
//         format!("{} {}", self.first_name, self.last_name)
//     }

//     pub fn get_age(&self) -> i32 {
//         self.age
//     }
// }

// fn main(){
//     let p1 = Person{first_name: String::from("Arvin"), last_name: String::from("Jafari"), age: 20};
//     println!("{}", p1.get_full_name());
//     println!("Age: {}", p1.get_age())
// }




// // In this case, we are using `&str` (string slices) instead of `String`.
// // Since `&str` is a reference type, we need to specify lifetimes to ensure the references are valid.

// // Define the `Person` struct with a lifetime parameter `'a`.
// // This tells the compiler that the references (`first_name` and `last_name`) must live at least as long as the struct.
// pub struct Person<'a> {
//     first_name: &'a str, // Reference type so we need to set a lifetime
//     last_name: &'a str,  // Reference type so we need to set a lifetime
//     age: i32,            // Owned type, no lifetime needed
// }

// // The lifetime `'a` must be specified here as well to match the struct definition.
// impl<'a> Person<'a> {

//     pub fn new(initial_first_name: &'a str, initial_last_name: &'a str, initial_age: i32) -> Self {
//         Person {
//             first_name: initial_first_name,
//             last_name: initial_last_name,
//             age: initial_age,
//         }
//     }

//     // now after initialing them thats same as before...

//     pub fn get_full_name(&self) -> String {
//         format!("{} {}", self.first_name, self.last_name)
//     }

//     pub fn get_age(&self) -> i32 {
//         self.age
//     }
// }

// fn main() {
//     // Create a `Person` instance using string literals (which have the `'static` lifetime).
//     let p1 = Person::new("Arvin", "Jafari", 20);

//     // Print the full name and age.
//     println!("Full Name: {}", p1.get_full_name()); // Output: Full Name: Arvin Jafari
//     println!("Age: {}", p1.get_age());            // Output: Age: 20
// }








// xX Enums Xx  O_____here_is_how_you_can_define_a_new_type_____O


#[derive(PartialEq)] // Derive PartialEq for easy comparison (its neccessary for comparing them like what we did)
enum Direction {
    Right,
    Left,
}

fn is_right(dir: &Direction) -> bool {
    *dir == Direction::Right
}

fn is_left(dir: &Direction) -> bool {
    *dir == Direction::Left
}

fn main() {
    let dir = Direction::Right;
    let right = is_right(&dir);
    let left = is_left(&dir);
    println!("Is the direction right? {}", right); // true
    println!("Is the direction left? {}", left);   // false
}






