struct Temperature {
    degrees_f: f64,
}

impl Temperature {

    fn freezing() -> Self {
        Self { degrees_f: 32.0 }
    }

    // borrowing
    fn show_temp(&self){
        println!("{:?} degrees F", self.degrees_f);
    }

}

enum Color {
    RED,
    BLUE,
    GRAY
}

impl Color {
    fn print(&self) {
        match self {
            Color::RED => println!("Red"),
            Color::BLUE => println!("Blue"),
            Color::GRAY => println!("Bray"),
        }
    }
}

struct Dimensions {
    length: i32,
    height: i32,
    width: i32,
}

impl Dimensions {
    fn print(&self) {
        println!("length: {:?}, height: {:?}, width: {:?}", self.length, self.height, self.width);
    }
}

struct Box {
    weight: i32,
    dimension: Dimensions,
    color: Color,
}

impl Box {
    fn new(weight: i32, color: Color, dimension: Dimensions) -> Self {
        Self {
            weight,
            color,
            dimension,
        }
    }

    fn print(&self) {
        self.color.print();
        self.dimension.print();
    }
}


fn main() {
    let hot = Temperature { degrees_f: 99.9};
    hot.show_temp();

    let cold = Temperature::freezing();
    cold.show_temp();

    let small_dimensions = Dimensions {
        width: 1,
        height: 5,
        length: 3,
    };

    let small_box = Box::new(5, Color::RED, small_dimensions);
    small_box.print();

}