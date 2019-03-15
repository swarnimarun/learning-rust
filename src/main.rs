
mod test; // to use the file test.rs

fn main() -> Result<(), std::io::Error> {
    test::do_something();
    Ok(())
}