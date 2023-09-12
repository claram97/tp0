use crate::estructuras_juego::bomba::*;
use crate::estructuras_juego::coordenada::*;
use crate::estructuras_juego::desvio::*;
use crate::estructuras_juego::enemigo::*;
use crate::estructuras_juego::obstaculo::*;
use std::rc::Rc;
use std::{
    fs::File,
    io::{self, Write},
};

const ENEMIGO: &str = "F";
const BOMBA_DE_TRANSPASO: &str = "S";
const BOMBA_NORMAL: &str = "B";
const PARED: &str = "W";
const ROCA: &str = "R";
const DESVIO: &str = "D";
const DESVIO_ARRIBA: &str = "DU";
const DESVIO_ABAJO: &str = "DD";
const DESVIO_IZQUIERDA: &str = "DL";
const DESVIO_DERECHA: &str = "DR";

pub struct Juego {
    pub dimension: i8,
    pub enemigos: Vec<Enemigo>,
    obstaculos: Vec<Obstaculo>,
    bombas: Vec<Bomba>,
    desvios: Vec<Desvio>,
}

impl Default for Juego {
    fn default() -> Self {
        Self::new()
    }
}

impl Juego {
    pub fn new() -> Juego {
        Juego {
            dimension: 0,
            enemigos: Vec::new(),
            obstaculos: Vec::new(),
            bombas: Vec::new(),
            desvios: Vec::new(),
        }
    }

    /// Inicializa la dimensión del juego.
    ///
    /// Esta función establece la dimensión del juego con el valor proporcionado.
    ///
    /// # Argumentos
    ///
    /// * `dimension`: El valor de la dimensión del juego.
    ///
    pub fn inicializar_dimension(&mut self, dimension: i8) {
        self.dimension = dimension;
    }

    /// Inicializa un desvío en el juego.
    ///
    /// Esta función crea y agrega un nuevo desvío al juego con la coordenada, dirección e identificador especificados.
    ///
    /// # Argumentos
    ///
    /// * `coordenada`: La coordenada en la que se inicializa el desvío.
    /// * `direccion`: La dirección del desvío.
    /// * `id`: El identificador del desvío.
    ///
    pub fn inicializar_desvio(&mut self, coordenada: Coordenada, direccion: String, id: String) {
        let desvio: Desvio = Desvio::new(coordenada, direccion, id);
        self.desvios.push(desvio);
    }

    /// Inicializa un enemigo en el juego.
    ///
    /// Esta función crea y agrega un nuevo enemigo al juego con la coordenada y puntos de vida especificados.
    ///
    /// # Argumentos
    ///
    /// * `coordenada`: La coordenada en la que se inicializa el enemigo.
    /// * `vida`: Los puntos de vida del enemigo.
    ///
    pub fn inicializar_enemigo(&mut self, coordenada: Coordenada, vida: i8) {
        let enemigo: Enemigo = Enemigo::new(coordenada, vida);
        self.enemigos.push(enemigo);
    }

    /// Inicializa una roca en el juego.
    ///
    /// Esta función crea y agrega una nueva roca al juego en la coordenada especificada.
    ///
    /// # Argumentos
    ///
    /// * `coordenada`: La coordenada en la que se inicializa la roca.
    ///
    pub fn inicializar_roca(&mut self, coordenada: Coordenada) {
        let roca: Obstaculo = Obstaculo::new(TipoDeObstaculo::Roca, coordenada);
        self.obstaculos.push(roca);
    }

    /// Inicializa una bomba en el juego.
    ///
    /// Esta función crea y agrega una nueva bomba al juego con la coordenada, alcance, tipo e identificador especificados.
    ///
    /// # Argumentos
    ///
    /// * `coordenada`: La coordenada en la que se inicializa la bomba.
    /// * `alcance`: El alcance de la bomba.
    /// * `tipo`: El tipo de bomba.
    /// * `id`: El identificador de la bomba.
    ///
    pub fn inicializar_bomba(
        &mut self,
        coordenada: Coordenada,
        alcance: i8,
        tipo: TipoDeBomba,
        id: String,
    ) {
        println!("Inicializar bomba en: ({},{})", coordenada.x, coordenada.y);
        let bomba: Bomba = Bomba::new(coordenada, alcance, tipo, id);
        self.bombas.push(bomba);
    }

