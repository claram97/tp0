
use crate::juego::coordenada::Coordenada;

pub struct Enemigo {
    id: char,
    pub vida: i8,
    pub coordenada: Coordenada,
}

impl Enemigo {
    pub fn impactar_con_rafaga() {
        println!("Usted ha impactado al enemigo!");
    }

    pub fn new(coordenada: Coordenada, vida: i8) -> Enemigo {
        Enemigo {
            id: 'F',
            vida,
            coordenada,
        }
    }

}