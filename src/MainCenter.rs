const Pi: f32 = 3.14159265359;

pub trait Shapes {
    fn init(&mut self ,r: f32, p: (i32,i32), i: i32);
    fn get(&mut self);
    fn get_perimeter(&self) -> f32;
}


pub struct Circle {
    radius: f32,
    position: (i32,i32),
    id: i32,
}

impl Shapes for Circle {
    fn init(&mut self ,r: f32, p: (i32,i32), i: i32) {
            self.radius = r;
            self.position = p;
            self.id = i;
    }

    fn get(&mut self) {
        println!("radius: {} \nposition: {},{}", self.radius, self.position.0, self.position.1);
    }

    fn get_perimeter(&self) -> f32 {
        (self.radius) * 2.0 * Pi
    }
}


pub struct System {
    pub shapes: Vec<Box<dyn Shapes>>,
    pub id: i32,
}

impl<'a> System {
    pub fn init() -> Self {
        System{
            shapes: Vec::new(),
            id: 0,            }
    }

    fn system_push<T : Shapes + 'static>(&mut self, s: Box<T>) {
        self.shapes.push(s);
        self.id = self.id + 1;
    }

    pub fn init_circle(&mut self, r: f32, p: (i32,i32)) -> i32 {
        let local_c = Circle {
            radius: r,
            position:(p),
            id: self.id,     };
        self.system_push(Box::new(local_c));
        return self.id-1;
    }

    pub fn max_objects(&mut self) -> i32 {
        self.id
    }
}




