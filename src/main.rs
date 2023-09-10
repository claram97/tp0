use std::env;
use std::fs::File;
use std::io::{self, BufRead};
mod juego;
use crate::juego::bomba::TipoDeBomba;
use crate::juego::coordenada::Coordenada;
use juego::Juego;
use std::fmt;

const DESVIO: char = 'D';
const ENEMIGO: char = 'F';
const BOMBA_DE_TRANSPASO: char = 'S';
const BOMBA_NORMAL: char = 'B';
const PARED: &str = "W";
const ROCA: &str = "R";
/*
fn chequear_argumentos(args: &Vec<String>) -> io::Result<()> {
    if args.len() != 5 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Se esperaban exactamente 4 argumentos.",
        ));
    }
    Ok(())
} */

#[derive(Debug)]
struct CustomError(String);

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Custom error: {}", self.0)
    }
}

fn procesar_bomba(palabra: &str, punto: Coordenada, juego: &mut Juego) -> io::Result<()> {
    if let Some(segundo_caracter) = palabra.chars().nth(1) {
        if let Some(digito) = segundo_caracter.to_digit(10) {
            let alcance = digito as i8;
            let tipo = if palabra.starts_with(BOMBA_DE_TRANSPASO) {
                TipoDeBomba::Traspaso
            } else {
                TipoDeBomba::Normal
            };
            juego.inicializar_bomba(punto, alcance, tipo, palabra.to_string());
        } else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Error al intentar inicializar la bomba con el número de alcance dado.",
            ));
        }
    } else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "No se pudo determinar el alcance de la bomba.",
        ));
    }
    Ok(())
}

fn procesar_enemigo(palabra: &str, punto: Coordenada, juego: &mut Juego) -> io::Result<()> {
    if let Some(segundo_caracter) = palabra.chars().nth(1) {
        if let Some(digito) = segundo_caracter.to_digit(10) {
            let vida = digito as i8;
            juego.inicializar_enemigo(punto, vida);
        } else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Error al intentar inicializar el enemigo con el puntaje de vida dado.",
            ));
        }
    } else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "No se pudo determinar la vida del enemigo.",
        ));
    }
    Ok(())
}

fn procesar_desvio(palabra: &str, punto: Coordenada, juego: &mut Juego) -> io::Result<()> {
    if let Some(segundo_caracter) = palabra.chars().nth(1) {
        let direccion = segundo_caracter;
        juego.inicializar_desvio(punto, direccion.to_string(), palabra.to_string());
    } else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Error al intentar inicializar el desvío en la dirección dada.",
        ));
    }
    Ok(())
}

fn inicializar_juego(punto: Coordenada, palabra: &str, juego: &mut Juego) -> io::Result<()> {
    if palabra == PARED {
        juego.inicializar_pared(punto);
    } else if palabra == ROCA {
        juego.inicializar_roca(punto);
    } else if palabra.starts_with(BOMBA_DE_TRANSPASO) || palabra.starts_with(BOMBA_NORMAL) {
        let resultado = procesar_bomba(palabra, punto, juego);
        match resultado {
            Ok(()) => {}
            Err(e) => {
                return Err(e);
            }
        }
    } else if palabra.starts_with(ENEMIGO) {
        let resultado = procesar_enemigo(palabra, punto, juego);
        match resultado {
            Ok(()) => {}
            Err(e) => {
                return Err(e);
            }
        }
    } else if palabra.starts_with(DESVIO) {
        let resultado = procesar_desvio(palabra, punto, juego);
        match resultado {
            Ok(()) => {}
            Err(e) => {
                return Err(e);
            }
        }
    }
    Ok(())
}

