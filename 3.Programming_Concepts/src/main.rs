// fn main() {
//     let mut x: u32= 5;
//     x = x + 2;
//     println!("Hello, world! from {x}");
// }

// fn main() {
//     let number = 3;

//     if number < 5 {
//         println!("condition was true");
//     } else {
//         println!("condition was false");
//     }
// }

// fn main() {
//     let condition = false;
//     let number = if condition { 5 } else { 6 };

//     println!("The value of number is: {number}");
// }

// fn main(){
//     let mut n= 3;

//     let x = loop {
//         n = n + 1;

//         if n == 5 {
//             break n*2;
//         }
//     };

//     println!("{x}");
// }

// fn main() {
//     let mut number = 3;

//     while number != 0 {
//         println!("{number}!");

//         number -= 1;
//     }

//     println!("LIFTOFF!!!");
// }

fn main(){
    // let a = [10, 20, 30, 40, 50];

   for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}