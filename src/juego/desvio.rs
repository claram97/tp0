use crate::juego::coordenada::Coordenada;

pub struct Desvio {
    coordenada: Coordenada,
    direccion: char,
}

impl Desvio {
    pub fn new(coordenada: Coordenada, direccion: char) -> Desvio {
        Desvio {
            direccion,
            coordenada,
        }
    }

}