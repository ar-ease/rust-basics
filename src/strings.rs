pub fn strings(){


let name = String::from("Arghya Jana");
    let len = get_str_len(name);

    println!("the length of the string is {}" , len);

fn get_str_len(str: String) -> usize{

println!("{}"  , str);
    str.chars().count()
    }


}