    /// Inicializa una pared en el juego.
    ///
    /// Esta función crea y agrega una nueva pared al juego en la coordenada especificada.
    ///
    /// # Argumentos
    ///
    /// * `coordenada`: La coordenada en la que se inicializa la pared.
    ///
    pub fn inicializar_pared(&mut self, coordenada: Coordenada) {
        let pared: Obstaculo = Obstaculo::new(TipoDeObstaculo::Pared, coordenada);
        self.obstaculos.push(pared);
    }

    /// Posiciona los enemigos en el tablero del juego.
    ///
    /// Esta función coloca los enemigos en las coordenadas correspondientes en el tablero,
    /// utilizando una representación única que combina el identificador y los puntos de vida del enemigo.
    ///
    /// # Argumentos
    ///
    /// * `tablero`: Una referencia mutable al tablero del juego representado como una matriz de cadenas.
    ///
    fn posicionar_enemigos(&self, tablero: &mut [Vec<String>]) {
        for enemigo in &self.enemigos {
            let id = (enemigo.id).to_string() + &enemigo.vida.to_string();
            tablero[enemigo.coordenada.x as usize][enemigo.coordenada.y as usize] = id;
        }
    }

    /// Posiciona los obstáculos en el tablero del juego.
    ///
    /// Esta función coloca los obstáculos en las coordenadas correspondientes en el tablero,
    /// utilizando el identificador del obstáculo como representación.
    ///
    /// # Argumentos
    ///
    /// * `tablero`: Una referencia mutable al tablero del juego representado como una matriz de cadenas.
    ///
    fn posicionar_obstaculos(&self, tablero: &mut [Vec<String>]) {
        for obstaculo in &self.obstaculos {
            let id: String = "".to_string() + &obstaculo.id;
            tablero[obstaculo.coordenada.x as usize][obstaculo.coordenada.y as usize] = id;
        }
    }

    /// Posiciona las bombas en el tablero del juego.
    ///
    /// Esta función coloca las bombas en las coordenadas correspondientes en el tablero,
    /// utilizando el identificador de la bomba como representación.
    ///
    /// # Argumentos
    ///
    /// * `tablero`: Una referencia mutable al tablero del juego representado como una matriz de cadenas.
    ///
    fn posicionar_bombas(&self, tablero: &mut [Vec<String>]) {
        for bomba in &self.bombas {
            let id: String = "".to_string() + &bomba.id;
            tablero[bomba.coordenada.x as usize][bomba.coordenada.y as usize] = id;
        }
    }

    /// Posiciona los desvíos en el tablero del juego.
    ///
    /// Esta función coloca los desvíos en las coordenadas correspondientes en el tablero,
    /// utilizando el identificador del desvío como representación.
    ///
    /// # Argumentos
    ///
    /// * `tablero`: Una referencia mutable al tablero del juego representado como una matriz de cadenas.
    ///
    fn posicionar_desvios(&self, tablero: &mut [Vec<String>]) {
        for desvio in &self.desvios {
            let id: String = "".to_string() + &desvio.id;
            tablero[desvio.coordenada.x as usize][desvio.coordenada.y as usize] = id;
        }
    }

    /// Posiciona todos los elementos en el tablero del juego.
    ///
    /// Esta función crea un nuevo tablero y coloca en él todos los elementos del juego,
    /// incluyendo enemigos, obstáculos, bombas y desvíos. El tablero es representado como
    /// una matriz de cadenas.
    ///
    /// # Retorno
    ///
    /// Un vector de vectores de cadenas que representa el tablero del juego.
    ///
    fn posicionar_elementos_en_tablero(&self) -> Vec<Vec<String>> {
        let mut tablero: Vec<Vec<String>> =
            vec![vec!["_".to_string(); self.dimension as usize]; self.dimension as usize];
        self.posicionar_enemigos(&mut tablero);
        self.posicionar_bombas(&mut tablero);
        self.posicionar_desvios(&mut tablero);
        self.posicionar_obstaculos(&mut tablero);
        tablero
    }

