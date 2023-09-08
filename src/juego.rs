pub mod coordenada;
pub mod enemigo;
pub mod obstaculo;
pub mod bomba;
pub mod desvio;
use obstaculo::Obstaculo;
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

    pub fn inicializar_dimension(&mut self,dimension: i8) {
        self.dimension_x = dimension;
        self.dimension_y = dimension;
    }

    pub fn inicializar_desvio(&mut self,coordenada: Coordenada, direccion: char) {
        println!("Inicializar desvío hacia la {}",direccion);
        let desvio: Desvio = Desvio::new(coordenada, direccion);
        self.desvios.push(desvio);
    }
    
    pub fn inicializar_enemigo(&mut self, coordenada: Coordenada,vida: i8) {
        println!("inicializar enemigo con vida {}",vida);
        let enemigo: Enemigo = Enemigo::new(coordenada,vida);
        self.enemigos.push(enemigo);
    }

    pub fn inicializar_roca(&mut self, coordenada: Coordenada) {
        println!("inicializar roca");
        let roca: Obstaculo = Obstaculo::new(obstaculo::TipoDeObstaculo::Roca,coordenada);
        self.obstaculos.push(roca);
    }

    pub fn inicializar_bomba(&mut self, coordenada: Coordenada,alcance: i8,tipo: bomba::TipoDeBomba) {
        println!("inicializar bomba");
        let bomba: Bomba = Bomba::new(coordenada,alcance,tipo);
        self.bombas.push(bomba);
    }

    pub fn inicializar_pared(&mut self, coordenada: Coordenada) {
        println!("inicializar pared");
        let pared: Obstaculo = Obstaculo::new(obstaculo::TipoDeObstaculo::Pared,coordenada);
        self.obstaculos.push(pared);
    }

    /*fn imprimir_tablero(dimension_x: i8, dimension_y: i8, output_path: String) {
        let tablero: Vec<Vec<char>> = vec![vec![' '; dimension_y as usize]; dimension_x as usize];
        for row in &tablero {
            for &element in row {
                print!("{} ", element);
            }
            println!();
        }
        println!("output path: {}",output_path);
    }*/

    fn eliminar_enemigo(&mut self,coordenada: Coordenada) -> bool {
        println!("Eliminar enemigo");
        let mut i: usize = self.enemigos.len();
        let mut found: bool = false;
        while i > 0 && !found {
            if self.enemigos[i].coordenada.is_equal_to(&coordenada) {
                if self.enemigos[i].vida > 0 {
                    self.enemigos[i].vida -= 1;
                    if self.enemigos[i].vida == 0 {
                        return true;
                    }
                }
                found = true;
            }
            i -= 1;
        }
        false
    }

    fn detonar_bomba(&mut self,coordenada: Coordenada) -> bool {
        println!("Bomba detonada");
        let mut i: usize = self.bombas.len();
        let mut found: bool = false;
        while i > 0 && !found {
            if self.bombas[i].coordenada.is_equal_to(&coordenada) {
                //Implementar funcionalidad de la bomba
                found = true;
                return true;
            }
            i -= 1;
        }
        false
    }

}