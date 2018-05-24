//use std::io;

fn main() {
    println!("Hello, World!");

    let x = 5;
    let x = x + 1; // shadowing -- mut keyword not needed
    let x = x * 2; // shadowing -- mut keyword not needed

    println!("Value of x is: {}", x);

    let k = 't';

    say_hi(k);


}

fn say_hi(said: char) {

    println!("Character entered: {}", said);

}