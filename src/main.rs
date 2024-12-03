mod greetings;
use greetings::{say_hello , Person};

#[allow(dead_code)]
   struct User {
    active : Option<bool>,
    username:String,
    email:Option<String>,
    sign_in_count: Option<u64>,
    }
fn main() {
       let user1 = User {
           active: Some(true),
           username : String::from("myusername"),
           email: Some(String::from("arease@gmail.com")),
           sign_in_count: Some(1),
       };

    let x: String = String::from("hii there ");
    println!("{}", x);
    print!("User 1 username is : {:?}" , user1.username);

    println!("{}", say_hello("Alice"));

    let person = Person {
    name : String::from("Bob"),
    age: 25 
    };
    println!("{:#?}", person);
}
