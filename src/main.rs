// Setup first time: https://www.rust-lang.org/learn/get-started
// https://stevedonovan.github.io/rust-gentle-intro/1-basics.html
// $cargo run // in the same folder
use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    // println!("Hello, world!");
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();

    let answer = 42; // immutable
    println!("Hello {}", answer);

    for i in 0..5 {
        println!("Hello {}", i);
    }

    let mut sum = 0; // mutable
    for i in 0..5 {
        sum += i;
    }
    println!("sum is {}", sum);

 
    // Function
    let res = sqr(2.0);
    println!("square is {}", res);

    // By reference
    let i = 10;
    let res1 = by_ref(&i);
    let res2 = by_ref(&41);
    println!("{} {}", res1,res2);
 
    // Modify one of the arguments
    let mut res = 0.0;
    modifies(&mut res);
    println!("res is {}", res);

    // Arrays
    let arr = [10, 20, 30, 40];
    let first = arr[0];
    println!("first {}", first);

    for i in 0..4 {
        println!("[{}] = {}", i,arr[i]);
    }
    println!("length {}", arr.len());

}

fn sqr(x: f64) -> f64 {
    return x * x;
}

// By reference
// A reference is created by & and dereferenced by *.
fn by_ref(x: &i32) -> i32{
    *x + 1
}

// If you want a function to modify one of its arguments? Enter mutable references:
fn modifies(x: &mut f64) {
    *x = 1.0;
}

