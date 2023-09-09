use crate::juego::coordenada::Coordenada;

pub struct Enemigo {
    pub id: String,
    pub vida: i8,
    pub coordenada: Coordenada,
}

impl Enemigo {
    pub fn new(coordenada: Coordenada, vida: i8) -> Enemigo {
        let id : String = "F".to_string();
        Enemigo {
            id : id,
            vida,
            coordenada,
        }
    }
}
