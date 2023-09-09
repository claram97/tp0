pub mod bomba;
pub mod coordenada;
pub mod desvio;
pub mod enemigo;
pub mod obstaculo;
use std::{
    fs::File,
    io::{self, Write},
};

use bomba::Bomba;
use coordenada::Coordenada;
use desvio::Desvio;
use enemigo::Enemigo;
use obstaculo::Obstaculo;

use self::bomba::TipoDeBomba;

const ENEMIGO: char = 'F';
const BOMBA_DE_TRANSPASO: char = 'S';
const BOMBA_NORMAL: char = 'B';
const PARED: char = 'W';
const ROCA: char = 'R';

pub struct Juego {
    dimension: i8,
    enemigos: Vec<Enemigo>,
    obstaculos: Vec<Obstaculo>,
    bombas: Vec<Bomba>,
    desvios: Vec<Desvio>,
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

    pub fn inicializar_dimension(&mut self, dimension: i8) {
        self.dimension = dimension;
    }

    pub fn inicializar_desvio(&mut self, coordenada: Coordenada, direccion: char) {
        let desvio: Desvio = Desvio::new(coordenada, direccion);
        self.desvios.push(desvio);
    }

    pub fn inicializar_enemigo(&mut self, coordenada: Coordenada, vida: i8) {
        let enemigo: Enemigo = Enemigo::new(coordenada, vida);
        self.enemigos.push(enemigo);
    }

    pub fn inicializar_roca(&mut self, coordenada: Coordenada) {
        let roca: Obstaculo = Obstaculo::new(obstaculo::TipoDeObstaculo::Roca, coordenada);
        self.obstaculos.push(roca);
    }

    pub fn inicializar_bomba(
        &mut self,
        coordenada: Coordenada,
        alcance: i8,
        tipo: bomba::TipoDeBomba,
    ) {
        println!("Inicializar bomba en: ({},{})", coordenada.x, coordenada.y);
        let bomba: Bomba = Bomba::new(coordenada, alcance, tipo);
        self.bombas.push(bomba);
    }

    pub fn inicializar_pared(&mut self, coordenada: Coordenada) {
        let pared: Obstaculo = Obstaculo::new(obstaculo::TipoDeObstaculo::Pared, coordenada);
        self.obstaculos.push(pared);
    }

    fn posicionar_enemigos(&self, tablero: &mut [Vec<char>]) {
        for enemigo in &self.enemigos {
            tablero[enemigo.coordenada.x as usize][enemigo.coordenada.y as usize] = enemigo.id;
        }
    }

    fn posicionar_obstaculos(&self, tablero: &mut [Vec<char>]) {
        for obstaculo in &self.obstaculos {
            tablero[obstaculo.coordenada.x as usize][obstaculo.coordenada.y as usize] =
                obstaculo.id;
        }
    }

    fn posicionar_bombas(&self, tablero: &mut [Vec<char>]) {
        for bomba in &self.bombas {
            tablero[bomba.coordenada.x as usize][bomba.coordenada.y as usize] = bomba.id;
        }
    }

    fn posicionar_desvios(&self, tablero: &mut [Vec<char>]) {
        for desvio in &self.desvios {
            tablero[desvio.coordenada.x as usize][desvio.coordenada.y as usize] = desvio.direccion;
        }
    }

    fn posicionar_elementos_en_tablero(&self) -> Vec<Vec<char>> {
        let mut tablero: Vec<Vec<char>> =
            vec![vec!['_'; self.dimension as usize]; self.dimension as usize];
        self.posicionar_enemigos(&mut tablero);
        self.posicionar_bombas(&mut tablero);
        self.posicionar_desvios(&mut tablero);
        self.posicionar_obstaculos(&mut tablero);
        tablero
    }

    pub fn imprimir_tablero(&self, tablero: &Vec<Vec<char>>) {
        for row in tablero {
            for &element in row {
                print!("{} ", element);
            }
            println!();
        }
    }

