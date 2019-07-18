mod geometry;
mod object;
mod parser;

fn main() {
    let obj = parser::parse("./tests/cube.obj".to_string());

    println!("{:#?}", obj);
}
