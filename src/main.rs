use std::env;
use std::io;
mod inicializar;
mod juego;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    inicializar::run(args)?;
    Ok(())
}
