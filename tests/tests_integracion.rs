use tp0::estructuras_juego::bomba::*;
use tp0::estructuras_juego::coordenada::*;
use tp0::inicializar::{inicializar_coordenada_de_la_bomba, inicializar_juego, procesar_enemigo};
use tp0::juego::*;

#[test]
pub fn inicializar_juego_con_elementos_invalidos() {
    let mut juego: Juego = Juego::new();
    let coordenada_enemigo: Coordenada = Coordenada::new(3, 5);
    let resultado = inicializar_juego(coordenada_enemigo, "FB", &mut juego);
    assert!(resultado.is_err())
}

#[test]
pub fn inicializar_juego_con_elementos_correctos() {
    let mut juego: Juego = Juego::new();
    let coordenada_enemigo: Coordenada = Coordenada::new(3, 5);
    let resultado = inicializar_juego(coordenada_enemigo, "F6", &mut juego);
    assert!(resultado.is_ok())
}

#[test]
pub fn inicializar_coordenadas_erroneas() {
    let args: Vec<String> = vec![
        "string".to_string(),
        "maze.txt".to_string(),
        "output.txt".to_string(),
        "x".to_string(),
        "9".to_string(),
    ];
    let resultado = inicializar_coordenada_de_la_bomba(&args);
    assert!(resultado.is_err())
}

#[test]
pub fn inicializar_coordenadas_correctas() {
    let args: Vec<String> = vec![
        "string".to_string(),
        "maze.txt".to_string(),
        "output.txt".to_string(),
        "8".to_string(),
        "9".to_string(),
    ];
    let resultado = inicializar_coordenada_de_la_bomba(&args);
    assert!(resultado.is_ok())
}

#[test]
pub fn inicializar_enemigo_con_formato_correcto() {
    let mut juego: Juego = Juego::new();
    let coordenada_enemigo: Coordenada = Coordenada::new(3, 5);
    let resultado = procesar_enemigo("F5", coordenada_enemigo, &mut juego);
    assert!(resultado.is_ok());
}

#[test]
pub fn inicializar_enemigo_sin_vida() {
    let mut juego: Juego = Juego::new();
    let coordenada_enemigo: Coordenada = Coordenada::new(3, 5);
    let resultado = procesar_enemigo("F", coordenada_enemigo, &mut juego);
    assert!(resultado.is_err());
}

#[test]
pub fn inicializar_enemigo_con_formato_erroneo() {
    let mut juego: Juego = Juego::new();
    let coordenada_enemigo: Coordenada = Coordenada::new(3, 5);
    let resultado = procesar_enemigo("FD", coordenada_enemigo, &mut juego);
    assert!(resultado.is_err());
}

#[test]
pub fn eliminar_enemigo_inexistente() {
    let mut juego: Juego = Juego::new();
    juego.inicializar_dimension(8);
    let coordenada_enemigo: Coordenada = Coordenada::new(3, 5);
    let coordenada_bomba: Coordenada = Coordenada::new(3, 4);
    juego.eliminar_enemigo(coordenada_enemigo, coordenada_bomba);
}

#[test]
pub fn eliminar_enemigo_del_mapa() {
    let mut juego: Juego = Juego::new();
    juego.inicializar_dimension(8);
    let coordenada_enemigo: Coordenada = Coordenada::new(3, 5);
    let coordenada_bomba: Coordenada = Coordenada::new(3, 4);
    juego.inicializar_enemigo(coordenada_enemigo, 1);
    let cantidad_inicial_de_enemigos = juego.enemigos.len();
    juego.eliminar_enemigo(coordenada_enemigo, coordenada_bomba);
    let cantidad_final_de_enemigos = juego.enemigos.len();
    assert_ne!(cantidad_inicial_de_enemigos, cantidad_final_de_enemigos);
}

