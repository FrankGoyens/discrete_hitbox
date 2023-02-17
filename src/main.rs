mod foundation;
mod shape_factory;

fn main() {
    let rect = shape_factory::make_rectangle(100., 200.);
    println!("Hello, world!");
}
