use crate::juego::coordenada::Coordenada;

enum TipoDeBomba {
    Normal,
    Traspaso 
}

pub struct Bomba {
    id: char,
    alcance: i8,
    tipo: TipoDeBomba,
    coordenada: Coordenada,
}

impl Bomba {
    fn detonar() {
        println!("Bomba detonada");
    }
}