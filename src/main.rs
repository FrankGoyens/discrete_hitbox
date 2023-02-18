mod foundation;
mod shape_factory;
mod linear_algebra;

fn main() {
    let rect = shape_factory::make_rectangle(100., 200.);
    println!("{:?}", rect);
    let rect_lines = shape_factory::expand_lines(&rect.into_iter().collect());
    println!("{:?}", rect_lines);
}
