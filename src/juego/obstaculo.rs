use crate::juego::coordenada::Coordenada;
const PARED : char = 'W';
const ROCA : char = 'R';

#[derive(Debug, PartialEq, Eq)]
pub enum TipoDeObstaculo {
    Pared,
    Roca
}

pub struct Obstaculo {
    pub coordenada: Coordenada,
    pub tipo: TipoDeObstaculo,
    pub id: char
}

impl Obstaculo {
    pub fn new(tipo: TipoDeObstaculo, coordenada: Coordenada) -> Obstaculo {
        let id = if tipo == TipoDeObstaculo::Pared {
            PARED
        } else {
            ROCA
        };
        Obstaculo {
            id,
            tipo,
            coordenada,
        }
    }
}