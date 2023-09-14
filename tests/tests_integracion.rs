use std::fs::File;
use tp0::estructuras_juego::bomba::*;
use tp0::estructuras_juego::coordenada::*;
use tp0::inicializar::{inicializar_juego, procesar_enemigo};
use tp0::juego::*;

#[test]
pub fn inicializar_juego_con_enemigo_invalido_devuelve_error(
) -> Result<(), Box<dyn std::error::Error>> {
    let file_creation_result = File::create("testing_output.txt");
    let mut file = match file_creation_result {
        Ok(file) => file,
        Err(err) => {
            return Err(err.into());
        }
    };

    let mut juego: Juego = Juego::new();
    let coordenada_enemigo: Coordenada = Coordenada::new(3, 5);
    let resultado = inicializar_juego(coordenada_enemigo, "FB", &mut juego, &mut file);

    assert!(resultado.is_err());

    Ok(())
}

#[test]
pub fn inicializar_juego_con_enemigo_correcto_devuelve_ok() -> Result<(), Box<dyn std::error::Error>>
{
    let file_creation_result = File::create("testing_output.txt");
    let mut file = match file_creation_result {
        Ok(file) => file,
        Err(err) => {
            return Err(err.into());
        }
    };
    let mut juego: Juego = Juego::new();
    let coordenada_enemigo: Coordenada = Coordenada::new(3, 5);
    let resultado = inicializar_juego(coordenada_enemigo, "F6", &mut juego, &mut file);
    assert!(resultado.is_ok());

    Ok(())
}

#[test]
pub fn inicializar_enemigo_sin_especificar_vida_devuelve_error() {
    let mut juego: Juego = Juego::new();
    let coordenada_enemigo: Coordenada = Coordenada::new(3, 5);
    let resultado = procesar_enemigo("F", coordenada_enemigo, &mut juego);
    assert!(resultado.is_err());
}

#[test]
pub fn inicializar_enemigo_con_formato_correcto_devuelve_ok() {
    let mut juego: Juego = Juego::new();
    let coordenada_enemigo: Coordenada = Coordenada::new(3, 5);
    let resultado = procesar_enemigo("F5", coordenada_enemigo, &mut juego);
    assert!(resultado.is_ok());
}

#[test]
pub fn inicializar_enemigo_con_formato_erroneo_devuelve_error() {
    let mut juego: Juego = Juego::new();
    let coordenada_enemigo: Coordenada = Coordenada::new(3, 5);
    let resultado = procesar_enemigo("FD", coordenada_enemigo, &mut juego);
    assert!(resultado.is_err());
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
pub fn evaluar_casillero_finaliza_al_encontrar_una_pared_con_bomba_de_traspaso() {
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
    juego.inicializar_bomba(coordenada_bomba, 5, TipoDeBomba::Traspaso, "S5".to_string());
    let mut coordenada_a_evaluar: Coordenada = Coordenada::new(4, 1);
    let i: i8 = 1;
    juego.evaluar_casillero(
        &mut coordenada_a_evaluar,
        &mut tablero,
        i,
        &bomba,
        coordenada_bomba,
    );
    assert_eq!(coordenada_a_evaluar.x, -1);
    assert_eq!(coordenada_a_evaluar.y, -1);
}

#[test]
pub fn evaluar_casillero_no_finaliza_al_encontrar_una_roca_con_bomba_de_traspaso() {
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
            "R".to_string(),
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
    juego.inicializar_bomba(coordenada_bomba, 5, TipoDeBomba::Traspaso, "S5".to_string());
    let mut coordenada_a_evaluar: Coordenada = Coordenada::new(4, 1);
    let i: i8 = 1;
    juego.evaluar_casillero(
        &mut coordenada_a_evaluar,
        &mut tablero,
        i,
        &bomba,
        coordenada_bomba,
    );
    assert_eq!(coordenada_a_evaluar.x, 4);
    assert_eq!(coordenada_a_evaluar.y, 1);
}

#[test]
pub fn evaluar_casillero_finaliza_al_encontrar_una_roca_con_bomba_normal() {
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
            "R".to_string(),
            "_".to_string(),
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
    let bomba: Bomba = Bomba::new(coordenada_bomba, 5, TipoDeBomba::Normal, "B5".to_string());
    juego.inicializar_roca(coordenada_roca);
    juego.inicializar_bomba(coordenada_bomba, 5, TipoDeBomba::Normal, "B5".to_string());
    let mut coordenada_a_evaluar: Coordenada = Coordenada::new(4, 1);
    let i: i8 = 1;
    juego.evaluar_casillero(
        &mut coordenada_a_evaluar,
        &mut tablero,
        i,
        &bomba,
        coordenada_bomba,
    );
    assert_eq!(coordenada_a_evaluar.x, -1);
    assert_eq!(coordenada_a_evaluar.y, -1);
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
pub fn bomba_no_se_detona_dos_veces() {
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
            "_".to_string(),
            "_".to_string(),
            "_".to_string(),
            "_".to_string(),
            "_".to_string(),
            "_".to_string(),
        ],
        vec![
            "_".to_string(),
            "B5".to_string(),
            "F5".to_string(),
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
    let coordenada_enemigo: Coordenada = Coordenada::new(5, 2);
    juego.inicializar_enemigo(coordenada_enemigo, 5);
    juego.inicializar_bomba(coordenada_bomba, 5, TipoDeBomba::Normal, "B5".to_string());
    juego.detonar_bomba(&mut tablero, coordenada_bomba);
    let coordenada_enemigo_nuevo: Coordenada = Coordenada::new(5, 2);
    juego.inicializar_enemigo(coordenada_enemigo_nuevo, 3);
    tablero[0][5] = "F3".to_string();
    juego.detonar_bomba(&mut tablero, coordenada_bomba);
    assert_eq!(juego.enemigos[1].vida, 3);
}

#[test]
pub fn se_actualiza_la_lista_de_bombas_del_enemigo_luego_del_impacto() {
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
            "_".to_string(),
            "_".to_string(),
            "_".to_string(),
            "_".to_string(),
            "_".to_string(),
            "_".to_string(),
        ],
        vec![
            "_".to_string(),
            "B5".to_string(),
            "F5".to_string(),
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
    let coordenada_enemigo: Coordenada = Coordenada::new(5, 2);
    let mut juego: Juego = Juego::new();
    juego.inicializar_dimension(7);
    juego.inicializar_enemigo(coordenada_enemigo, 5);
    juego.inicializar_bomba(coordenada_bomba, 5, TipoDeBomba::Normal, "B5".to_string());
    let antes = juego.enemigos[0].bombas_que_lo_impactaron.len();
    juego.detonar_bomba(&mut tablero, coordenada_bomba);
    let despues: usize = juego.enemigos[0].bombas_que_lo_impactaron.len();
    assert_ne!(antes, despues);
}