fn procesar_linea_de_configuracion(
    l: &str,
    filas: &mut i8,
    coordenada_y: &mut i8,
    juego: &mut Juego,
) -> io::Result<()> {
    let palabras: Vec<&str> = l.split_whitespace().collect();
    for palabra in palabras {
        let punto = Coordenada::new(*filas, *coordenada_y);
        let resultado = inicializar_juego(punto, palabra, juego);
        match resultado {
            Ok(()) => {}
            Err(e) => {
                return Err(e);
            }
        }
        *coordenada_y += 1;
    }
    *filas += 1;
    *coordenada_y = 0;
    Ok(())
}

fn inicializar_coordenada_de_la_bomba(args: &[String]) -> io::Result<Coordenada> {
    let x: i8;
    let y: i8;

    if let Ok(arg) = args[3].parse::<i8>() {
        x = arg;
    } else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Error en coordenada x.",
        ));
    }
    if let Ok(arg) = args[4].parse::<i8>() {
        y = arg;
    } else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Error en coordenada y.",
        ));
    }

    let coordenada_bomba: Coordenada = Coordenada::new(x, y);
    Ok(coordenada_bomba)
}

/*
fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    match chequear_argumentos(&args) {
        Ok(_) => {}
        Err(e) => {
            return Err(e);
        }
    }

    let output_file_name: &String = &args[2];

    let maze_file_name: &String = &args[1];
    let maze_file = File::open(maze_file_name)?;
    let reader = io::BufReader::new(maze_file);
    let mut juego: Juego = Juego::new();

    let mut filas: i8 = 0;
    let mut coordenada_y: i8 = 0;

    for linea in reader.lines() {
        match linea {
            Ok(l) => {
                let resultado =
                    procesar_linea_de_configuracion(&l, &mut filas, &mut coordenada_y, &mut juego);
                match resultado {
                    Ok(_) => continue,
                    Err(err) => {
                        return Err(io::Error::new(io::ErrorKind::Other, err.to_string()));
                    }
                }
            }
            Err(_e) => {
                return Err(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "Error al leer línea:",
                ));
            }
        }
    }

    juego.inicializar_dimension(filas);

    let mut coordenada_bomba: Coordenada = Coordenada::new(0, 0);
    let mut resultado = inicializar_coordenada_de_la_bomba(&args, &mut coordenada_bomba);
    match resultado {
        Ok(()) => {}
        Err(e) => return Err(e),
    }
    resultado = juego.realizar_jugada(output_file_name, coordenada_bomba);
    match resultado {
        Ok(()) => {}
        Err(e) => return Err(e),
    }

    Ok(())
}
*/
// Define las estructuras y funciones necesarias aquí, como `Juego`, `Coordenada`, etc.

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    run(args)?;
    Ok(())
}

fn run(args: Vec<String>) -> io::Result<()> {
    if args.len() != 5 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Se esperaban exactamente 4 argumentos.",
        ));
    }

    let maze_file_name = &args[1];
    let output_file_name = &args[2];

    let mut juego = match cargar_juego(maze_file_name) {
        Ok(juego) => juego,
        Err(e) => return Err(e),
    };

    let coordenada_bomba = match inicializar_coordenada_de_la_bomba(&args) {
        Ok(coordenada) => coordenada,
        Err(e) => return Err(e),
    };

    juego.realizar_jugada(output_file_name, coordenada_bomba)?;

    Ok(())
}

fn cargar_juego(maze_file_name: &str) -> io::Result<Juego> {
    let maze_file = File::open(maze_file_name)?;
    let reader = io::BufReader::new(maze_file);
    let mut juego: Juego = Juego::new();
    let mut filas: i8 = 0;
    let mut coordenada_y: i8 = 0;

    for linea in reader.lines() {
        match linea {
            Ok(l) => {
                if let Err(err) =
                    procesar_linea_de_configuracion(&l, &mut filas, &mut coordenada_y, &mut juego)
                {
                    return Err(io::Error::new(io::ErrorKind::Other, err.to_string()));
                }
            }
            Err(_e) => {
                return Err(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "Error al leer línea:",
                ));
            }
        }
    }

    juego.inicializar_dimension(filas);
    Ok(juego)
}
