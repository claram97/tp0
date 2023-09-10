use std::cell::RefCell;

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
    pub ha_impactado: RefCell<bool>,
}

impl Bomba {
    pub fn new(coordenada: Coordenada, alcance: i8, tipo: TipoDeBomba, id: String) -> Bomba {
        let detonada: bool = false;
        let ha_impactado : bool = false;
        Bomba {
            ha_impactado: RefCell::new(ha_impactado),
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
/*
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
} */