    /// Imprime el tablero en la consola.
    ///
    /// Esta función imprime el contenido del tablero en la consola. Cada elemento del tablero se separa por espacio,
    /// y cada fila del tablero se imprime en una nueva línea.
    ///
    /// # Argumentos
    ///
    /// * `tablero`: Un vector de vectores de cadenas que representa el tablero del juego.
    ///
    pub fn imprimir_tablero(&self, tablero: &Vec<Vec<String>>) {
        for row in tablero {
            for element in row {
                print!("{} ", element);
            }
            println!();
        }
    }

    /// Imprime el tablero en un archivo.
    ///
    /// Esta función imprime el contenido del tablero en un archivo abierto en modo escritura. Cada elemento del tablero se
    /// separa por espacio, y cada fila del tablero se imprime en una nueva línea en el archivo.
    ///
    /// # Argumentos
    ///
    /// * `output_file`: Una referencia mutable a un archivo abierto en modo escritura.
    /// * `tablero`: Un vector de vectores de cadenas que representa el tablero del juego.
    ///
    /// # Errores
    ///
    /// Esta función devuelve un error si no se puede escribir en el archivo.
    ///
    pub fn imprimir_tablero_en_archivo(
        &self,
        output_file: &mut File,
        tablero: &Vec<Vec<String>>,
    ) -> std::io::Result<()> {
        for row in tablero {
            let row_str: String = row
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<_>>()
                .join(" ");
            let resultado = writeln!(output_file, "{}", row_str);
            match resultado {
                Err(e) => return Err(e),
                Ok(()) => continue,
            }
        }
        Ok(())
    }

    /// Realiza una jugada en el juego y guarda el estado final en un archivo.
    ///
    /// Esta función realiza una jugada en el juego, detonando una bomba en la coordenada especificada.
    /// Luego, muestra el tablero inicial y el tablero final en la consola, y guarda el tablero final en un archivo.
    ///
    /// # Argumentos
    ///
    /// * `output_file`: Una referencia mutable a un archivo abierto en modo escritura donde se guardará el tablero final.
    /// * `coordenada`: La coordenada en la que se detonará la bomba.
    ///
    /// # Errores
    ///
    /// Esta función devuelve un error si no se puede escribir en el archivo de salida.
    ///
    pub fn realizar_jugada(
        &mut self,
        output_file: &mut File,
        coordenada: Coordenada,
    ) -> io::Result<()> {
        let mut tablero: Vec<Vec<String>> = self.posicionar_elementos_en_tablero();

        self.detonar_bomba(&mut tablero, coordenada);
        println!();
        println!("***************");
        println!("Tablero inicial: ");
        self.imprimir_tablero(&tablero);

        let tablero_final = self.posicionar_elementos_en_tablero();
        println!();
        println!("***************");
        println!("Tablero final: ");
        self.imprimir_tablero(&tablero_final);

        self.imprimir_tablero_en_archivo(output_file, &tablero_final)?;

        println!();
        println!("***************");
        println!();
        Ok(())
    }

    /// Busca un enemigo en el vector de enemigos en función de sus coordenadas.
    ///
    /// # Argumentos
    ///
    /// * `coordenada`: La coordenada en la que se busca al enemigo.
    ///
    /// # Devolución
    ///
    /// Si se encuentra un enemigo con las coordenadas especificadas, se devuelve
    /// `Some(usize)` que contiene la posición del enemigo en el vector de enemigos.
    /// Si no se encuentra ningún enemigo con las coordenadas dadas, se devuelve `None`.
    ///
    pub fn buscar_enemigo(&self, coordenada: Coordenada) -> Option<usize> {
        self.enemigos
            .iter()
            .position(|enemigo| enemigo.coordenada.is_equal_to(&coordenada))
    }

