use crate::estructuras_juego::coordenada::Coordenada;

/// Representa un enemigo en el juego con una vida determinada que puede ser afectado por bombas.
pub struct Enemigo {
    /// El identificador único del enemigo.
    pub id: String,

    /// La cantidad de vida del enemigo, que determina cuántas bombas puede resistir.
    pub vida: i8,

    /// La coordenada en la que se encuentra el enemigo en el tablero.
    pub coordenada: Coordenada,

    /// Lista de coordenadas de bombas que han impactado a este enemigo.
    pub bombas_que_lo_impactaron: Vec<Coordenada>,
}

impl Enemigo {
    /// Crea un nuevo objeto de enemigo con la coordenada y vida especificadas.
    ///
    /// # Argumentos
    ///
    /// * `coordenada`: La coordenada en la que se encuentra el enemigo en el tablero.
    /// * `vida`: La cantidad de vida del enemigo, que determina cuántas bombas puede resistir.
    ///
    pub fn new(coordenada: Coordenada, vida: i8) -> Enemigo {
        let id: String = "F".to_string();
        let bombas_que_lo_impactaron = Vec::new(); // Crea un vector vacío de bombas

        Enemigo {
            id,
            vida,
            coordenada,
            bombas_que_lo_impactaron,
        }
    }

    /// Actualiza la lista de coordenadas de bombas que han impactado a este enemigo.
    ///
    /// # Argumentos
    ///
    /// * `coordenada_bomba`: La coordenada de la bomba que impactó al enemigo.
    ///
    pub fn actualizar_lista_de_bombas(&mut self, coordenada_bomba: Coordenada) {
        self.bombas_que_lo_impactaron.push(coordenada_bomba);
    }
}
