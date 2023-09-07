
use crate::juego::coordenada::Coordenada;

pub struct Enemigo {
    id: char,
    vida: i8,
    coordenada: Coordenada,
}

impl Enemigo {
    fn impactar_con_rafaga() {
        println!("Usted ha impactado al enemigo!");
    }
}