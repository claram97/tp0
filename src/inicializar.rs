//! Este módulo contiene las funciones y estructuras para procesar el archivo de configuración del juego.

use crate::constantes;
use crate::estructuras_juego::bomba::TipoDeBomba;
use crate::estructuras_juego::coordenada::Coordenada;
use crate::juego::Juego;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::{Path, PathBuf};

#[derive(Debug)]
struct CustomError(String);

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
            let tipo = if palabra.starts_with(constantes::BOMBA_DE_TRANSPASO) {
                TipoDeBomba::Traspaso
            } else {
                TipoDeBomba::Normal
            };
            juego.inicializar_bomba(punto, alcance, tipo, palabra.to_string());
        } else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "ERROR: Error al intentar inicializar la bomba con el número de alcance dado.\n",
            ));
        }
    } else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "ERROR: No se pudo determinar el alcance de la bomba.\n",
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

fn procesar_enemigo(palabra: &str, punto: Coordenada, juego: &mut Juego) -> io::Result<()> {
    if let Some(segundo_caracter) = palabra.chars().nth(1) {
        if let Some(digito) = segundo_caracter.to_digit(10) {
            let vida = digito as i8;
            juego.inicializar_enemigo(punto, vida);
        } else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "ERROR: Error al intentar inicializar el enemigo con el puntaje de vida dado.\n",
            ));
        }
    } else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "ERROR: No se pudo determinar la vida del enemigo.\n",
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

fn procesar_desvio(palabra: &str, punto: Coordenada, juego: &mut Juego) -> io::Result<()> {
    if let Some(segundo_caracter) = palabra.chars().nth(1) {
        let direccion = segundo_caracter;
        juego.inicializar_desvio(punto, direccion.to_string(), palabra.to_string());
    } else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "ERROR: error al intentar inicializar el desvío en la dirección dada.\n",
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

fn inicializar_juego(punto: Coordenada, palabra: &str, juego: &mut Juego) -> io::Result<()> {
    if palabra == constantes::PARED {
        juego.inicializar_pared(punto);
    } else if palabra == constantes::ROCA {
        juego.inicializar_roca(punto);
    } else if palabra.starts_with(constantes::BOMBA_DE_TRANSPASO)
        || palabra.starts_with(constantes::BOMBA_NORMAL)
    {
        let resultado = procesar_bomba(palabra, punto, juego);
        resultado?;
    } else if palabra.starts_with(constantes::ENEMIGO) {
        let resultado = procesar_enemigo(palabra, punto, juego);
        resultado?;
    } else if palabra.starts_with(constantes::DESVIO) {
        let resultado = procesar_desvio(palabra, punto, juego);
        resultado?;
    } else if palabra == constantes::VACIO {
        return Ok(());
    } else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "ERROR: se encontraron caracteres inválidos en el archivo de entrada.\n",
        ));
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

fn inicializar_coordenada_de_la_bomba(args: &[String]) -> io::Result<Coordenada> {
    let x: i8;
    let y: i8;

    if let Ok(arg) = args[3].parse::<i8>() {
        y = arg;
    } else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "ERROR: Error en coordenada correspondiente a la fila.\n",
        ));
    }
    if let Ok(arg) = args[4].parse::<i8>() {
        x = arg;
    } else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "ERROR: Error en coordenada correspondiente a la columna.\n",
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

