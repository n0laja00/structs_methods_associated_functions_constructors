use std::f32::consts::PI;

//the struct of Circle.
#[derive(Debug)]
struct Circle {
    circumference: f32,
    radius: f32,
    diameter: f32,
    area: f32,
}

//Implementation block
impl Circle {
    //Method: Using the reference of self to read the radius to get the right number.
    fn get_circumeference(&self) -> f32 {
        2.0 * PI * self.radius
    }
    //Method: Using the reference of self to read the radius to get diameter.
    fn get_diameter(&self) -> f32 {
        self.radius * 2.0
    }
    //Constructor: Here's a constructor to make new circles. It takes radius and uses that to construct a new circle.
    fn circle(radius: f32) -> Self {
        Self {
            circumference: 2.0 * PI * radius,
            radius,
            diameter: radius * 2.0,
            area: PI * radius.powf(2.0),
        }
    }
}

fn main() {
    // Making a new circle, circle1, by giving the constructor the radius of 2.0.
    let circle1: Circle = Circle::circle(2.0);
    println!("The circle was created! We got {:#?}", circle1);

    //We only need to call the get_circumeference() function that exists within the context of the circle1.
    //If we wanted, we could also just get the same information with "circle1.circumference" and "circle1.diameter".
    println!(
        "Calculating the circumeference of our circle gives us: {}",
        circle1.get_circumeference()
    );
    println!(
        "Calculating the diameter of our circle gives us: {}",
        circle1.get_diameter()
    );
}