    pub fn realizar_jugada(
        &mut self,
        output_path: &String,
        coordenada: Coordenada,
    ) -> io::Result<()> {
        /*let mut tablero: Vec<Vec<char>> =
            vec![vec!['_'; self.dimension as usize]; self.dimension as usize];
        self.posicionar_elementos_en_tablero(&mut tablero);
        */
        let mut tablero = self.posicionar_elementos_en_tablero();

        self.detonar_bomba(&mut tablero, coordenada);
        println!("Tablero inicial: ");
        self.imprimir_tablero(&tablero);

        let tablero_final = self.posicionar_elementos_en_tablero();
        println!("Tablero final: ");
        self.imprimir_tablero(&tablero_final);

        let mut output_file = File::create(output_path)?;

        for row in &tablero {
            let row_str: String = row
                .iter()
                .map(|&c| c.to_string())
                .collect::<Vec<_>>()
                .join(" ");
            writeln!(output_file, "{}", row_str)?;
        }

        Ok(())
    }

    fn eliminar_enemigo(&mut self, coordenada: Coordenada) -> bool {
        println!("Eliminar enemigo");
        if let Some(i) = self
            .enemigos
            .iter_mut()
            .position(|enemigo| enemigo.coordenada.is_equal_to(&coordenada))
        {
            if self.enemigos[i].vida > 0 {
                println!("Vida inicial: {}",self.enemigos[i].vida);
                self.enemigos[i].vida -= 1;
                println!("Vida actual: {}",self.enemigos[i].vida);
                if self.enemigos[i].vida == 0 {
                    self.enemigos.swap_remove(i);
                    return true;
                }
            }
        }

        false
    }

    fn funcion_bomba(&mut self, bomba: &Bomba, tablero: &mut Vec<Vec<char>>) {
        let alcance = bomba.alcance;
        let coordenada = bomba.coordenada;

        let direcciones = [(0, 1), (1, 0), (0, -1), (-1, 0)]; // Direcciones: Derecha, Abajo, Izquierda, Arriba

        for &(dx, dy) in &direcciones {
            let mut incremento = 1;
            let mut x = coordenada.x + dx;
            let mut y = coordenada.y + dy;

            while incremento <= alcance {
                if x >= 0 && y >= 0 && x < tablero.len() as i8 && y < tablero[0].len() as i8 {
                    let casilla = tablero[x as usize][y as usize];

                    match casilla {
                        ENEMIGO => {
                            let coordenada_enemigo = Coordenada::new(x, y);
                            self.eliminar_enemigo(coordenada_enemigo);
                        }
                        BOMBA_DE_TRANSPASO | BOMBA_NORMAL => {
                            let coordenada_bomba = Coordenada::new(x, y);
                            self.detonar_bomba(tablero, coordenada_bomba);
                        }
                        ROCA => {
                            if bomba.tipo == TipoDeBomba::Normal {
                                println!("Las bombas normales no pueden atravesar rocas.");
                                break; // Detenerse si es una roca y la bomba es normal
                            }
                        }
                        PARED => {
                            println!("Ninguna pared puede ser atravesada.");
                            break; // Detenerse si es una pared
                        }
                        _ => {} // Otros casos
                    }
                } else {
                    break; // Detenerse si se sale del tablero
                }

                incremento += 1;
                x += dx;
                y += dy;
            }
        }
    }

    fn detonar_bomba(&mut self, tablero: &mut Vec<Vec<char>>, coordenada: Coordenada) {
        let mut bomba_index: Option<usize> = None;

        // Encuentra la bomba correspondiente
        for (i, bomba) in self.bombas.iter().enumerate().rev() {
            if bomba.coordenada.is_equal_to(&coordenada) {
                bomba_index = Some(i);
                break;
            }
        }

        if let Some(i) = bomba_index {
            // Hacer una copia mutable de la bomba
            if !self.bombas[i].detonada {
                self.bombas[i].detonar();
                let bomba = self.bombas[i].clone();

                // Llama a la función para detonar la bomba
                self.funcion_bomba(&self.bombas[i].clone(), tablero);

                // Actualiza la bomba en self.bombas
                self.bombas[i] = bomba;

                println!(
                    "Bomba detonada en ({},{})",
                    self.bombas[i].coordenada.x, self.bombas[i].coordenada.y
                );
                return;
            } else {
                return;
            }
        }

        println!("Bomba no encontrada");
    }
}
