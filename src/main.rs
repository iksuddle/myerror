use myerror_derive::MyError;
use std::num::ParseIntError;

#[derive(MyError)]
enum FooError {
    IO(std::io::Error),
    Parse(ParseIntError),
}

fn do_something() -> Result<(), FooError> {
    let num: u32 = "123".parse()?;
    println!("{num}");

    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf)?;

    Ok(())
}

fn main() {}