#[test]
pub fn eliminar_enemigo_con_vida_restante_mayor_a_cero_no_muere() {
    let mut juego: Juego = Juego::new();
    juego.inicializar_dimension(8);
    let coordenada_enemigo: Coordenada = Coordenada::new(3, 5);
    let coordenada_bomba: Coordenada = Coordenada::new(3, 4);
    juego.inicializar_enemigo(coordenada_enemigo, 5);
    let cantidad_inicial_de_enemigos = juego.enemigos.len();
    juego.eliminar_enemigo(coordenada_enemigo, coordenada_bomba);
    let cantidad_final_de_enemigos = juego.enemigos.len();
    assert_eq!(cantidad_inicial_de_enemigos, cantidad_final_de_enemigos);
}

#[test]
pub fn desvio_funciona_correctamente() {
    let mut juego: Juego = Juego::new();
    juego.inicializar_dimension(5);
    let coordenada_enemigo: Coordenada = Coordenada::new(4, 4);
    let coordenada_bomba: Coordenada = Coordenada::new(2, 2);
    let coordenada_desvio: Coordenada = Coordenada::new(2, 4);
    juego.inicializar_enemigo(coordenada_enemigo, 5);
    juego.inicializar_bomba(coordenada_bomba, 5, TipoDeBomba::Normal, "B5".to_string());
    juego.inicializar_desvio(coordenada_desvio, "D".to_string(), "DD".to_string());

    let mut tablero: Vec<Vec<String>> = vec![
        vec![
            "_".to_string(),
            "_".to_string(),
            "_".to_string(),
            "_".to_string(),
            "_".to_string(),
        ],
        vec![
            "_".to_string(),
            "_".to_string(),
            "_".to_string(),
            "_".to_string(),
            "_".to_string(),
        ],
        vec![
            "_".to_string(),
            "_".to_string(),
            "B5".to_string(),
            "_".to_string(),
            "DD".to_string(),
        ],
        vec![
            "_".to_string(),
            "_".to_string(),
            "_".to_string(),
            "_".to_string(),
            "_".to_string(),
        ],
        vec![
            "_".to_string(),
            "_".to_string(),
            "_".to_string(),
            "_".to_string(),
            "F5".to_string(),
        ],
    ];
    let vida_inicial = juego.enemigos[0].vida;
    juego.detonar_bomba(&mut tablero, coordenada_bomba);
    let vida_final = juego.enemigos[0].vida;
    assert_eq!(vida_final, vida_inicial - 1);
}

#[test]
pub fn bomba_vuelve_en_desvio_pero_no_vuelve_a_hacer_danio() {
    let mut juego: Juego = Juego::new();
    juego.inicializar_dimension(5);
    let coordenada_enemigo: Coordenada = Coordenada::new(2, 1);
    let coordenada_bomba: Coordenada = Coordenada::new(2, 2);
    let coordenada_desvio: Coordenada = Coordenada::new(2, 4);
    juego.inicializar_enemigo(coordenada_enemigo, 5);
    juego.inicializar_bomba(coordenada_bomba, 5, TipoDeBomba::Normal, "B5".to_string());
    juego.inicializar_desvio(coordenada_desvio, "L".to_string(), "DL".to_string());

    let mut tablero: Vec<Vec<String>> = vec![
        vec![
            "_".to_string(),
            "_".to_string(),
            "_".to_string(),
            "_".to_string(),
            "_".to_string(),
        ],
        vec![
            "_".to_string(),
            "_".to_string(),
            "_".to_string(),
            "_".to_string(),
            "_".to_string(),
        ],
        vec![
            "_".to_string(),
            "F5".to_string(),
            "B5".to_string(),
            "_".to_string(),
            "DL".to_string(),
        ],
        vec![
            "_".to_string(),
            "_".to_string(),
            "_".to_string(),
            "_".to_string(),
            "_".to_string(),
        ],
        vec![
            "_".to_string(),
            "_".to_string(),
            "_".to_string(),
            "_".to_string(),
            "_".to_string(),
        ],
    ];
    let vida_inicial = juego.enemigos[0].vida;
    juego.detonar_bomba(&mut tablero, coordenada_bomba);
    let vida_final = juego.enemigos[0].vida;
    assert_eq!(vida_final, vida_inicial - 1);
}

