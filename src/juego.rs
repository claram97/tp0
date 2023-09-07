mod coordenada;
mod enemigo;
mod obstaculo;
mod bomba;
mod desvio;
use obstaculo::Obstaculo;
use enemigo::Enemigo;
use bomba::Bomba;
use desvio::Desvio;
pub struct Juego {
    dimension_x: i8,
    dimension_y: i8,
    enemigos: Vec<Enemigo>,
    obstaculos: Vec<Obstaculo>,
    bombas: Vec<Bomba>,
    desvios: Vec<Desvio>
}

impl Juego {
    fn inicializar_enemigo() {

    }

    fn inicializar_obstaculo() {

    }

    fn inicializar_bomba() {

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