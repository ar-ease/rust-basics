pub fn say_hello(name : &str)->String{
format!("hello, {}!", name)
}
#[derive(Debug)]

pub struct Person {
    pub name : String ,
    pub age : u32
}
