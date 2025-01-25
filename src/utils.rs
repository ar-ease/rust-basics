use crate::greetings::say_hello; // Access from the root module

pub fn use_say_hello(name: &str) {
    let greeting = say_hello(name);
    println!("{}", greeting);
}
