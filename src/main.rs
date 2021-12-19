

const Pi: f32 = 3.14159265359;

trait Shapes {
    fn init(&mut self ,r: f32, p: (i32,i32));
    fn get(&mut self);
    fn get_perimeter(&self) -> f32;
}

struct Circle {
    radius: f32,
    position: (i32,i32),
}

impl Shapes for Circle {

    fn init(&mut self ,r: f32, p: (i32,i32) ) {
            self.radius = r;
            self.position = p;
    }


    fn get(&mut self) {
        println!("radius: {} \nposition: {},{}", self.radius, self.position.0, self.position.1);
    }


    fn get_perimeter(&self) -> f32 {
        (self.radius) * 2.0 * Pi
    }
}



struct System {
    pub shapes: Vec<Box<dyn Shapes>>,
}


impl<'a> System {
    fn init() -> Self {
        System{
            shapes: Vec::new(),
        }
    }

    fn system_push<T : Shapes + 'static>(&mut self, s: Box<T>) {
        self.shapes.push(s);
    }
}



fn main() {

let mut Central = System::init();
let mut Object = Circle{radius:5.0,position:(250,100)};
Central.system_push(Box::new(Object));

Central.shapes[0].get();
println!("{}", Central.shapes[0].get_perimeter());
}


