use crate::juego::coordenada::Coordenada;
const BOMBA_NORMAL : char = 'B';
const BOMBA_DE_TRANSPASO : char = 'S';

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TipoDeBomba {
    Normal,
    Traspaso 
}

pub struct Bomba {
    pub id: char,
    pub alcance: i8,
    pub tipo: TipoDeBomba,
    pub coordenada: Coordenada,
    pub detonada: bool,
}

impl Bomba {

    pub fn new(coordenada: Coordenada, alcance: i8, tipo: TipoDeBomba) -> Bomba {
        let id = if tipo == TipoDeBomba::Normal {
            BOMBA_NORMAL
        } else {
            BOMBA_DE_TRANSPASO
        };
        let detonada : bool = false;
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

impl Clone for Bomba {
    fn clone(&self) -> Self {
        Bomba {
            detonada: self.detonada,
            id: self.id,
            alcance: self.alcance,
            tipo: self.tipo.clone(), // Clonar el campo tipo si es clonable
            coordenada: self.coordenada, // Clonar el campo coordenada si es clonable
        }
    }
}