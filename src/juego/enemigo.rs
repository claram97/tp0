use crate::juego::coordenada::Coordenada;

pub struct Enemigo {
    pub id: String,
    pub vida: i8,
    pub coordenada: Coordenada,
    pub bombas_que_lo_impactaron: Vec<Coordenada>,
}

impl Enemigo {
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

    pub fn actualizar_lista_de_bombas(&mut self, coordenada_bomba: Coordenada) {
        self.bombas_que_lo_impactaron.push(coordenada_bomba);
    }
}
