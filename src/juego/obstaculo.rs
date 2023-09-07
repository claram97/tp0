use crate::juego::coordenada::Coordenada;

enum TipoDeObstaculo {
    Pared,
    Roca
}

pub struct Obstaculo {
    coordenada: Coordenada,
    tipo: TipoDeObstaculo
}