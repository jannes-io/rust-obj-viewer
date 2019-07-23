mod geometry;
mod object;
mod parser;
mod viewer;

fn main() -> Result<(), String> {
    let obj = parser::parse("./tests/teapot.obj".to_string());
    let mut viewer = viewer::Viewer::new(obj)?;
    viewer.run();

    Ok(())
}
