pub fn structs() {

struct Rect { 
width : i32,
height : i32
}

impl Rect {

fn area(&self) -> i32{
return self.width * self.height;
 }
fn parimeter(&self , _num: i32)-> i32{
return 2 * (self.width + self.height);
    }

        fn debug() -> i32{

            return 1;
        }
}


let rect1  = Rect{ 
width : 10,
    height:20


};

println!("area is {}" , rect1.area());
println!("parimeter is {}" , rect1.parimeter(6));
println!("{}", Rect::debug());
}
