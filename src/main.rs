use std::env;
use std::fs::File;
//use std::io::{self, BufRead, Write};
use std::io::{self, BufRead};
mod juego;
use juego::Juego;
use crate::juego::coordenada::Coordenada;


fn chequear_argumentos(args: &Vec<String>) -> io::Result<()> {
    if args.len() != 5 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Se esperaban exactamente 5 argumentos.",
        ));
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    match chequear_argumentos(&args) {
        Ok(_) => {
            // El resto del código sigue aquí si no hubo errores.
        }
        Err(e) => {
            return Err(e);
        }
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
    let mut juego: Juego = Juego::new();

    let mut filas: i8 = 1;
    let mut coordenada_y: i8 = 0;

    // Itera sobre las líneas del archivo
    for linea in reader.lines() {
        match linea {
            Ok(l) => {
                let palabras: Vec<&str> = l.split_whitespace().collect();
                for palabra in palabras {
                    coordenada_y = coordenada_y+1;
                    print!("posición {}: ",coordenada_y);
                    print!("{} ",palabra); 
                    let punto = Coordenada::new(filas, coordenada_y);
                    if palabra == "W" {
                        juego.inicializar_pared(punto);
                    }
                    else if palabra.starts_with("S") {
                        juego.inicializar_bomba(punto);
                    }
                    else if palabra.starts_with("B") {
                        juego.inicializar_bomba(punto);
                    }  
                    else if palabra.starts_with("F") {
                        juego.inicializar_enemigo(punto);
                    }
                }
                println!();
                println!("Linea {} leída!",filas);
                filas = filas+1;
                coordenada_y = 0;
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

