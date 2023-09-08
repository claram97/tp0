use crate::juego::coordenada::Coordenada;

#[derive(Debug, PartialEq, Eq)]
pub enum TipoDeBomba {
    Normal,
    Traspaso 
}

pub struct Bomba {
    id: char,
    alcance: i8,
    tipo: TipoDeBomba,
    pub coordenada: Coordenada,
}

impl Bomba {

    pub fn new(coordenada: Coordenada, alcance: i8, tipo: TipoDeBomba) -> Bomba {
        let id = if tipo == TipoDeBomba::Normal {
            'B'
        } else {
            'S'
        };
    
        Bomba {
            id,
            alcance,
            tipo,
            coordenada,
        }
    }
    
}