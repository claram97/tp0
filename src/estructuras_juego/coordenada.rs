/// Representa una coordenada en el tablero del juego con valores `x` e `y`.
#[derive(Debug, Copy, Clone)]
pub struct Coordenada {
    /// Valor de la coordenada en el eje X.
    pub x: i8,

    /// Valor de la coordenada en el eje Y.
    pub y: i8,
}

impl Coordenada {
    /// Crea una nueva coordenada con valores `x` e `y` especificados.
    ///
    /// # Argumentos
    ///
    /// * `x`: Valor de la coordenada en el eje X.
    /// * `y`: Valor de la coordenada en el eje Y.
    ///
    /*/// # Ejemplo
    ///
    /// ```rust
    /// use mi_modulo::Coordenada;
    ///
    /// let coordenada = Coordenada::new(2, 3);
    /// ```*/
    pub fn new(x: i8, y: i8) -> Coordenada {
        Coordenada { x, y }
    }

    /// Comprueba si esta coordenada es igual a otra coordenada dada.
    ///
    /// # Argumentos
    ///
    /// * `coordenada`: La coordenada con la que se va a comparar.
    ///
    /*/// # Ejemplo
    ///
    /// ```rust
    /// use mi_modulo::Coordenada;
    ///
    /// let coordenada1 = Coordenada::new(2, 3);
    /// let coordenada2 = Coordenada::new(2, 3);
    ///
    /// assert!(coordenada1.is_equal_to(&coordenada2));
    /// ```*/
    pub fn is_equal_to(&self, coordenada: &Coordenada) -> bool {
        self.x == coordenada.x && self.y == coordenada.y
    }
}
