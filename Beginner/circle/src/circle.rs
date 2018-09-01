use std::f64::consts::PI;

#[derive(Debug)]
pub struct Circle {
    pub radius: f64,
    pub diameter: f64,
    pub area: f64,
}

impl Circle {
    pub fn new() -> Self {
        Circle {
            radius: 0.0,
            diameter: 0.0,
            area: 0.0,
        }
    }

    fn calculate_area(&mut self) {
        self.area = self.radius * self.radius * PI;
    }

    fn calculate_diameter(&mut self) {
        self.diameter = self.radius * 2 as f64;
    }

    fn calculate_radius(&mut self) {
        self.radius = self.diameter / 2 as f64;
    }

    fn calculate_radius_from_area(&mut self) {
        let inter = self.area / PI;
        self.radius = inter.powf(0.5);
    }
}

pub fn circle_radius(rad: f64) -> Circle {
    let mut circle = Circle::new();
    circle.radius = rad;
    circle.calculate_area();
    circle.calculate_diameter();
    circle
}

pub fn circle_diameter(diam: f64) -> Circle {
    let mut circle = Circle::new();
    circle.diameter = diam;
    circle.calculate_radius();
    circle.calculate_area();
    circle
}

pub fn circle_area(ar: f64) -> Circle {
    let mut circle = Circle::new();
    circle.area = ar;
    circle.calculate_radius_from_area();
    circle.calculate_diameter();
    circle
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let mut circle = Circle::new();
        circle.radius = 1.0;
        circle.calculate_diameter();
        circle.calculate_area();
        assert_eq!(2.0, circle.diameter);
        assert_eq!(PI, circle.area);
    }

    #[test]
    fn test_two() {
        let mut circle = Circle::new();
        circle.area = PI;
        circle.calculate_radius_from_area();
        circle.calculate_diameter();
        assert_eq!(1.0, circle.radius);
        assert_eq!(2.0, circle.diameter);
    }
}
