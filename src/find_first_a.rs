pub fn find(){


let index = find_first_a(String::from("Arghya"));

match index{ 
Some(value)=> println!("index is {}" ,value),
None=> println!("not found"),
    }


pub fn find_first_a(s:String) -> Option<i32>{



for (index, char) in s.chars().enumerate(){

        if char == 'a' ||char ==  'A'{

            return  Some(index as i);
        }

    }
    return None;

}
}

//
//pub fn find(){
//
//enum CustomOption {
        //Some(i32),
        //None,
    //}
//
//let index = find_first_a(String::from("rghya"));
//
//match index{ 
        //CustomOption::Some(value)=> println!("index is {}" ,value),
        //CustomOption::None=> println!("not found"),
    //}
//
//
//pub fn find_first_a(s:String) -> CustomOption{
//
//
//
//for (index, char) in s.chars().enumerate(){
//
        //if char == 'a' ||char ==  'A'{
//
            //return CustomOption::Some(index as i32);
        //}
//
    //}
    //return CustomOption::None;
//
//}
//}

