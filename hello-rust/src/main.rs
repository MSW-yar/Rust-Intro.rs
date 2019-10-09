/** fn main() {
*   println!("Hello, world!");
* }
*/

//use ferris_says:: say;
// this tells that we can use say that ferris_say is providing.

//use std::io::{stdout, BufWriter};

// fn main() {
//     let stdout = stdout();
//     let out = b"Hello baby";
//     let width = 14;
//     let mut writer = BufWriter::new(stdout.lock());
//     say(out, width, &mut writer).unwrap();
// }

use std::io; 
/**
    to bring an input library for the user, only ::io is involved.
    it will generate the output according to it.    
*/

fn main() { //entry point to rust is fn main(){}

    println!("Guess a number");

    println!("Enter a number");

    let mut guess = String :: new(); //mut makes the value mutable, oe it binds and immutate it.
    // ::new() indicates that new is a fn asssociated with String Type.
    // its actually a mutable var that is currently bound to a new, empty instance of String.
    
    io :: stdin().read_line(&mut guess)
        .expect ("failed to guess");

/**
     this line 35 have a fn that returns an instance which is handle to the std io of terminal.
     readline is method on std io to get inp from user/
     purpose is to take whatever the user types into std::io and place in string.
     its taking string as an argument. & indicates tthe reference. let multiple part of code
     to access the one piece of data. mut indicates the mutable guess.
     
*/
    println!("you guessed {}", guess);
}