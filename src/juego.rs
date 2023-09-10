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
const DESVIO_ARRIBA: &str = "DU";
const DESVIO_ABAJO: &str = "DD";
const DESVIO_IZQUIERDA: &str = "DL";
const DESVIO_DERECHA: &str = "DR";

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
            let id = (enemigo.id).to_string() + &enemigo.vida.to_string();
            tablero[enemigo.coordenada.x as usize][enemigo.coordenada.y as usize] = id;
        }
    }

    fn posicionar_obstaculos(&self, tablero: &mut [Vec<String>]) {
        for obstaculo in &self.obstaculos {
            let id: String = "".to_string() + &obstaculo.id;
            tablero[obstaculo.coordenada.x as usize][obstaculo.coordenada.y as usize] = id;
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

    fn imprimir_tablero_en_archivo(&self, output_file: &mut File, tablero : &Vec<Vec<String>>) -> std::io::Result<()> {
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

    pub fn realizar_jugada(
        &mut self,
        mut output_file : &mut File,
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

        self.imprimir_tablero_en_archivo(&mut output_file,&tablero_final)?;
        /*for row in &tablero_final {
            let row_str: String = row
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<_>>()
                .join(" ");
            writeln!(output_file, "{}", row_str)?;
        }*/
        println!();
        println!("***************");
        println!();
        Ok(())
    }

    fn eliminar_enemigo(&mut self, coordenada: Coordenada, coordenada_bomba: Coordenada) {
        println!("Eliminar enemigo");

        if let Some(i) = self
            .enemigos
            .iter_mut()
            .position(|enemigo| enemigo.coordenada.is_equal_to(&coordenada))
        {
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
                    println!("Se encontró un enemigo");
                    self.eliminar_enemigo(*coordenada, coordenada_original);
                }
                b if b.starts_with(BOMBA_DE_TRANSPASO) || b.starts_with(BOMBA_NORMAL) => {
                    println!("Se encontró una bomba: próxima a explotar.");
                    self.detonar_bomba(tablero, *coordenada);
                }
                c if c.starts_with(DESVIO) => {
                    //println!("Desvío encontrado. Alcance actual: {}",i);
                    i += 1;
                    //println!("Incrementado alcance al {}... Listo!",i);
                    if c == DESVIO_ARRIBA {
                        println!("Desviando hacia arriba :)");
                        self.evaluar_arriba(coordenada, tablero, i, bomba);
                    } else if c == DESVIO_ABAJO {
                        println!("Desviando hacia abajo :)");
                        self.evaluar_abajo(coordenada, tablero, i, bomba);
                    } else if c == DESVIO_DERECHA {
                        println!("Desviando hacia derecha :)");
                        self.evaluar_izquierda(coordenada, tablero, i, bomba);
                    } else if c == DESVIO_IZQUIERDA {
                        println!("Desviando hacia izquierda :)");
                        self.evaluar_derecha(coordenada, tablero, i, bomba);
                    }
                }
                d if d.starts_with(ROCA) => {
                    if bomba.tipo == TipoDeBomba::Normal {
                        coordenada.x = -1;
                        coordenada.y = -1;
                        println!("Las bombas normales no pueden atravesar rocas.");
                    } else {
                        println!("Super bomba! :D");
                    }
                }
                e if e.starts_with(PARED) => {
                    coordenada.x = -1;
                    coordenada.y = -1;
                    println!("Ninguna pared puede ser atravesada.");
                }
                _ => {}
            }
        }
    }

    fn evaluar_arriba(
        &mut self,
        coordenada: &Coordenada,
        tablero: &mut Vec<Vec<String>>,
        mut i: i8,
        bomba: &Bomba,
    ) {
        let mut j: i8 = 1;
        while i <= bomba.alcance {
            println!("Evaluando coordenada: ({},{})", coordenada.x, coordenada.y);
            let mut coordenada_a_evaluar = Coordenada::new(coordenada.x - j, coordenada.y);
            println!(
                "Evaluando arriba: ({},{})",
                coordenada_a_evaluar.x, coordenada_a_evaluar.y
            );
            self.evaluar_casillero(&mut coordenada_a_evaluar, tablero, i, bomba, *coordenada);
            if coordenada_a_evaluar.x == -1 && coordenada_a_evaluar.y == -1 {
                return;
            }
            j += 1;
            i += 1;
        }
    }

    fn evaluar_abajo(
        &mut self,
        coordenada: &Coordenada,
        tablero: &mut Vec<Vec<String>>,
        mut i: i8,
        bomba: &Bomba,
    ) {
        let mut j: i8 = 1;
        while i <= bomba.alcance {
            println!("Evaluando coordenada: ({},{})", coordenada.x, coordenada.y);
            let mut coordenada_a_evaluar: Coordenada =
                Coordenada::new(coordenada.x + j, coordenada.y);
            println!(
                "Evaluando abajo: ({},{})",
                coordenada_a_evaluar.x, coordenada_a_evaluar.y
            );
            self.evaluar_casillero(&mut coordenada_a_evaluar, tablero, i, bomba, *coordenada);
            if coordenada_a_evaluar.x == -1 && coordenada_a_evaluar.y == -1 {
                return;
            }
            j += 1;
            i += 1;
        }
    }

    fn evaluar_izquierda(
        &mut self,
        coordenada: &Coordenada,
        tablero: &mut Vec<Vec<String>>,
        mut i: i8,
        bomba: &Bomba,
    ) {
        let mut j: i8 = 1;
        while i <= bomba.alcance {
            println!("Evaluando coordenada: ({},{})", coordenada.x, coordenada.y);
            let mut coordenada_a_evaluar: Coordenada =
                Coordenada::new(coordenada.x, coordenada.y - j);
            println!(
                "Evaluando izquierda: ({},{})",
                coordenada_a_evaluar.x, coordenada_a_evaluar.y
            );
            self.evaluar_casillero(&mut coordenada_a_evaluar, tablero, i, bomba, *coordenada);
            if coordenada_a_evaluar.x == -1 && coordenada_a_evaluar.y == -1 {
                return;
            }
            i += 1;
            j += 1;
        }
    }

    fn evaluar_derecha(
        &mut self,
        coordenada: &Coordenada,
        tablero: &mut Vec<Vec<String>>,
        mut i: i8,
        bomba: &Bomba,
    ) {
        let mut j: i8 = 1;
        while i <= bomba.alcance {
            println!("Evaluando coordenada: ({},{})", coordenada.x, coordenada.y);
            let mut coordenada_a_evaluar: Coordenada =
                Coordenada::new(coordenada.x, coordenada.y + j);
            println!(
                "Evaluando derecha: ({},{})",
                coordenada_a_evaluar.x, coordenada_a_evaluar.y
            );
            self.evaluar_casillero(&mut coordenada_a_evaluar, tablero, i, bomba, *coordenada);
            if coordenada_a_evaluar.x == -1 && coordenada_a_evaluar.y == -1 {
                return;
            }
            i += 1;
            j += 1;
        }
    }

    fn funcion_bomba(&mut self, bomba: &Bomba, tablero: &mut Vec<Vec<String>>) {
        self.evaluar_arriba(&bomba.coordenada, tablero, 1, bomba);
        self.evaluar_abajo(&bomba.coordenada, tablero, 1, bomba);
        self.evaluar_izquierda(&bomba.coordenada, tablero, 1, bomba);
        self.evaluar_derecha(&bomba.coordenada, tablero, 1, bomba);
    }

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

                return;
            } else {
                return;
            }
        }

        println!("Bomba no encontrada");
    }
}

    /*
    fn eliminar_enemigo(&mut self, coordenada: Coordenada, impactado : bool) {
        println!("Eliminar enemigo");

        if !impactado {
            if let Some(i) = self
                .enemigos
                .iter_mut()
                .position(|enemigo| enemigo.coordenada.is_equal_to(&coordenada))
            {
                if self.enemigos[i].vida > 0 {
                    self.enemigos[i].vida -= 1;
                    if self.enemigos[i].vida == 0 {
                        self.enemigos.swap_remove(i);
                    }
                }
            }
        }
    } */

