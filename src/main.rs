pub mod MainCenter;
use MainCenter::System;






fn main() {

let mut Central = System::init();
Central.init_circle(5.0,(100,100));
Central.init_circle(10.0,(100,100));


println!("{}", Central.max_objects());

Central.shapes[0].get();
println!("{}", Central.shapes[0].get_perimeter());
println!("{}", Central.shapes[1].get_perimeter());

}


