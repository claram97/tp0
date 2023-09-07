use std::env;
use std::fs::File;
//use std::io::{self, BufRead, Write};
use std::io::{self, BufRead};
mod juego;
use juego::Juego;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Se esperaban exactamente 5 argumentos.",
        ));
    }

    let output_file_name: &String = &args[2];
    let mut _output_file = File::create(output_file_name)?;
    /*
    let contenido = "Hola, este es un ejemplo de escritura en un archivo desde Rust.\n";

    output_file.write_all(contenido.as_bytes())?;
 */
    let maze_file_name: &String = &args[1];
    let maze_file = File::open(maze_file_name)?;
    let reader = io::BufReader::new(maze_file);
    let mut juego: Juego;

    let mut filas: i8 = 1;
    // Itera sobre las líneas del archivo
    for linea in reader.lines() {
        match linea {
            Ok(l) => {
                let palabras: Vec<&str> = l.split_whitespace().collect();
                for palabra in palabras {
                    print!("{} ",palabra);
                }
                println!();
                println!("Linea {} leída!",filas);
                filas = filas+1;
            }
            Err(e) => {
                eprintln!("Error al leer línea: {}", e);
            }
        }
    }

    filas = filas-1;
    println!("Hay {} filas",filas);

    Ok(())   
}
/*     let output_dir = &args[3];
    let x = &args[4];
    let y = &args[5];

    // Ahora puedes utilizar las variables `maze_file`, `output_dir`, `x` y `y` en tu programa
    println!("Laberinto: {}", maze_file);
    println!("Directorio de salida: {}", output_dir);
    println!("X: {}", x);
    println!("Y: {}", y);
 */

