//! Este módulo contiene las funciones y estructuras para procesar el archivo de configuración del juego.

use crate::estructuras_juego::bomba::TipoDeBomba;
use crate::estructuras_juego::coordenada::Coordenada;
use crate::juego::Juego;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};

const DESVIO: char = 'D';
const ENEMIGO: char = 'F';
const BOMBA_DE_TRANSPASO: char = 'S';
const BOMBA_NORMAL: char = 'B';
const PARED: &str = "W";
const ROCA: &str = "R";

#[derive(Debug)]
struct CustomError(String);

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Custom error: {}", self.0)
    }
}

/// Procesa una bomba en el archivo de configuración.
///
/// Esta función procesa una línea del archivo de configuración que representa una bomba
/// y la inicializa en el juego.
///
/// # Argumentos
///
/// * `palabra`: La palabra que representa la bomba en el archivo de configuración.
/// * `punto`: La coordenada donde se inicializa la bomba.
/// * `juego`: Referencia mutable al juego en el que se inicializa la bomba.
///
/// # Errores
///
/// Esta función devuelve un error si no se puede determinar el alcance de la bomba o si hay
/// un problema al inicializarla.
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

/// Procesa una entrada que representa un enemigo en el archivo de configuración y lo inicializa en el juego.
///
/// Esta función toma una palabra del archivo de configuración que representa un enemigo y lo inicializa
/// en el juego con la vida especificada. La palabra debe tener un formato específico donde el segundo
/// carácter es un dígito que representa la vida del enemigo.
///
/// # Argumentos
///
/// * `palabra`: La palabra que representa el enemigo en el archivo de configuración.
/// * `punto`: La coordenada donde se inicializa el enemigo.
/// * `juego`: Referencia mutable al juego en el que se inicializa el enemigo.
///
/// # Errores
///
/// Esta función devuelve un error si no se puede determinar la vida del enemigo o si hay un problema al inicializarlo.
///
/// # Ejemplo
///
/*/// ```rust
/// use tu_modulo::procesar_enemigo;
/// use tu_modulo::Coordenada;
/// use tu_modulo::Juego;
///
/// let mut juego = Juego::new();
/// let palabra = "F3"; // Representa un enemigo con 3 puntos de vida.
/// let punto = Coordenada::new(0, 0);
/// let resultado = procesar_enemigo(palabra, punto, &mut juego);
///
/// assert!(resultado.is_ok());
/// ```*/
pub fn procesar_enemigo(palabra: &str, punto: Coordenada, juego: &mut Juego) -> io::Result<()> {
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

/// Procesa una entrada que representa un desvío en el archivo de configuración y lo inicializa en el juego.
///
/// Esta función toma una palabra del archivo de configuración que representa un desvío y lo inicializa
/// en el juego con la dirección y la palabra especificada. La dirección se encuentra en el segundo carácter
/// de la palabra.
///
/// # Argumentos
///
/// * `palabra`: La palabra que representa el desvío en el archivo de configuración.
/// * `punto`: La coordenada donde se inicializa el desvío.
/// * `juego`: Referencia mutable al juego en el que se inicializa el desvío.
///
/// # Errores
///
/// Esta función devuelve un error si no se puede determinar la dirección del desvío o si hay un problema al inicializarlo.
///
/// # Ejemplo
/*///
/// ```rust
/// use tu_modulo::procesar_desvio;
/// use tu_modulo::Coordenada;
/// use tu_modulo::Juego;
///
/// let mut juego = Juego::new();
/// let palabra = "Dn"; // Representa un desvío en la dirección 'n'.
/// let punto = Coordenada::new(0, 0);
/// let resultado = procesar_desvio(palabra, punto, &mut juego);
///
/// assert!(resultado.is_ok());
/// ```*/
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

/// Inicializa elementos del juego basados en una palabra del archivo de configuración.
///
/// Esta función procesa una palabra del archivo de configuración y realiza la inicialización
/// correspondiente en el juego en función del tipo de elemento representado por la palabra.
///
/// # Argumentos
///
/// * `punto`: La coordenada en la que se inicializa el elemento.
/// * `palabra`: La palabra que representa un elemento en el archivo de configuración.
/// * `juego`: Referencia mutable al juego en el que se inicializa el elemento.
///
/// # Errores
///
/// Esta función devuelve un error si hay problemas al inicializar el elemento, como la detección
/// de un formato incorrecto o la incapacidad de inicializar un tipo específico.
///
/*/// # Ejemplo
///
/// ```rust
/// use tu_modulo::inicializar_juego;
/// use tu_modulo::Coordenada;
/// use tu_modulo::Juego;
///
/// let mut juego = Juego::new();
/// let palabra = "W"; // Representa una pared.
/// let punto = Coordenada::new(0, 0);
/// let resultado = inicializar_juego(punto, palabra, &mut juego);
///
/// assert!(resultado.is_ok());
/// ```*/
pub fn inicializar_juego(punto: Coordenada, palabra: &str, juego: &mut Juego) -> io::Result<()> {
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

/// Procesa una línea del archivo de configuración y realiza la inicialización en el juego.
///
/// Esta función toma una línea del archivo de configuración que contiene palabras separadas por espacios en blanco.
/// Cada palabra representa un elemento que se inicializa en el juego en función de su tipo.
///
/// # Argumentos
///
/// * `l`: La línea del archivo de configuración que se va a procesar.
/// * `filas`: Referencia mutable al contador de filas.
/// * `coordenada_y`: Referencia mutable al contador de coordenadas en el eje Y.
/// * `juego`: Referencia mutable al juego en el que se inicializan los elementos.
///
/// # Errores
///
/// Esta función devuelve un error si hay problemas al procesar la línea, como la detección de un formato
/// incorrecto o la incapacidad de inicializar elementos.
///
/*/// # Ejemplo
///
/// ```rust
/// use tu_modulo::procesar_linea_de_configuracion;
/// use tu_modulo::Coordenada;
/// use tu_modulo::Juego;
///
/// let mut juego = Juego::new();
/// let linea = "W B2 F3"; // Representa una fila con una pared, una bomba y un enemigo.
/// let mut filas = 0;
/// let mut coordenada_y = 0;
/// let resultado = procesar_linea_de_configuracion(linea, &mut filas, &mut coordenada_y, &mut juego);
///
/// assert!(resultado.is_ok());
/// ```*/
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

/// Inicializa la coordenada de la bomba basándose en argumentos de entrada.
///
/// Esta función toma un vector de argumentos de entrada y extrae las coordenadas X e Y
/// para inicializar la posición de una bomba en el juego.
///
/// # Argumentos
///
/// * `args`: Vector de cadenas que contiene los argumentos de entrada, donde
///   `args[3]` representa la coordenada X y `args[4]` representa la coordenada Y.
///
/// # Errores
///
/// Esta función devuelve un error si hay problemas al inicializar las coordenadas, como la detección
/// de un formato incorrecto o la incapacidad de convertir las cadenas en valores numéricos.
///
/*/// # Ejemplo
///
/// ```rust
/// use tu_modulo::inicializar_coordenada_de_la_bomba;
/// use tu_modulo::Coordenada;
///
/// let args = vec!["programa", "archivo.txt", "salida.txt", "2", "3"];
/// let resultado = inicializar_coordenada_de_la_bomba(&args);
///
/// assert!(resultado.is_ok());
/// let coordenada = resultado.unwrap();
/// assert_eq!(coordenada.x, 2);
/// assert_eq!(coordenada.y, 3);
/// ```*/
pub fn inicializar_coordenada_de_la_bomba(args: &[String]) -> io::Result<Coordenada> {
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

/// Crea un archivo en la ruta especificada.
///
/// Esta función toma una ruta de archivo como entrada y crea un archivo en esa ruta. Si la ruta es relativa,
/// se considera en relación con el directorio de trabajo actual. Si la ruta es absoluta, se utiliza tal como está.
///
/// # Argumentos
///
/// * `ruta_entrada`: La ruta del archivo que se va a crear.
///
/// # Errores
///
/// Esta función devuelve un error si no se puede crear el archivo en la ruta especificada.
///
/*/// # Ejemplo
///
/// ```rust
/// use tu_modulo::crear_archivo_en_ruta;
/// use std::fs::File;
///
/// let resultado = crear_archivo_en_ruta("salida.txt");
///
/// assert!(resultado.is_ok());
/// let archivo = resultado.unwrap();
/// assert!(archivo.is_file());
/// ```*/
fn crear_archivo_en_ruta(ruta_entrada: &str) -> io::Result<File> {
    let ruta = Path::new(ruta_entrada);

    if ruta.is_absolute() {
        let ruta_completa: PathBuf = ruta.to_path_buf();
        File::create(ruta_completa)
    } else {
        let directorio_actual = std::env::current_dir()?;
        let ruta_completa: PathBuf = directorio_actual.join(ruta);
        File::create(ruta_completa)
    }
}

/// Ejecuta el programa principal.
///
/// Esta función es la entrada principal del programa. Toma un vector de argumentos de línea de comandos y
/// realiza la ejecución principal del programa, que incluye la carga del juego, la inicialización de la coordenada
/// de la bomba, y la realización de una jugada en el juego. El resultado se guarda en el archivo de salida especificado.
///
/// # Argumentos
///
/// * `args`: Vector de cadenas que contiene los argumentos de línea de comandos.
///
/// # Errores
///
/// Esta función devuelve un error si no se proporcionan exactamente 4 argumentos en la línea de comandos
/// o si hay problemas durante la ejecución del programa.
///
/*/// # Ejemplo
///
/// ```rust
/// use tu_modulo::run;
///
/// let args = vec!["programa", "archivo.txt", "salida.txt", "2", "3"];
/// let resultado = run(args);
///
/// assert!(resultado.is_ok());
/// ```*/
pub fn run(args: Vec<String>) -> io::Result<()> {
    if args.len() != 5 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Se esperaban exactamente 4 argumentos.",
        ));
    }

    let maze_file_name = &args[1];
    let output_file_name = &args[2];

    let mut output_file = crear_archivo_en_ruta(output_file_name)?;

    let mut juego = match cargar_juego(maze_file_name) {
        Ok(juego) => juego,
        Err(e) => return Err(e),
    };

    let coordenada_bomba = match inicializar_coordenada_de_la_bomba(&args) {
        Ok(coordenada) => coordenada,
        Err(e) => return Err(e),
    };

    if coordenada_bomba.x >= juego.dimension || coordenada_bomba.y >= juego.dimension {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "La bomba no puede estar fuera de rango.",
        ));
    }

    juego.realizar_jugada(&mut output_file, coordenada_bomba)?;

    Ok(())
}