    /// Elimina un enemigo en función de sus coordenadas y la coordenada de la bomba.
    ///
    /// Si se encuentra un enemigo en las coordenadas especificadas y la bomba no ha impactado
    /// previamente en él, reduce la vida del enemigo y actualiza su lista de bombas.
    /// Si la vida del enemigo llega a 0, se elimina del vector de enemigos.
    ///
    /// # Argumentos
    ///
    /// * `coordenada`: La coordenada en la que se busca al enemigo.
    /// * `coordenada_bomba`: La coordenada de la bomba.
    ///
    pub fn eliminar_enemigo(&mut self, coordenada: Coordenada, coordenada_bomba: Coordenada) {
        println!("Eliminar enemigo");

        if let Some(i) = self.buscar_enemigo(coordenada) {
            if let Some(_bomba_index) = self.enemigos[i]
                .bombas_que_lo_impactaron
                .iter()
                .position(|&coord| coord.is_equal_to(&coordenada_bomba))
            {
                return;
            }
            if self.enemigos[i].vida > 0 {
                self.enemigos[i].vida -= 1;
                self.enemigos[i].actualizar_lista_de_bombas(coordenada_bomba);
                if self.enemigos[i].vida == 0 {
                    self.enemigos.swap_remove(i);
                }
            }
        }
    }

    /// Evalúa el contenido de un casillero en el tablero y realiza las acciones correspondientes.
    ///
    /// Esta función evalúa el contenido de un casillero en el tablero en función de su tipo y estado actual.
    /// Realiza acciones como eliminar enemigos, detonar bombas, desviar la trayectoria de una bomba o manejar obstáculos.
    ///
    /// # Argumentos
    ///
    /// * `coordenada`: Una referencia mutable a la coordenada actual en el tablero.
    /// * `tablero`: Una referencia mutable al tablero del juego.
    /// * `i`: Un contador que representa el alcance actual de la explosión de una bomba.
    /// * `bomba`: Una referencia a la bomba actual que está siendo evaluada.
    /// * `coordenada_original`: La coordenada original de la bomba antes de comenzar su trayectoria.
    ///
    fn evaluar_casillero(
        &mut self,
        coordenada: &mut Coordenada,
        tablero: &mut Vec<Vec<String>>,
        mut i: i8,
        bomba: &Bomba,
        coordenada_original: Coordenada,
    ) {
        if coordenada.x >= 0
            && coordenada.y >= 0
            && coordenada.x < self.dimension
            && coordenada.y < self.dimension
        {
            let casillero: &str = &tablero[coordenada.x as usize][coordenada.y as usize];
            match casillero {
                a if a.starts_with(ENEMIGO) => {
                    self.eliminar_enemigo(*coordenada, coordenada_original);
                }
                b if b.starts_with(BOMBA_DE_TRANSPASO) || b.starts_with(BOMBA_NORMAL) => {
                    self.detonar_bomba(tablero, *coordenada);
                }
                c if c.starts_with(DESVIO) => {
                    //println!("Desvío encontrado. Alcance actual: {}",i);
                    i += 1;
                    //println!("Incrementado alcance al {}... Listo!",i);
                    if c == DESVIO_ARRIBA {
                        self.evaluar_arriba(coordenada, tablero, i, bomba);
                    } else if c == DESVIO_ABAJO {
                        self.evaluar_abajo(coordenada, tablero, i, bomba);
                    } else if c == DESVIO_DERECHA {
                        self.evaluar_izquierda(coordenada, tablero, i, bomba);
                    } else if c == DESVIO_IZQUIERDA {
                        self.evaluar_derecha(coordenada, tablero, i, bomba);
                    }
                }
                d if d.starts_with(ROCA) => {
                    if bomba.tipo == TipoDeBomba::Normal {
                        coordenada.x = -1;
                        coordenada.y = -1;
                    }
                }
                e if e.starts_with(PARED) => {
                    coordenada.x = -1;
                    coordenada.y = -1;
                }
                _ => {}
            }
        }
    }

