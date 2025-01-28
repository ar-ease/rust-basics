use std::fs::read_to_string;

pub fn result_enum() {


    let result = read_to_string("hello.txt");

    match result {
        Ok(data) => println!("the data is : {}", data),
        Err(err) => println!("Error while reading the file: {}", err),
    }
}
