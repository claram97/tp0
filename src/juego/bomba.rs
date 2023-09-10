use crate::juego::coordenada::Coordenada;

/// Enumeración que representa los tipos de bombas en el juego.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TipoDeBomba {
    /// Representa una bomba de tipo "Normal".
    Normal,

    /// Representa una bomba de tipo "Traspaso".
    Traspaso,
}

/// Representa una bomba en el juego con un identificador, alcance, tipo, coordenada y estado de detonación.
#[derive(Clone)]
pub struct Bomba {
    /// Identificador único de la bomba.
    pub id: String,

    /// Alcance de la explosión de la bomba.
    pub alcance: i8,

    /// Tipo de bomba, que puede ser "Normal" o "Traspaso".
    pub tipo: TipoDeBomba,

    /// Coordenada en la que se encuentra la bomba en el tablero.
    pub coordenada: Coordenada,

    /// Estado de detonación de la bomba (true si ha detonado, false si no).
    pub detonada: bool,
}

impl Bomba {
    /// Crea una nueva bomba con el tipo, alcance, identificador y coordenada especificados.
    ///
    /// # Argumentos
    ///
    /// * `tipo`: Tipo de bomba, que puede ser "Normal" o "Traspaso".
    /// * `alcance`: Alcance de la explosión de la bomba.
    /// * `id`: Identificador único de la bomba.
    /// * `coordenada`: Coordenada en el tablero donde se encuentra la bomba.
    ///
    /// # Ejemplo
    ///
    /// ```rust
    /// use mi_modulo::{Bomba, TipoDeBomba, Coordenada};
    ///
    /// let coordenada = Coordenada::new(2, 3);
    /// let bomba_normal = Bomba::new(TipoDeBomba::Normal, 3, "B1".to_string(), coordenada);
    /// let bomba_traspaso = Bomba::new(TipoDeBomba::Traspaso, 2, "B2".to_string(), coordenada);
    /// ```
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

    /// Detona la bomba, cambiando su estado de detonación a true.
    pub fn detonar(&mut self) {
        self.detonada = true;
    }
}
