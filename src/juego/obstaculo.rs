use crate::juego::coordenada::Coordenada;

#[derive(Debug, PartialEq, Eq)]
pub enum TipoDeObstaculo {
    Pared,
    Roca,
}

pub struct Obstaculo {
    pub coordenada: Coordenada,
    pub tipo: TipoDeObstaculo,
    pub id: String,
}

impl Obstaculo {
    pub fn new(tipo: TipoDeObstaculo, coordenada: Coordenada) -> Obstaculo {
        let id = if tipo == TipoDeObstaculo::Pared {
            "W".to_string()
        } else {
            "R".to_string()
        };
        Obstaculo {
            id,
            tipo,
            coordenada,
        }
    }
}
