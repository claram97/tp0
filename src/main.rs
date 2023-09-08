use std::env;
use std::fs::File;
//use std::io::{self, BufRead, Write};
use std::io::{self, BufRead};
mod juego;
use juego::Juego;
use crate::juego::coordenada::Coordenada;
use crate::juego::bomba::TipoDeBomba;

const DESVIO: char = 'D';
const ENEMIGO: char = 'F';
const BOMBA_DE_TRANSPASO: char = 'S';
const BOMBA_NORMAL: char = 'B';
const PARED: &str = "W";
const ROCA: &str = "R";
const VACIO: &str = "_";


fn chequear_argumentos(args: &Vec<String>) -> io::Result<()> {
    if args.len() != 5 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Se esperaban exactamente 5 argumentos.",
        ));
    }
    Ok(())
}

fn funcion(l: &String,mut filas: i8,mut coordenada_y: i8){
    let mut juego: Juego = Juego::new();
    //que me devuelva filas
    //funcion(&l,filas,coordenada_y);
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
                    //return Err(io::Error::new(io::ErrorKind::InvalidData, "Error al intentar inicializar la bomba con el número de alcance dado.",));
                }
            }
            else {
                //return Err(io::Error::new(io::ErrorKind::InvalidInput,"No se pudo determinar el alcance de la bomba.",));
            }
        }
        else if palabra.starts_with(BOMBA_NORMAL) {
            if let Some(segundo_caracter) = palabra.chars().nth(1) {
                if let Some(digito) = segundo_caracter.to_digit(10) {
                    let alcance = digito as i8;
                    juego.inicializar_bomba(punto,alcance,juego::bomba::TipoDeBomba::Normal);
                }
                else {
                    //return Err(io::Error::new(io::ErrorKind::InvalidData, "Error al intentar inicializar la bomba con el número de alcance dado.",));
                }
            }
            else {
                //return Err(io::Error::new( io::ErrorKind::InvalidInput,"No se pudo determinar el alcance de la bomba.",));
            }
        }  
        else if palabra.starts_with(ENEMIGO) {
            if let Some(segundo_caracter) = palabra.chars().nth(1) {
                if let Some(digito) = segundo_caracter.to_digit(10) {
                    let vida = digito as i8;
                    juego.inicializar_enemigo(punto,vida);
                }
                else {
                    //return Err(io::Error::new(io::ErrorKind::InvalidData,"Error al intentar inicializar el enemigo con el puntaje de vida dado.",));
                }
            }
            else {
                    //return Err(io::Error::new(io::ErrorKind::InvalidInput,"No se pudo determinar la vida del enemigo."));
            }
        }
        else if palabra.starts_with(DESVIO) {
            if let Some(segundo_caracter) = palabra.chars().nth(1) {
                let direccion = segundo_caracter;
                juego.inicializar_desvio(punto, direccion);
            }
            else {
                //return Err(io::Error::new(io::ErrorKind::InvalidData,"Error al intentar inicializar el desvío en la dirección dada."));
            }
        }
        coordenada_y += 1;
    }
    //println!();
    //println!("Linea {} leída!",filas);
    filas += 1;
    coordenada_y = 0;
}

use std::error::Error;
use std::fmt;

// Define un tipo de error personalizado que implementa std::error::Error.
#[derive(Debug)]
struct CustomError(String);

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Custom error: {}", self.0)
    }
}

impl Error for CustomError {}

fn procesar_linea_de_configuracion(l: &str, filas: &mut i8, coordenada_y: &mut i8,juego: &mut Juego) -> Result<(), CustomError> {
    let mut errores: Vec<String> = Vec::new();

    let palabras: Vec<&str> = l.split_whitespace().collect();
    for palabra in palabras {
        let punto = Coordenada::new(*filas, *coordenada_y);

        match palabra {
            VACIO => {},
            PARED => juego.inicializar_pared(punto),
            ROCA => juego.inicializar_roca(punto),
            _ => {
                if palabra.starts_with(BOMBA_DE_TRANSPASO) {
                    if let Some(segundo_caracter) = palabra.chars().nth(1) {
                        if let Some(digito) = segundo_caracter.to_digit(10) {
                            let alcance = digito as i8;
                            juego.inicializar_bomba(punto, alcance, TipoDeBomba::Traspaso);
                        } else {
                            errores.push("Error al intentar inicializar la bomba con el número de alcance dado.".to_string());
                        }
                    } else {
                        errores.push("No se pudo determinar el alcance de la bomba.".to_string());
                    }
                } else if palabra.starts_with(BOMBA_NORMAL) {
                    if let Some(segundo_caracter) = palabra.chars().nth(1) {
                        if let Some(digito) = segundo_caracter.to_digit(10) {
                            let alcance = digito as i8;
                            juego.inicializar_bomba(punto, alcance, TipoDeBomba::Normal);
                        } else {
                            errores.push("Error al intentar inicializar la bomba normal con el número de alcance dado.".to_string());
                        }
                    } else {
                        errores.push("No se pudo determinar el alcance de la bomba normal.".to_string());
                    }
                } else if palabra.starts_with(ENEMIGO) {
                    if let Some(segundo_caracter) = palabra.chars().nth(1) {
                        if let Some(digito) = segundo_caracter.to_digit(10) {
                            let vida = digito as i8;
                            juego.inicializar_enemigo(punto, vida);
                        } else {
                            errores.push("Error al intentar inicializar el enemigo con el puntaje de vida dado.".to_string());
                        }
                    } else {
                        errores.push("No se pudo determinar la vida del enemigo.".to_string());
                    }
                } else if palabra.starts_with(DESVIO) {
                    if let Some(segundo_caracter) = palabra.chars().nth(1) {
                        let direccion = segundo_caracter;
                        juego.inicializar_desvio(punto, direccion);
                    } else {
                        errores.push("Error al intentar inicializar el desvío en la dirección dada.".to_string());
                    }
                } else {
                    errores.push(format!("Palabra desconocida: {}", palabra));
                }
            }
        }

        *coordenada_y += 1;
    }

    // Incrementamos filas y reiniciamos coordenada_y al final de cada línea
    *filas += 1;
    *coordenada_y = 0;

    // Comprobar errores y devolverlos si los hay
    let result = if errores.is_empty() {
        Ok(())
    } else {
        Err(CustomError(errores.join("\n")))
    };

    result
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
                let resultado = procesar_linea_de_configuracion(&l, &mut filas, &mut coordenada_y,&mut juego);
                match resultado {
                    Ok(_) => {
                
                    }
                    Err(err) => {
                        return Err(io::Error::new(
                            io::ErrorKind::Other,
                            err.to_string(),
                        ));
                    }
                }
                //que me devuelva filas
                //funcion(&l,filas,coordenada_y);
                /*let palabras: Vec<&str> = l.split_whitespace().collect();
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
*/          }
            Err(_e) => {
                return Err(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "Error al leer línea:",
                ));
            }
        }
    }

    filas -= 1;
    //println!("Hay {} filas",filas);

    juego.inicializar_dimension(filas);

    if let Ok(arg) = args[3].parse::<i8>() {
        // Intenta convertir el argumento en i8
        let _x: i8 = arg;
    }
    else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Error en coordenada x.",
        ));
    }
    if let Ok(arg) = args[4].parse::<i8>() {
        // Intenta convertir el argumento en i8
        let _y: i8 = arg;
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

