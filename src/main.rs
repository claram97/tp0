use std::env;
use std::fs::File;
//use std::io::{self, BufRead, Write};
use std::io::{self, BufRead};
mod juego;
use juego::Juego;
use crate::juego::coordenada::Coordenada;

const DESVIO: char = 'D';
const ENEMIGO: char = 'F';
const BOMBA_DE_TRANSPASO: char = 'D';
const BOMBA_NORMAL: char = 'F';
const PARED: &str = "W";
const ROCA: &str = "R";


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
    let mut coordenada_y: i8 = 1;

    // Itera sobre las líneas del archivo
    for linea in reader.lines() {
        match linea {
            Ok(l) => {
                let palabras: Vec<&str> = l.split_whitespace().collect();
                for palabra in palabras {
                    //print!("posición {}: ",coordenada_y);
                    //print!("{} ",palabra); 
                    let punto = Coordenada::new(filas, coordenada_y);
                    if palabra == PARED {
                        juego.inicializar_pared(punto);
                    }
                    else if palabra == ROCA {
                        juego.inicializar_roca(punto);
                    }
                    else if palabra.starts_with(BOMBA_DE_TRANSPASO) {
                        if let Some(segundo_caracter) = palabra.chars().nth(1) {
                            if let Some(digito) = segundo_caracter.to_digit(10) {
                                let alcance = digito as i8;
                                juego.inicializar_bomba(punto,alcance,juego::bomba::TipoDeBomba::Traspaso);
                            }
                            else {
                                return Err(io::Error::new(
                                    io::ErrorKind::InvalidData,
                                    "Error al intentar inicializar la bomba con el número de alcance dado.",
                                ));
                            }
                        }
                        else {
                            return Err(io::Error::new(
                                io::ErrorKind::InvalidInput,
                                "No se pudo determinar el alcance de la bomba.",
                            ));
                        }
                    }
                    else if palabra.starts_with(BOMBA_NORMAL) {
                        if let Some(segundo_caracter) = palabra.chars().nth(1) {
                            if let Some(digito) = segundo_caracter.to_digit(10) {
                                let alcance = digito as i8;
                                juego.inicializar_bomba(punto,alcance,juego::bomba::TipoDeBomba::Normal);
                            }
                            else {
                                return Err(io::Error::new(
                                    io::ErrorKind::InvalidData,
                                    "Error al intentar inicializar la bomba con el número de alcance dado.",
                                ));
                            }
                        }
                        else {
                            return Err(io::Error::new(
                                io::ErrorKind::InvalidInput,
                                "No se pudo determinar el alcance de la bomba.",
                            ));
                        }
                    }  
                    else if palabra.starts_with(ENEMIGO) {
                        if let Some(segundo_caracter) = palabra.chars().nth(1) {
                            if let Some(digito) = segundo_caracter.to_digit(10) {
                                let vida = digito as i8;
                                juego.inicializar_enemigo(punto,vida);
                            }
                            else {
                                return Err(io::Error::new(
                                    io::ErrorKind::InvalidData,
                                    "Error al intentar inicializar el enemigo con el puntaje de vida dado.",
                                ));
                            }
                        }
                        else {
                            return Err(io::Error::new(
                                io::ErrorKind::InvalidInput,
                                "No se pudo determinar la vida del enemigo.",
                            ));
                        }
                    }
                    else if palabra.starts_with(DESVIO) {
                        if let Some(segundo_caracter) = palabra.chars().nth(1) {
                            let direccion = segundo_caracter;
                            juego.inicializar_desvio(punto, direccion);
                        }
                        else {
                            return Err(io::Error::new(
                                io::ErrorKind::InvalidData,
                                "Error al intentar inicializar el desvío en la dirección dada.",
                            ));
                        }
                    }
                    coordenada_y += 1;
                }
                //println!();
                //println!("Linea {} leída!",filas);
                filas += 1;
                coordenada_y = 0;
            }
            Err(e) => {
                eprintln!("Error al leer línea: {}", e);
            }
        }
    }

    filas -= 1;
    //println!("Hay {} filas",filas);

    juego.inicializar_dimension(filas);

    if let Ok(arg) = args[3].parse::<i8>() {
        // Intenta convertir el argumento en i8
        let x: i8 = arg;
    }
    else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Error en coordenada x.",
        ));
    }
    if let Ok(arg) = args[4].parse::<i8>() {
        // Intenta convertir el argumento en i8
        let y: i8 = arg;
    }
    else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Error en coordenada y.",
        ));
    }

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

