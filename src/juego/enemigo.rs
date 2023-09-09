
use crate::juego::coordenada::Coordenada;

pub struct Enemigo {
    pub id: char,
    pub vida: i8,
    pub coordenada: Coordenada,
}

impl Enemigo {
    pub fn new(coordenada: Coordenada, vida: i8) -> Enemigo {
        Enemigo {
            id: 'F',
            vida,
            coordenada,
        }
    }
}