/// Verifica si la coordenada de la bomba está dentro del rango válido del juego.
///
/// # Argumentos
///
/// * `coordenada_bomba`: Coordenada de la bomba a verificar.
/// * `dimension_juego`: Dimensión del juego (tamaño del tablero).
/// * `output_file`: Archivo de salida donde se escribirán mensajes de error si es necesario.
///
/// # Devuelve
///
/// Retorna un `Result` que indica si la coordenada de la bomba es válida o no.
///
/// Si la coordenada de la bomba está fuera de rango, se escribirá un mensaje de error en `output_file`
/// y se devolverá un error de tipo `io::ErrorKind::InvalidInput`. En caso contrario, se devuelve `Ok(())`.
fn el_rango_de_la_bomba_es_valido(
    coordenada_bomba: Coordenada,
    dimension_juego: i8,
    output_file: &mut File,
) -> io::Result<()> {
    if coordenada_bomba.x >= dimension_juego || coordenada_bomba.y >= dimension_juego {
        let mensaje_error = "ERROR: La bomba no puede estar fuera de rango.\n";
        output_file.write_all(mensaje_error.as_bytes())?;
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "ERROR: La bomba no puede estar fuera de rango.\n",
        ));
    }
    Ok(())
}

/// Verifica si la cantidad de argumentos proporcionados es válida.
///
/// # Argumentos
///
/// * `len`: Cantidad de argumentos proporcionados.
///
/// # Devuelve
///
/// Retorna un `Result` que indica si la cantidad de argumentos es válida o no.
///
/// Si la cantidad de argumentos no es igual a 5, se devuelve un error de tipo `io::ErrorKind::InvalidInput`.
/// En caso contrario, se devuelve `Ok(())`.
fn la_cantidad_de_argumentos_es_valida(len: usize) -> io::Result<()> {
    if len != 5 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "ERROR: Se esperaban exactamente 4 argumentos.\n",
        ));
    }
    Ok(())
}

/// Función principal que ejecuta el juego.
///
/// # Argumentos
///
/// * `args`: Vector de cadenas de texto que contiene los argumentos de la línea de comandos.
/// En particular: el nombre del archivo de input (debe estar en la carpeta src),
/// el path del archivo de output (relativo, absoluto o nombre del archivo),
/// y las coordenadas de la bomba a explotar, que se recibirán en el formato columna fila.
///
/// # Devuelve
///
/// Retorna un `Result` que indica si la ejecución del juego fue exitosa o si hubo errores.
///
/// Esta función realiza las siguientes acciones:
/// 1. Verifica la cantidad de argumentos.
/// 2. Lee el nombre del archivo de laberinto y el archivo de salida de `args`.
/// 3. Carga el juego desde el archivo de laberinto.
/// 4. Inicializa la coordenada de la bomba.
/// 5. Verifica si la coordenada de la bomba está dentro del rango válido.
/// 6. Realiza una jugada en el juego.
///
/// Si en alguna etapa se encuentra un error, se escribirá un mensaje en el archivo de salida y se
/// devolverá un error correspondiente. En caso de éxito, se devuelve `Ok(())`.
pub fn run(args: Vec<String>) -> io::Result<()> {
    la_cantidad_de_argumentos_es_valida(args.len())?;

    let maze_file_name = &args[1];
    let output_file_name = &args[2];

    let mut output_file = crear_archivo_en_ruta(output_file_name)?;

    let mut juego = match cargar_juego(maze_file_name, &mut output_file) {
        Ok(juego) => juego,
        Err(e) => return Err(e),
    };

    let coordenada_bomba = match inicializar_coordenada_de_la_bomba(&args) {
        Ok(coordenada) => coordenada,
        Err(e) => {
            let mensaje_error = e.to_string();
            output_file.write_all(mensaje_error.as_bytes())?;
            return Err(e);
        }
    };

    el_rango_de_la_bomba_es_valido(coordenada_bomba, juego.dimension, &mut output_file)?;

    juego.realizar_jugada(&mut output_file, coordenada_bomba)?;

    Ok(())
}

