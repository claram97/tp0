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

const ENEMIGO: &str = "F";
const BOMBA_DE_TRANSPASO: &str = "S";
const BOMBA_NORMAL: &str = "B";
const PARED: &str = "W";
const ROCA: &str = "R";
const DESVIO: &str = "D";

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

    pub fn inicializar_desvio(&mut self, coordenada: Coordenada, direccion: String, id: String) {
        let desvio: Desvio = Desvio::new(coordenada, direccion, id);
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
        id: String,
    ) {
        println!("Inicializar bomba en: ({},{})", coordenada.x, coordenada.y);
        let bomba: Bomba = Bomba::new(coordenada, alcance, tipo, id);
        self.bombas.push(bomba);
    }

    pub fn inicializar_pared(&mut self, coordenada: Coordenada) {
        let pared: Obstaculo = Obstaculo::new(obstaculo::TipoDeObstaculo::Pared, coordenada);
        self.obstaculos.push(pared);
    }

    fn posicionar_enemigos(&self, tablero: &mut [Vec<String>]) {
        for enemigo in &self.enemigos {
            let id = "F".to_string() + &enemigo.vida.to_string(); 
            tablero[enemigo.coordenada.x as usize][enemigo.coordenada.y as usize] =
                id;
        }
    }

    fn posicionar_obstaculos(&self, tablero: &mut [Vec<String>]) {
        for obstaculo in &self.obstaculos {
            let id: String = "".to_string() + &obstaculo.id;
            tablero[obstaculo.coordenada.x as usize][obstaculo.coordenada.y as usize] =
            id;
        }
    }

    fn posicionar_bombas(&self, tablero: &mut [Vec<String>]) {
        for bomba in &self.bombas {
            let id: String = "".to_string() + &bomba.id;
            tablero[bomba.coordenada.x as usize][bomba.coordenada.y as usize] = id;
        }
    }

    fn posicionar_desvios(&self, tablero: &mut [Vec<String>]) {
        for desvio in &self.desvios {
            let id: String = "".to_string() + &desvio.id;
            tablero[desvio.coordenada.x as usize][desvio.coordenada.y as usize] = id;
        }
    }

    fn posicionar_elementos_en_tablero(&self) -> Vec<Vec<String>> {
        let mut tablero: Vec<Vec<String>> =
            vec![vec!["_".to_string(); self.dimension as usize]; self.dimension as usize];
        self.posicionar_enemigos(&mut tablero);
        self.posicionar_bombas(&mut tablero);
        self.posicionar_desvios(&mut tablero);
        self.posicionar_obstaculos(&mut tablero);
        tablero
    }

    pub fn imprimir_tablero(&self, tablero: &Vec<Vec<String>>) {
        for row in tablero {
            for element in row {
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
        let mut tablero: Vec<Vec<String>> = self.posicionar_elementos_en_tablero();

        self.detonar_bomba(&mut tablero, coordenada);
        println!("Tablero inicial: ");
        self.imprimir_tablero(&tablero);

        let tablero_final = self.posicionar_elementos_en_tablero();
        println!("Tablero final: ");
        self.imprimir_tablero(&tablero_final);

        let mut output_file = File::create(output_path)?;

        for row in &tablero_final {
            let row_str: String = row
                .iter()
                .map(|c| c.to_string())
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
                println!("Vida inicial: {}", self.enemigos[i].vida);
                self.enemigos[i].vida -= 1;
                println!("Vida actual: {}", self.enemigos[i].vida);
                if self.enemigos[i].vida == 0 {
                    self.enemigos.swap_remove(i);
                    return true;
                }
            }
        }

        false
    }

    fn evaluar_casillero(&mut self,coordenada : &mut Coordenada, tablero : &mut Vec<Vec<String>>,tipo: &bomba::TipoDeBomba,mut i : i8) {
        println!("evaluar casillero en ({},{})",coordenada.x,coordenada.y);
        if coordenada.x >= 0 && coordenada.y >= 0  && coordenada.x < self.dimension && coordenada.y < self.dimension {
            let casillero : &str = &tablero[coordenada.x as usize][coordenada.y as usize];
            match casillero {
                a if a.starts_with(ENEMIGO) => {
                    self.eliminar_enemigo(*coordenada);
                }
                b if b.starts_with(BOMBA_DE_TRANSPASO) || b.starts_with(BOMBA_NORMAL) => {
                    self.detonar_bomba(tablero, *coordenada);
                }
                c if c.starts_with(DESVIO) => {
                    //Not yet implemented
                }
                d if d.starts_with(ROCA) => {
                    if *tipo == TipoDeBomba::Normal {
                        coordenada.x = -1;
                        coordenada.y = -1;
                        println!("Las bombas normales no pueden atravesar rocas.");
                         // Detenerse si es una roca y la bomba es normal
                    }
                }
                e if e.starts_with(PARED) => {
                    coordenada.x = -1;
                    coordenada.y = -1;
                    println!("Ninguna pared puede ser atravesada.");
                     // Detenerse si es una pared
                }
                _ => {} // Otros casos
            }
        }
    }

    fn evaluar_arriba(&mut self,coordenada : &Coordenada, alcance : &i8, tipo : &bomba::TipoDeBomba, tablero : &mut Vec<Vec<String>>, mut i : i8) {
        println!("Evaluando arriba la coordenada: ({},{})",coordenada.x,coordenada.y);
        if coordenada.x > -1 {
            while &i <= alcance {
                let mut coordenada_a_evaluar = Coordenada::new(coordenada.x-i,coordenada.y);
                println!("Coordenada de arriba: ({},{})",coordenada_a_evaluar.x,coordenada_a_evaluar.y);
                self.evaluar_casillero(&mut coordenada_a_evaluar, tablero, tipo, i);
                i += 1;
            }
        }
        println!("Fin de evaluación arriba");
    }

    fn evaluar_abajo(&mut self,coordenada : &Coordenada, alcance : &i8, tipo : &bomba::TipoDeBomba, tablero : &mut Vec<Vec<String>>, mut i : i8) {
        println!("Evaluando abajo la coordenada: ({},{})",coordenada.x,coordenada.y);
        if coordenada.x > -1 {
            while &i <= alcance {
                let mut coordenada_a_evaluar : Coordenada = Coordenada::new(coordenada.x+i,coordenada.y);
                println!("Coordenada de abajo: ({},{})",coordenada_a_evaluar.x,coordenada_a_evaluar.y);
                self.evaluar_casillero(&mut coordenada_a_evaluar, tablero, tipo, i);
                i += 1;
            }
        }
    }

    fn evaluar_izquierda(&mut self,coordenada : &Coordenada, alcance : &i8, tipo : &bomba::TipoDeBomba, tablero : &mut Vec<Vec<String>>, mut i : i8) {
        if coordenada.x > -1 {
            while &i <= alcance {
                let mut coordenada_a_evaluar : Coordenada = Coordenada::new(coordenada.x,coordenada.y-i);
                self.evaluar_casillero(&mut coordenada_a_evaluar, tablero, tipo,i);
                i += 1;
            }
        }
    }

    fn evaluar_derecha(&mut self,coordenada : &Coordenada, alcance : &i8, tipo : &bomba::TipoDeBomba, tablero : &mut Vec<Vec<String>>, mut i : i8) {
        if coordenada.x > -1 {
            while &i <= alcance {
                let mut coordenada_a_evaluar : Coordenada = Coordenada::new(coordenada.x,coordenada.y+i);
                self.evaluar_casillero(&mut coordenada_a_evaluar, tablero, tipo,i);
                i += 1;
            }
        }
    }

    fn funcion_bomba(&mut self, bomba: &Bomba, tablero: &mut Vec<Vec<String>>) {
        println!("funcion bomba en ({},{})",bomba.coordenada.x,bomba.coordenada.y);
        self.evaluar_arriba(&bomba.coordenada,&bomba.alcance,&bomba.tipo,tablero,1);
        self.evaluar_abajo(&bomba.coordenada,&bomba.alcance,&bomba.tipo,tablero,1);
        self.evaluar_izquierda(&bomba.coordenada,&bomba.alcance,&bomba.tipo,tablero,1);
        self.evaluar_derecha(&bomba.coordenada,&bomba.alcance,&bomba.tipo,tablero,1);
    }

    /*fn funcion_bomba(&mut self, bomba: &Bomba, tablero: &mut Vec<Vec<String>>) {
        let alcance = bomba.alcance;
        let coordenada = bomba.coordenada;

        let direcciones = [(0, 1), (1, 0), (0, -1), (-1, 0)]; // Direcciones: Derecha, Abajo, Izquierda, Arriba

        for &(dx, dy) in &direcciones {
            let mut incremento = 1;
            let mut x = coordenada.x + dx;
            let mut y = coordenada.y + dy;

            while incremento <= alcance {
                if x >= 0 && y >= 0 && x < tablero.len() as i8 && y < tablero[0].len() as i8 {
                    let casilla = &tablero[x as usize][y as usize];

                    match casilla {
                        a if a.starts_with(ENEMIGO) => {
                            let coordenada_enemigo = Coordenada::new(x, y);
                            self.eliminar_enemigo(coordenada_enemigo);
                        }
                        b if b.starts_with(BOMBA_DE_TRANSPASO) || b.starts_with(BOMBA_NORMAL) => {
                            let coordenada_bomba = Coordenada::new(x, y);
                            self.detonar_bomba(tablero, coordenada_bomba);
                        }
                        c if c.starts_with(DESVIO) => {
                            //Not yet implemented
                        }
                        d if d.starts_with(ROCA) => {
                            if bomba.tipo == TipoDeBomba::Normal {
                                println!("Las bombas normales no pueden atravesar rocas.");
                                break; // Detenerse si es una roca y la bomba es normal
                            }
                        }
                        e if e.starts_with(PARED) => {
                            println!("Ninguna pared puede ser atravesada.");
                            break; // Detenerse si es una pared
                        }
                        _ => {} // Otros casos
                    }
                } else {
                    break;
                }

                incremento += 1;
                x += dx;
                y += dy;
            }
        }
    }*/

    fn detonar_bomba(&mut self, tablero: &mut Vec<Vec<String>>, coordenada: Coordenada) {
        let mut bomba_index: Option<usize> = None;

        for (i, bomba) in self.bombas.iter().enumerate().rev() {
            if bomba.coordenada.is_equal_to(&coordenada) {
                bomba_index = Some(i);
                break;
            }
        }

        if let Some(i) = bomba_index {
            if !self.bombas[i].detonada {
                self.bombas[i].detonar();
                let bomba = self.bombas[i].clone();

                self.funcion_bomba(&self.bombas[i].clone(), tablero);

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
