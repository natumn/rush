use std::env;
use std::io;
use std::error::Error;

fn main() {
    ::std::process::exit(match run_loop() {
        Ok(_) => 0,
        Err(err) => {
            writeln!(io::stderr(), "error: {:?}", err).unwrap();
            1
        }
    });
}

fn run_loop() -> Result<(), ()> {
    let args = env::args().collect();
    let mut status = false;

    while status {
        print!("> ");
    }

    Ok(())
}