/// Verifica si el tablero ingresado en el archivo de laberinto es cuadrado.
///
/// # Argumentos
///
/// * `cantidad_cadenas`: La cantidad total de cadenas procesadas del archivo de laberinto.
/// * `filas`: El número de filas procesadas del archivo de laberinto.
/// * `output_file`: Archivo de salida donde se escribirán mensajes de error si es necesario.
///
/// # Devuelve
///
/// Retorna un `Result` que indica si el tablero es cuadrado o no.
///
/// Si la cantidad de cadenas procesadas no coincide con el número de filas al cuadrado, se escribirá un mensaje de error
/// en `output_file` y se devolverá un error de tipo `io::ErrorKind::InvalidInput`. En caso contrario, se devuelve `Ok(())`.
fn el_tablero_ingresado_es_cuadrado(
    cantidad_cadenas: usize,
    filas: i8,
    output_file: &mut File,
) -> io::Result<()> {
    let cadenas_esperadas: usize = (filas * filas) as usize;
    if cantidad_cadenas != cadenas_esperadas {
        let mensaje_error = "ERROR: el tablero debe ser cuadrado.\n";
        output_file.write_all(mensaje_error.as_bytes())?;
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "ERROR: el tablero debe ser cuadrado.\n",
        ));
    }
    Ok(())
}

/// Carga el estado del juego desde un archivo de laberinto especificado y lo inicializa.
///
/// # Argumentos
///
/// * `maze_file_name`: El nombre del archivo de laberinto desde el cual se cargará el juego.
/// * `output_file`: Archivo de salida donde se escribirán mensajes de error si es necesario.
///
/// # Devuelve
///
/// Retorna un `Result` que contiene el objeto `Juego` inicializado si la carga y procesamiento del archivo
/// fueron exitosos, o un error en caso contrario.
///
/// Esta función realiza las siguientes acciones:
/// 1. Abre el archivo de laberinto especificado.
/// 2. Lee cada línea del archivo, procesando la configuración del juego y actualizando el estado de `juego`.
/// 3. Maneja los errores de lectura y procesamiento, escribiendo mensajes en `output_file` y devolviendo errores apropiados.
/// 4. Verifica si el tablero ingresado es cuadrado.
/// 5. Inicializa la dimensión del juego en base al número de filas procesadas.
///
/// Si todo se realiza correctamente, devuelve `Ok(juego)`. En caso de errores, se devuelve un error `io::Result`.
fn cargar_juego(maze_file_name: &str, output_file: &mut File) -> io::Result<Juego> {
    let maze_file = File::open(maze_file_name)?;
    let reader = io::BufReader::new(maze_file);
    let mut juego: Juego = Juego::new();
    let mut filas: i8 = 0;
    let mut coordenada_y: i8 = 0;
    let mut cantidad_cadenas = 0;
    for linea in reader.lines() {
        match linea {
            Ok(l) => {
                if let Err(err) =
                    procesar_linea_de_configuracion(&l, &mut filas, &mut coordenada_y, &mut juego)
                {
                    output_file.write_all(err.to_string().as_bytes())?;
                    return Err(io::Error::new(io::ErrorKind::Other, err.to_string()));
                }
                let cadenas = l.split_whitespace().collect::<Vec<&str>>();
                cantidad_cadenas += cadenas.len();
            }
            Err(e) => {
                output_file.write_all(e.to_string().as_bytes())?;
                return Err(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "ERROR: se han presentado inconvenientes en la lectura del archivo.\n",
                ));
            }
        }
    }
    el_tablero_ingresado_es_cuadrado(cantidad_cadenas, filas, output_file)?;
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
    pub fn inicializar_juego_con_enemigo_invalido_devuelve_error() {
        let mut juego: Juego = Juego::new();
        let coordenada_enemigo: Coordenada = Coordenada::new(3, 5);
        let resultado = inicializar_juego(coordenada_enemigo, "FB", &mut juego);

        assert!(resultado.is_err());
    }

    #[test]
    pub fn inicializar_juego_con_enemigo_correcto_devuelve_ok() {
        let mut juego: Juego = Juego::new();
        let coordenada_enemigo: Coordenada = Coordenada::new(3, 5);
        let resultado = inicializar_juego(coordenada_enemigo, "F6", &mut juego);
        assert!(resultado.is_ok());
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