#[test]
pub fn bomba_puede_daniar_a_dos_enemigos_diferentes() {
    let mut juego: Juego = Juego::new();
    juego.inicializar_dimension(5);
    let coordenada_enemigo: Coordenada = Coordenada::new(2, 2);
    let coordenada_enemigo_2: Coordenada = Coordenada::new(2, 4);
    let coordenada_bomba: Coordenada = Coordenada::new(2, 3);
    juego.inicializar_enemigo(coordenada_enemigo, 5);
    juego.inicializar_enemigo(coordenada_enemigo_2, 4);
    juego.inicializar_bomba(coordenada_bomba, 5, TipoDeBomba::Normal, "B5".to_string());

    let mut tablero: Vec<Vec<String>> = vec![
        vec![
            "_".to_string(),
            "_".to_string(),
            "_".to_string(),
            "_".to_string(),
            "_".to_string(),
        ],
        vec![
            "_".to_string(),
            "_".to_string(),
            "_".to_string(),
            "_".to_string(),
            "_".to_string(),
        ],
        vec![
            "_".to_string(),
            "F5".to_string(),
            "B5".to_string(),
            "F4".to_string(),
            "_".to_string(),
        ],
        vec![
            "_".to_string(),
            "_".to_string(),
            "_".to_string(),
            "_".to_string(),
            "_".to_string(),
        ],
        vec![
            "_".to_string(),
            "_".to_string(),
            "_".to_string(),
            "_".to_string(),
            "_".to_string(),
        ],
    ];

    let vida_inicial_enemigo_1 = juego.enemigos[0].vida;
    let vida_inicial_enemigo_2 = juego.enemigos[0].vida;

    juego.detonar_bomba(&mut tablero, coordenada_bomba);

    let vida_final_enemigo_1: i8 = juego.enemigos[1].vida;
    let vida_final_enemigo_2: i8 = juego.enemigos[1].vida;

    assert_ne!(vida_inicial_enemigo_1, vida_final_enemigo_1);
    assert_ne!(vida_inicial_enemigo_2, vida_final_enemigo_2);
}

#[test]
pub fn enemigo_recibe_danio_de_dos_bombas_diferentes() {
    let mut juego: Juego = Juego::new();
    juego.inicializar_dimension(8);
    let coordenada_enemigo: Coordenada = Coordenada::new(3, 5);
    let coordenada_bomba: Coordenada = Coordenada::new(3, 4);
    let coordenada_bomba_2: Coordenada = Coordenada::new(2, 5);
    juego.inicializar_enemigo(coordenada_enemigo, 5);
    juego.inicializar_bomba(coordenada_bomba, 5, TipoDeBomba::Normal, "B5".to_string());
    juego.inicializar_bomba(coordenada_bomba_2, 3, TipoDeBomba::Normal, "B3".to_string());

    juego.eliminar_enemigo(coordenada_enemigo, coordenada_bomba);
    let vida_inicial = juego.enemigos[0].vida;
    juego.eliminar_enemigo(coordenada_enemigo, coordenada_bomba_2);
    let vida_final: i8 = juego.enemigos[0].vida;
    assert_ne!(vida_inicial, vida_final);
}

#[test]
pub fn enemigo_no_recibe_danio_de_la_misma_bomba_dos_veces() {
    let mut juego: Juego = Juego::new();
    juego.inicializar_dimension(8);
    let coordenada_enemigo: Coordenada = Coordenada::new(3, 5);
    let coordenada_bomba: Coordenada = Coordenada::new(3, 4);
    juego.inicializar_enemigo(coordenada_enemigo, 5);
    juego.inicializar_bomba(coordenada_bomba, 5, TipoDeBomba::Normal, "B5".to_string());
    juego.eliminar_enemigo(coordenada_enemigo, coordenada_bomba);
    let vida_inicial = juego.enemigos[0].vida;
    juego.eliminar_enemigo(coordenada_enemigo, coordenada_bomba);
    let vida_final: i8 = juego.enemigos[0].vida;
    assert_eq!(vida_inicial, vida_final);
}

