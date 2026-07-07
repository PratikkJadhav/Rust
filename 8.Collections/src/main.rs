use std::collections::{HashMap, hash_map};

// fn main(){
//     let mut v: Vec<i32> = Vec::new();
//     let mut a = vec![1,2,3,4,5];

//     v.push(1);
//     v.push(1);
//     v.push(1);
//     v.push(1);
//     v.push(1);


//     // let third = a.get(1);
//     // match third {
//     //     Some(third) => print!("{third}" ),
//     //     _ => print!("not third"),
//     // }

//     // let x = &v[100];
//     // let y = v.get(100);

//     // for i in &mut a {
//     //     *i += i;
//     // }

//     enum SpreadsheetCell {
//         Int(i32),
//         Float(f64),
//         Text(String),
//     }

//     let row = vec![
//         SpreadsheetCell::Int(3),
//         SpreadsheetCell::Text(String::from("blue")),
//         SpreadsheetCell::Float(10.12),
//     ];

// }

// fn main(){
//     let mut s21 = String::new();
//     let data = "initial comment";
//     let five = data.to_string();
//     let s = String::from("initial contents");

//     let mut s = String::from("foo");
//     s.push_str("bar");

//     let s1 = String::from("Hello, ");
//     let s2 = String::from("world!");
//     let s3 = s1 + &s2;

//     for c in "Зд".chars() {
//         println!("{c}");
//     }

//     for b in "Зд".bytes() {
//         println!("{b}");
//     }

// }

fn main(){
    // let mut map = HashMap::new();

    // map.insert(String::from("Blue"), 10);
    // map.insert(String::from("Red"), 10);

    // let mut scores = HashMap::new();

    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);

    // let team_name = String::from("Blue");
    // let score = scores.get(&team_name).copied().unwrap_or(0);

    // for (key, value) in &scores {
    //     println!("{key}: {value}");
    // }

    // let mut scores = HashMap::new();
    // scores.insert(String::from("Blue"), 10);

    // scores.entry(String::from("Yellow")).or_insert(50);
    // scores.entry(String::from("Blue")).or_insert(50);

    // println!("{scores:?}");

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
}