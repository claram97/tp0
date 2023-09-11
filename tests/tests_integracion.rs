/*use tp0::juego::{eliminar_enemigo,inicializar_enemigo};

#[test]
pub fn eliminar_enemigo_inexistente() {
    let juego : Juego = Juego::new();
    juego.inicializar_dimension(8);
    let coordenada_enemigo : Coordenada = Coordenada::new(3,5);
    let coordenada_bomba : Coordenada = Coordenada::new(3,4);
    eliminar_enemigo(coordenada_enemigo, coordenada_bomba);
}

#[test]
pub fn eliminar_enemigo_del_mapa() {
    let juego : Juego = Juego::new();
    juego.inicializar_dimension(8);
    let coordenada_enemigo : Coordenada = Coordenada::new(3,5);
    let coordenada_bomba : Coordenada = Coordenada::new(3,4);
    juego.inicializar_enemigo(coordenada: coordenada_enemigo, vida: 1);
    eliminar_enemigo(coordenada_enemigo, coordenada_bomba);
}

#[test]
pub fn eliminar_enemigo_con_vida_mayor_a_cero() {
    let juego : Juego = Juego::new();
    juego.inicializar_dimension(8);
    let coordenada_enemigo : Coordenada = Coordenada::new(3,5);
    let coordenada_bomba : Coordenada = Coordenada::new(3,4);
    juego.inicializar_enemigo(coordenada: coordenada_enemigo, vida: 5);
    eliminar_enemigo(coordenada_enemigo, coordenada_bomba);
}

#[test]
pub fn enemigo_no_recibe_danio_de_la_misma_bomba_dos_veces() {
    let juego : Juego = Juego::new();
    juego.inicializar_dimension(8);
    let coordenada_enemigo : Coordenada = Coordenada::new(3,5);
    let coordenada_bomba : Coordenada = Coordenada::new(3,4);
    juego.inicializar_enemigo(coordenada: coordenada_enemigo, vida: 5);
    juego.inicializar_bomba(coordenada_bomba,5,TipoDeBomba::Normal,"B5");
    eliminar_enemigo(coordenada_enemigo, coordenada_bomba);
    eliminar_enemigo(coordenada_enemigo,coordenada_bomba);
}

#[test]
pub fn evaluar_finaliza_al_encontrar_una_pared() {
    let juego : Juego = Juego::new();
    juego.inicializar_dimension(4);
    let tablero: Vec<Vec<String>> = vec![
        vec!["_".to_string(), "_".to_string(), "_".to_string(), "_".to_string()],
        vec!["_".to_string(), "W".to_string(), "_".to_string(), "_".to_string()],
        vec!["_".to_string(), "_".to_string(), "_".to_string(), "_".to_string()],
        vec!["_".to_string(), "B5".to_string(), "_".to_string(), "_".to_string()],
    ];
    let coordenada_bomba : Coordenada = Coordenada::new(3,1);
    let coordenada_pared : Coordenada = Coordenada::new(1,1);
    let bomba : Bomba = Bomba::new(coordenada_bomba, 5, TipoDeBomba::Normal, "B5");
    juego.inicializar_pared(coordenada_pared);
    juego.inicializar_bomba(coordenada_bomba);
    juego.evaluar_arriba();
    let mut i: i8 = 1;
    juego.evaluar_arriba(coordenada: coordenada_bomba, tablero: tablero, i: i, bomba: &bomba);
}

#[test]
pub fn evaluar_no_finaliza_al_encontrar_una_pared_con_super_bomba() {
    let juego : Juego = Juego::new();
    juego.inicializar_dimension(7);
    let tablero: Vec<Vec<String>> = vec![
        vec!["_".to_string(), "_".to_string(), "_".to_string(), "_".to_string()],
        vec!["_".to_string(), "W".to_string(), "_".to_string(), "_".to_string()],
        vec!["_".to_string(), "_".to_string(), "_".to_string(), "_".to_string()],
        vec!["_".to_string(), "_".to_string(), "_".to_string(), "_".to_string()],
        vec!["_".to_string(), "_".to_string(), "_".to_string(), "_".to_string()],
        vec!["_".to_string(), "S5".to_string(), "_".to_string(), "_".to_string()],
        vec!["_".to_string(), "_".to_string(), "_".to_string(), "_".to_string()],
    ];
    let coordenada_bomba : Coordenada = Coordenada::new(5,1);
    let coordenada_pared : Coordenada = Coordenada::new(1,1);
    let bomba : Bomba = Bomba::new(coordenada_bomba, 5, TipoDeBomba::Traspaso, "S5");
    juego.inicializar_pared(coordenada_pared);
    juego.inicializar_bomba(coordenada_bomba);
    juego.evaluar_arriba();
    let mut i: i8 = 1;
    juego.evaluar_arriba(coordenada: coordenada_bomba, tablero: tablero, i: i, bomba: &bomba);
}*/