    /// Evalúa las posiciones hacia arriba desde la coordenada actual en busca de obstáculos o enemigos.
    ///
    /// Esta función verifica las posiciones hacia arriba desde la coordenada actual y realiza las
    /// acciones correspondientes, como eliminar enemigos, detonar bombas o desviar la trayectoria de una bomba.
    ///
    /// # Argumentos
    ///
    /// * `coordenada`: Una referencia inmutable a la coordenada actual en el tablero.
    /// * `tablero`: Una referencia mutable al tablero del juego.
    /// * `i`: Un contador que representa el alcance actual de la explosión de una bomba.
    /// * `bomba`: Una referencia a la bomba actual que está siendo evaluada.
    ///
    pub fn evaluar_arriba(
        &mut self,
        coordenada: &Coordenada,
        tablero: &mut Vec<Vec<String>>,
        mut i: i8,
        bomba: &Bomba,
    ) {
        let mut j: i8 = 1;
        while i <= bomba.alcance {
            let mut coordenada_a_evaluar = Coordenada::new(coordenada.x - j, coordenada.y);
            self.evaluar_casillero(&mut coordenada_a_evaluar, tablero, i, bomba, *coordenada);
            if coordenada_a_evaluar.x == -1 && coordenada_a_evaluar.y == -1 {
                return;
            }
            j += 1;
            i += 1;
        }
    }

    /// Evalúa las posiciones hacia abajo desde la coordenada actual en busca de obstáculos o enemigos.
    ///
    /// Esta función verifica las posiciones hacia abajo desde la coordenada actual y realiza las
    /// acciones correspondientes, como eliminar enemigos, detonar bombas o desviar la trayectoria de una bomba.
    ///
    /// # Argumentos
    ///
    /// * `coordenada`: Una referencia inmutable a la coordenada actual en el tablero.
    /// * `tablero`: Una referencia mutable al tablero del juego.
    /// * `i`: Un contador que representa el alcance actual de la explosión de una bomba.
    /// * `bomba`: Una referencia a la bomba actual que está siendo evaluada.
    ///
    fn evaluar_abajo(
        &mut self,
        coordenada: &Coordenada,
        tablero: &mut Vec<Vec<String>>,
        mut i: i8,
        bomba: &Bomba,
    ) {
        let mut j: i8 = 1;
        while i <= bomba.alcance {
            let mut coordenada_a_evaluar: Coordenada =
                Coordenada::new(coordenada.x + j, coordenada.y);
            self.evaluar_casillero(&mut coordenada_a_evaluar, tablero, i, bomba, *coordenada);
            if coordenada_a_evaluar.x == -1 && coordenada_a_evaluar.y == -1 {
                return;
            }
            j += 1;
            i += 1;
        }
    }

    /// Evalúa las posiciones hacia la izquierda desde la coordenada actual en busca de obstáculos o enemigos.
    ///
    /// Esta función verifica las posiciones hacia la izquierda desde la coordenada actual y realiza las
    /// acciones correspondientes, como eliminar enemigos, detonar bombas o desviar la trayectoria de una bomba.
    ///
    /// # Argumentos
    ///
    /// * `coordenada`: Una referencia inmutable a la coordenada actual en el tablero.
    /// * `tablero`: Una referencia mutable al tablero del juego.
    /// * `i`: Un contador que representa el alcance actual de la explosión de una bomba.
    /// * `bomba`: Una referencia a la bomba actual que está siendo evaluada.
    ///
    fn evaluar_izquierda(
        &mut self,
        coordenada: &Coordenada,
        tablero: &mut Vec<Vec<String>>,
        mut i: i8,
        bomba: &Bomba,
    ) {
        let mut j: i8 = 1;
        while i <= bomba.alcance {
            let mut coordenada_a_evaluar: Coordenada =
                Coordenada::new(coordenada.x, coordenada.y - j);

            self.evaluar_casillero(&mut coordenada_a_evaluar, tablero, i, bomba, *coordenada);
            if coordenada_a_evaluar.x == -1 && coordenada_a_evaluar.y == -1 {
                return;
            }
            i += 1;
            j += 1;
        }
    }

