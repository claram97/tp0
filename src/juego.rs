pub mod coordenada;
mod enemigo;
mod obstaculo;
mod bomba;
mod desvio;
use obstaculo::Obstaculo;
use obstaculo::TipoDeObstaculo;
use enemigo::Enemigo;
use bomba::Bomba;
use desvio::Desvio;
use coordenada::Coordenada;
pub struct Juego {
    dimension_x: i8,
    dimension_y: i8,
    enemigos: Vec<Enemigo>,
    obstaculos: Vec<Obstaculo>,
    bombas: Vec<Bomba>,
    desvios: Vec<Desvio>
}

impl Juego {
    pub fn new() -> Juego {
        Juego {
            dimension_x: 0,
            dimension_y: 0,
            enemigos: Vec::new(),
            obstaculos: Vec::new(),
            bombas: Vec::new(),
            desvios: Vec::new(),
        }
    }

    pub fn inicializar_enemigo(&mut self, coordenada: Coordenada) {
        println!("inicializar enemigo");
    }

    pub fn inicializar_roca(&mut self, coordenada: Coordenada) {
        println!("inicializar roca");
    }

    pub fn inicializar_bomba(&mut self, coordenada: Coordenada) {
        println!("inicializar bomba");
    }

    pub fn inicializar_pared(&mut self, coordenada: Coordenada) {
        println!("inicializar pared");
        let pared: Obstaculo = Obstaculo::new(obstaculo::TipoDeObstaculo::Pared,coordenada);
        self.obstaculos.push(pared);
    }

    fn imprimir_tablero(dimension_x: i8, dimension_y: i8, output_path: String) {
        let tablero: Vec<Vec<char>> = vec![vec![' '; dimension_y as usize]; dimension_x as usize];
        for row in &tablero {
            for &element in row {
                print!("{} ", element);
            }
            println!();
        }
        println!("output path: {}",output_path);
    }

    fn eliminar_enemigo() {
        println!("Enemigo muerto");
    }

    fn detonar_bomba() {
        println!("Bomba detonada");
    }

}