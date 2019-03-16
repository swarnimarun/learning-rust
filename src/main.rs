
// learning Rust project - simple project with sample code to learn rust


// very simple main function

pub type TResult<T> = Result<T, std::io::Error>;

fn main() -> TResult<()> {
    // this function is supposed to do nothing
    Ok(())
}