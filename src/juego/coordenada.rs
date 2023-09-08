pub struct Coordenada {
    x: i8,
    y: i8
}

impl Coordenada {
    // Método new que toma dos argumentos para inicializar una Coordenada
    pub fn new(x: i8, y: i8) -> Coordenada {
        Coordenada { x, y }
    }
}
