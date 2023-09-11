use crate::estructuras_juego::coordenada::Coordenada;

/// Enumeración que representa los tipos de obstáculos en el juego.
#[derive(Debug, PartialEq, Eq)]
pub enum TipoDeObstaculo {
    /// Representa un obstáculo de tipo "Pared".
    Pared,

    /// Representa un obstáculo de tipo "Roca".
    Roca,
}

/// Representa un obstáculo en el tablero del juego con una coordenada, un tipo y un identificador.
pub struct Obstaculo {
    /// Coordenada en la que se encuentra el obstáculo en el tablero.
    pub coordenada: Coordenada,

    /// Tipo de obstáculo, que puede ser "Pared" o "Roca".
    pub tipo: TipoDeObstaculo,

    /// Identificador del obstáculo (por ejemplo, "W" para Pared, "R" para Roca).
    pub id: String,
}

impl Obstaculo {
    /// Crea un nuevo obstáculo con el tipo y la coordenada especificados.
    ///
    /// # Argumentos
    ///
    /// * `tipo`: Tipo de obstáculo, que puede ser "Pared" o "Roca".
    /// * `coordenada`: Coordenada en el tablero donde se encuentra el obstáculo.
    ///
    /*/// # Ejemplo
    ///
    /// ```rust
    /// use mi_modulo::{Obstaculo, TipoDeObstaculo, Coordenada};
    ///
    /// let coordenada = Coordenada::new(2, 3);
    /// let pared = Obstaculo::new(TipoDeObstaculo::Pared, coordenada);
    /// let roca = Obstaculo::new(TipoDeObstaculo::Roca, coordenada);
    /// ```*/
    pub fn new(tipo: TipoDeObstaculo, coordenada: Coordenada) -> Obstaculo {
        let id = if tipo == TipoDeObstaculo::Pared {
            "W".to_string()
        } else {
            "R".to_string()
        };
        Obstaculo {
            id,
            tipo,
            coordenada,
        }
    }
}
