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
//     // we must say that the what will come to the function 
//     // for example in here we want to pass a string so we will
//     // mention that the type of name is &str

//     fn printHello(name: &str){
//         println!("hi {name}")
//     }
//     printHello("arvin");
    
    
//     // we can mention that what we are going to return from function
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
//                               // it can print! so so so unique syntax :)

    



//     // code Blocks
    
//     //example
//     {
//         let a = 0;
//         let s = "string";
//         println!("{}", format!("{a} {s}"));
//         // we cannot access these variables outside of the code Block! ;)
//         // code Blocks can return values too! so Coooooool...
//         // we cannot reuse code Blocks!
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
//         'A' // do not place ';'
//     } else if num >=60 {
//         'B' // do not place ';'
//     } else {
//         'C' // do not place ';'
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
//     } // but you can go out of it by break

//     // we can Label something like:
//     'label: loop {
//         println!("now im in a loop which has label");

//         // break with label so you can get out of multiple loops with only one break!
//         break 'label;
//     }


//     // we can make a var of value with loops
//     // for example:
//     let mut b = 1;
//     let a = loop{
//         b += 1;


//         if b==5{
//             break b;
//         }
//     };
//     // now you can see that in this case when b reached 5, a value became 5!
//     // OUTPUT: a=5
//     println!("a={}", a);






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




//     // if we make a var and not use it it gives us warning, lets fix it:
//     // let mamamia = "hi";
//     // so we will use something like this
//     let _mamamia = "hi";
//     print!("{_mamamia}");




//     // static is something like a const but...
//     // if you want to store tons of data in a var
//     // static is a better choice!
//     static data: &str = "this is my data";



// }











// _______________________________________________part-2___________________________________________________

fn main(){

    // look at this great example of ref value
    let n = String::from("22");
    let b = &n; // here we can use both n.clone() or &n
    println!("n is {} and b is {}", n, b)

}