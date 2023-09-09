use crate::juego::coordenada::Coordenada;

pub struct Desvio {
    pub coordenada: Coordenada,
    pub direccion: String,
    pub id: String,
}

impl Desvio {
    pub fn new(coordenada: Coordenada, direccion: String, id: String) -> Desvio {
        Desvio {
            id,
            direccion,
            coordenada,
        }
    }
}
