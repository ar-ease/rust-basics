
pub fn fib(num: i32) -> i32  {

let _hello = 0

let mut first = 0;
let mut second = 1;

if 0 == num {

        return 0;
    }
if 1 == num {

        return 1;
    }
    for _i in 2..=num{

       let  temp = second; 
    second = first + second;
    first = temp;
    }

return second;
}
