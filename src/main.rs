use std::env;
use std::io;
mod juego;
mod inicializar;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    inicializar::run(args)?;
    Ok(())
}