/// Carga el juego desde un archivo de configuración.
///
/// Esta función toma el nombre de un archivo de configuración como entrada y carga el juego
/// a partir de dicho archivo. El juego se inicializa con elementos según la configuración
/// del archivo.
///
/// # Argumentos
///
/// * `maze_file_name`: El nombre del archivo de configuración del juego.
///
/// # Errores
///
/// Esta función devuelve un error si no se puede abrir o leer el archivo de configuración,
/// o si hay problemas al procesar su contenido.
///
/*/// # Ejemplo
///
/// ```rust
/// use tu_modulo::cargar_juego;
///
/// let maze_file_name = "archivo.txt";
/// let resultado = cargar_juego(maze_file_name);
///
/// assert!(resultado.is_ok());
/// ```*/
pub fn cargar_juego(maze_file_name: &str) -> io::Result<Juego> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn inicializar_coordenadas_erroneas_devuelve_error() {
        let args: Vec<String> = vec![
            "string".to_string(),
            "maze.txt".to_string(),
            "output.txt".to_string(),
            "x".to_string(),
            "9".to_string(),
        ];
        let resultado = inicializar_coordenada_de_la_bomba(&args);
        assert!(resultado.is_err())
    }

    #[test]
    pub fn inicializar_coordenadas_correctas_devuelve_ok() {
        let args: Vec<String> = vec![
            "string".to_string(),
            "maze.txt".to_string(),
            "output.txt".to_string(),
            "8".to_string(),
            "9".to_string(),
        ];
        let resultado = inicializar_coordenada_de_la_bomba(&args);
        assert!(resultado.is_ok())
    }

    #[test]
    pub fn inicializar_enemigo_sin_especificar_vida_devuelve_error() {
        let mut juego: Juego = Juego::new();
        let coordenada_enemigo: Coordenada = Coordenada::new(3, 5);
        let resultado = procesar_enemigo("F", coordenada_enemigo, &mut juego);
        assert!(resultado.is_err());
    }

    #[test]
    pub fn inicializar_enemigo_con_formato_correcto_devuelve_ok() {
        let mut juego: Juego = Juego::new();
        let coordenada_enemigo: Coordenada = Coordenada::new(3, 5);
        let resultado = procesar_enemigo("F5", coordenada_enemigo, &mut juego);
        assert!(resultado.is_ok());
    }

    #[test]
    pub fn inicializar_enemigo_con_formato_erroneo_devuelve_error() {
        let mut juego: Juego = Juego::new();
        let coordenada_enemigo: Coordenada = Coordenada::new(3, 5);
        let resultado = procesar_enemigo("FD", coordenada_enemigo, &mut juego);
        assert!(resultado.is_err());
    }
}