#[test]
pub fn evaluar_finaliza_al_encontrar_una_pared() {
    let mut juego: Juego = Juego::new();
    juego.inicializar_dimension(4);
    let mut tablero: Vec<Vec<String>> = vec![
        vec![
            "_".to_string(),
            "_".to_string(),
            "_".to_string(),
            "_".to_string(),
        ],
        vec![
            "_".to_string(),
            "W".to_string(),
            "_".to_string(),
            "_".to_string(),
        ],
        vec![
            "_".to_string(),
            "_".to_string(),
            "_".to_string(),
            "_".to_string(),
        ],
        vec![
            "_".to_string(),
            "B5".to_string(),
            "_".to_string(),
            "_".to_string(),
        ],
    ];
    let coordenada_bomba: Coordenada = Coordenada::new(3, 1);
    let coordenada_pared: Coordenada = Coordenada::new(1, 1);
    let bomba: Bomba = Bomba::new(coordenada_bomba, 5, TipoDeBomba::Normal, "B5".to_string());
    juego.inicializar_pared(coordenada_pared);
    juego.inicializar_bomba(coordenada_bomba, 5, TipoDeBomba::Normal, "B5".to_string());
    let i: i8 = 1;
    juego.evaluar_arriba(&coordenada_bomba, &mut tablero, i, &bomba);
}

#[test]
pub fn evaluar_no_finaliza_al_encontrar_una_roca_con_super_bomba() {
    let mut juego: Juego = Juego::new();
    juego.inicializar_dimension(7);
    let mut tablero: Vec<Vec<String>> = vec![
        vec![
            "_".to_string(),
            "_".to_string(),
            "_".to_string(),
            "_".to_string(),
            "_".to_string(),
            "_".to_string(),
            "_".to_string(),
        ],
        vec![
            "_".to_string(),
            "_".to_string(),
            "_".to_string(),
            "_".to_string(),
            "_".to_string(),
            "_".to_string(),
            "_".to_string(),
        ],
        vec![
            "_".to_string(),
            "_".to_string(),
            "_".to_string(),
            "_".to_string(),
            "_".to_string(),
            "_".to_string(),
            "_".to_string(),
        ],
        vec![
            "_".to_string(),
            "_".to_string(),
            "_".to_string(),
            "_".to_string(),
            "_".to_string(),
            "_".to_string(),
            "_".to_string(),
        ],
        vec![
            "_".to_string(),
            "W".to_string(),
            "_".to_string(),
            "_".to_string(),
            "_".to_string(),
            "_".to_string(),
            "_".to_string(),
        ],
        vec![
            "_".to_string(),
            "S5".to_string(),
            "_".to_string(),
            "_".to_string(),
            "_".to_string(),
            "_".to_string(),
            "_".to_string(),
        ],
        vec![
            "_".to_string(),
            "_".to_string(),
            "_".to_string(),
            "_".to_string(),
            "_".to_string(),
            "_".to_string(),
            "_".to_string(),
        ],
    ];
    let coordenada_bomba: Coordenada = Coordenada::new(5, 1);
    let coordenada_roca: Coordenada = Coordenada::new(4, 1);
    let bomba: Bomba = Bomba::new(coordenada_bomba, 5, TipoDeBomba::Traspaso, "S5".to_string());
    juego.inicializar_roca(coordenada_roca);
    juego.inicializar_bomba(coordenada_bomba, 5, TipoDeBomba::Normal, "B5".to_string());
    let i: i8 = 1;
    juego.evaluar_arriba(&coordenada_bomba, &mut tablero, i, &bomba);
}
