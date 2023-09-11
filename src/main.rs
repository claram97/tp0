use std::env;
use std::io;
use tp0::inicializar;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    inicializar::run(args)?;
    Ok(())
}