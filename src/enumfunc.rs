pub fn enumfunc(){



    enum Shape{
        Rectangle(f64, f64),
        Circle(f64)
    }

    let rect = Shape::Rectangle(1.0 , 2.0);
    let x = calculate_area(rect);

    println!("{}", x);
    let circle = Shape::Circle(1.0);
    let y = calculate_area(circle);
    println!("{}", y);

    fn calculate_area(shape : Shape) -> f64{
        let area = match shape {

            Shape::Rectangle(a,b)=>a*b,
            Shape::Circle(r)=>3.14*r*r,

        };
        return area;
    }



}
