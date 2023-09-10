use crate::juego::coordenada::Coordenada;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TipoDeBomba {
    Normal,
    Traspaso,
}

#[derive(Clone)]
pub struct Bomba {
    pub id: String,
    pub alcance: i8,
    pub tipo: TipoDeBomba,
    pub coordenada: Coordenada,
    pub detonada: bool,
}

impl Bomba {
    pub fn new(coordenada: Coordenada, alcance: i8, tipo: TipoDeBomba, id: String) -> Bomba {
        let detonada: bool = false;
        Bomba {
            detonada,
            id,
            alcance,
            tipo,
            coordenada,
        }
    }

    pub fn detonar(&mut self) {
        self.detonada = true;
    }
}
