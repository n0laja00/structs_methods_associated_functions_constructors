use std::f32::consts::PI;
#[derive(Debug)]
struct Circle {
    circumference: f32,
    radius: f32,
    diameter: f32,
    area: f32,
}

impl Circle {
    fn get_circumeference(&self) -> f32 {
        2.0 * PI * self.radius
    }
    fn get_diameter(&self) -> f32 {
        self.radius * 2.0
    }
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
    let circle1 = Circle::circle(2.0);
    println!("The circle was created! We got {:#?}", circle1);

    println!(
        "Calculating the circumeference of our circle gives us: {}",
        circle1.get_circumeference()
    );
    println!(
        "Calculating the diameter of our circle gives us: {}",
        circle1.get_diameter()
    );
}
