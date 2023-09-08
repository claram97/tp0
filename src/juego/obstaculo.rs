use crate::juego::coordenada::Coordenada;

pub enum TipoDeObstaculo {
    Pared,
    Roca
}

pub struct Obstaculo {
    coordenada: Coordenada,
    tipo: TipoDeObstaculo
}

impl Obstaculo {
    pub fn new(tipo: TipoDeObstaculo, coordenada: Coordenada) -> Obstaculo {
        Obstaculo {
            tipo,
            coordenada,
        }
    }
}