    /// Evalúa las posiciones hacia la derecha desde la coordenada actual en busca de obstáculos o enemigos.
    ///
    /// Esta función verifica las posiciones hacia la derecha desde la coordenada actual y realiza las
    /// acciones correspondientes, como eliminar enemigos, detonar bombas o desviar la trayectoria de una bomba.
    ///
    /// # Argumentos
    ///
    /// * `coordenada`: Una referencia inmutable a la coordenada actual en el tablero.
    /// * `tablero`: Una referencia mutable al tablero del juego.
    /// * `i`: Un contador que representa el alcance actual de la explosión de una bomba.
    /// * `bomba`: Una referencia a la bomba actual que está siendo evaluada.
    ///
    fn evaluar_derecha(
        &mut self,
        coordenada: &Coordenada,
        tablero: &mut Vec<Vec<String>>,
        mut i: i8,
        bomba: &Bomba,
    ) {
        let mut j: i8 = 1;
        while i <= bomba.alcance {
            let mut coordenada_a_evaluar: Coordenada =
                Coordenada::new(coordenada.x, coordenada.y + j);

            self.evaluar_casillero(&mut coordenada_a_evaluar, tablero, i, bomba, *coordenada);
            if coordenada_a_evaluar.x == -1 && coordenada_a_evaluar.y == -1 {
                return;
            }
            i += 1;
            j += 1;
        }
    }

    /// Evalúa el alcance de una bomba y su impacto en el tablero.
    ///
    /// Esta función evalúa el alcance de una bomba y su impacto en el tablero, considerando las direcciones
    /// arriba, abajo, izquierda y derecha desde la posición de la bomba. Realiza las acciones correspondientes
    /// como eliminar enemigos, detonar bombas o desviar la trayectoria de una bomba en cada dirección.
    ///
    /// # Argumentos
    ///
    /// * `bomba`: Una referencia a la bomba que se va a evaluar.
    /// * `tablero`: Una referencia mutable al tablero del juego.
    ///
    fn funcion_bomba(&mut self, bomba: &Bomba, tablero: &mut Vec<Vec<String>>) {
        self.evaluar_arriba(&bomba.coordenada, tablero, 1, bomba);
        self.evaluar_abajo(&bomba.coordenada, tablero, 1, bomba);
        self.evaluar_izquierda(&bomba.coordenada, tablero, 1, bomba);
        self.evaluar_derecha(&bomba.coordenada, tablero, 1, bomba);
    }

    /// Detona una bomba en el tablero en la coordenada especificada.
    ///
    /// Esta función busca una bomba en la lista de bombas del objeto actual (`self`)
    /// que coincide con la coordenada especificada. Si se encuentra una bomba válida,
    /// la función la detona y ejecuta la función `funcion_bomba` con la bomba como argumento.
    /// Si la bomba ya ha sido detonada, la función no hace nada.
    ///
    /// # Argumentos
    ///
    /// - `tablero`: Un tablero mutable representado como un vector bidimensional de cadenas.
    /// - `coordenada`: La coordenada en la que se busca y detona la bomba.
    ///
    /*/// # Ejemplo
    ///
    /// ```rust
    /// let mut juego = Juego::new(); // Crea una instancia del juego.
    /// let coordenada = Coordenada { x: 2, y: 3 };
    /// juego.detonar_bomba(&mut tablero, coordenada);
    /// ```*/
    pub fn detonar_bomba(&mut self, tablero: &mut Vec<Vec<String>>, coordenada: Coordenada) {
        let mut bomba_index: Option<usize> = None;

        for (i, bomba) in self.bombas.iter().enumerate().rev() {
            if bomba.coordenada.is_equal_to(&coordenada) {
                bomba_index = Some(i);
                break;
            }
        }

        if let Some(i) = bomba_index {
            if !self.bombas[i].detonada {
                let bomba_rc = Rc::new(self.bombas[i].clone());
                self.bombas[i].detonar();

                self.funcion_bomba(&bomba_rc, tablero);

                return;
            } else {
                return;
            }
        }

        println!("Bomba no encontrada");
    }
}
