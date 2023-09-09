#[derive(Debug, Copy, Clone)]
pub struct Coordenada {
    pub x: i8,
    pub y: i8
}

impl Coordenada {
    pub fn new(x: i8, y: i8) -> Coordenada {
        Coordenada { x, y }
    }

    pub fn is_equal_to(&self,coordenada: &Coordenada) -> bool{
        self.x == coordenada.x && self.y == coordenada.y
    }

}
