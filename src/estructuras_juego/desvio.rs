use crate::estructuras_juego::coordenada::Coordenada;

/// Representa un objeto de desvío en el juego, que puede alterar la trayectoria de una bomba.
pub struct Desvio {
    /// La coordenada en la que se encuentra el desvío en el tablero.
    pub coordenada: Coordenada,

    /// La dirección en la que el desvío redirige las bombas.
    pub direccion: String,

    /// El identificador único del desvío.
    pub id: String,
}

impl Desvio {
    /// Crea un nuevo objeto de desvío con la coordenada, dirección e ID especificados.
    ///
    /// # Argumentos
    ///
    /// * `coordenada`: La coordenada en la que se encuentra el desvío en el tablero.
    /// * `direccion`: La dirección en la que el desvío redirige las bombas.
    /// * `id`: El identificador único del desvío.
    ///
    pub fn new(coordenada: Coordenada, direccion: String, id: String) -> Desvio {
        Desvio {
            id,
            direccion,
            coordenada,
        }
    }
}
