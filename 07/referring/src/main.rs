pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

use a::series::of;


enum TrafficLight {
    Red,
    Yellow,
    Green,
}

use TrafficLight::*;



fn main() {
    let red = Red;
    let green = TrafficLight::Green;
    of::nested_modules();
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
