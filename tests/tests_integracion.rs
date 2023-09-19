use std::fs::File;
use std::io;
use std::io::Read;
use tp0::inicializar;

#[test]
pub fn el_juego_crea_los_archivos_correctamente() -> io::Result<()> {
    let comando: Vec<String> = vec![
        "".to_string(),
        "src/maze.txt".to_string(),
        "tests/test_files/output_del_test/try_crear_archivo.txt".to_string(),
        "0".to_string(),
        "0".to_string(),
    ];
    inicializar::run(comando)?;
    let archivo_destino = "tests/test_files/output_del_test/try_crear_archivo.txt";

    assert!(std::fs::metadata(archivo_destino).is_ok());

    Ok(())
}

#[test]
pub fn test_sucesion_de_bombas_eliminan_al_enemigo_esperado() -> io::Result<()> {
    let comando: Vec<String> = vec![
        "".to_string(),
        "src/maze.txt".to_string(),
        "tests/test_files/output_del_test/maze_output.txt".to_string(),
        "0".to_string(),
        "0".to_string(),
    ];
    let _result = inicializar::run(comando);

    let mut output_file = File::open("tests/test_files/output_del_test/maze_output.txt")?;
    let mut expected_output_file = File::open("tests/test_files/output_esperado/maze_output.txt")?;

    let mut buf1 = [0; 1];
    let mut buf2 = [0; 1];

    let mut result = Ok(());

    loop {
        let bytes_read1 = output_file.read(&mut buf1)?;
        let bytes_read2 = expected_output_file.read(&mut buf2)?;

        if bytes_read1 == 0 && bytes_read2 == 0 {
            break;
        }

        if buf1[0] != buf2[0] {
            result = Err(io::Error::new(
                io::ErrorKind::Other,
                "Los archivos son diferentes",
            ));
            break;
        }
    }

    assert!(result.is_ok());

    result
}

#[test]
pub fn test_una_bomba_solo_explota_a_las_que_estan_a_su_alcance() -> io::Result<()> {
    let comando: Vec<String> = vec![
        "".to_string(),
        "src/maze2.txt".to_string(),
        "tests/test_files/output_del_test/maze_2_output.txt".to_string(),
        "2".to_string(),
        "4".to_string(),
    ];
    let _result = inicializar::run(comando);

    let mut output_file = File::open("tests/test_files/output_del_test/maze_2_output.txt")?;
    let mut expected_output_file =
        File::open("tests/test_files/output_esperado/maze_2_output.txt")?;

    let mut buf1 = [0; 1];
    let mut buf2 = [0; 1];

    let mut result = Ok(());

    loop {
        let bytes_read1 = output_file.read(&mut buf1)?;
        let bytes_read2 = expected_output_file.read(&mut buf2)?;

        if bytes_read1 == 0 && bytes_read2 == 0 {
            break;
        }

        if buf1[0] != buf2[0] {
            result = Err(io::Error::new(
                io::ErrorKind::Other,
                "Los archivos son diferentes",
            ));
            break;
        }
    }

    assert!(result.is_ok());

    result
}

#[test]
pub fn test_bomba_explota_pero_no_logra_atacar_un_enemigo() -> io::Result<()> {
    let comando: Vec<String> = vec![
        "".to_string(),
        "src/maze2.txt".to_string(),
        "tests/test_files/output_del_test/maze_2_output_2.txt".to_string(),
        "4".to_string(),
        "0".to_string(),
    ];
    let _result = inicializar::run(comando);

    let mut output_file = File::open("tests/test_files/output_del_test/maze_2_output_2.txt")?;
    let mut expected_output_file =
        File::open("tests/test_files/output_esperado/maze_2_output_2.txt")?;

    let mut buf1 = [0; 1];
    let mut buf2 = [0; 1];

    let mut result = Ok(());

    loop {
        let bytes_read1 = output_file.read(&mut buf1)?;
        let bytes_read2 = expected_output_file.read(&mut buf2)?;

        if bytes_read1 == 0 && bytes_read2 == 0 {
            break;
        }

        if buf1[0] != buf2[0] {
            result = Err(io::Error::new(
                io::ErrorKind::Other,
                "Los archivos son diferentes",
            ));
            break;
        }
    }

    assert!(result.is_ok());

    result
}

#[test]
pub fn test_desvio_genera_el_resultado_correcto() -> io::Result<()> {
    let comando: Vec<String> = vec![
        "".to_string(),
        "src/maze3.txt".to_string(),
        "tests/test_files/output_del_test/maze_3_output.txt".to_string(),
        "0".to_string(),
        "4".to_string(),
    ];
    let _result = inicializar::run(comando);

    let mut output_file = File::open("tests/test_files/output_del_test/maze_3_output.txt")?;
    let mut expected_output_file =
        File::open("tests/test_files/output_esperado/maze_3_output.txt")?;

    let mut buf1 = [0; 1];
    let mut buf2 = [0; 1];

    let mut result = Ok(());

    loop {
        let bytes_read1 = output_file.read(&mut buf1)?;
        let bytes_read2 = expected_output_file.read(&mut buf2)?;

        if bytes_read1 == 0 && bytes_read2 == 0 {
            break;
        }

        if buf1[0] != buf2[0] {
            result = Err(io::Error::new(
                io::ErrorKind::Other,
                "Los archivos son diferentes",
            ));
            break;
        }
    }

    assert!(result.is_ok());

    result
}

#[test]
pub fn test_se_actualiza_la_vida_del_enemigo_en_el_archivo_cuando_es_atacado_pero_no_muere() -> io::Result<()> {
    let comando: Vec<String> = vec![
        "".to_string(),
        "src/maze4.txt".to_string(),
        "tests/test_files/output_del_test/maze_4_output.txt".to_string(),
        "0".to_string(),
        "0".to_string(),
    ];
    let _result = inicializar::run(comando);

    let mut output_file = File::open("tests/test_files/output_del_test/maze_4_output.txt")?;
    let mut expected_output_file =
        File::open("tests/test_files/output_esperado/maze_4_output.txt")?;

    let mut buf1 = [0; 1];
    let mut buf2 = [0; 1];

    let mut result = Ok(());

    loop {
        let bytes_read1 = output_file.read(&mut buf1)?;
        let bytes_read2 = expected_output_file.read(&mut buf2)?;

        if bytes_read1 == 0 && bytes_read2 == 0 {
            break;
        }

        if buf1[0] != buf2[0] {
            result = Err(io::Error::new(
                io::ErrorKind::Other,
                "Los archivos son diferentes",
            ));
            break;
        }
    }

    assert!(result.is_ok());

    result
}

#[test]
pub fn test_super_bomba_atraviesa_super_roca_y_logra_atacar_al_enemigo() -> io::Result<()> {
    let comando: Vec<String> = vec![
        "".to_string(),
        "src/maze5.txt".to_string(),
        "tests/test_files/output_del_test/maze_5_output.txt".to_string(),
        "3".to_string(),
        "5".to_string(),
    ];
    let _result = inicializar::run(comando);

    let mut output_file = File::open("tests/test_files/output_del_test/maze_5_output.txt")?;
    let mut expected_output_file =
        File::open("tests/test_files/output_esperado/maze_5_output.txt")?;

    let mut buf1 = [0; 1];
    let mut buf2 = [0; 1];

    let mut result = Ok(());

    loop {
        let bytes_read1 = output_file.read(&mut buf1)?;
        let bytes_read2 = expected_output_file.read(&mut buf2)?;

        if bytes_read1 == 0 && bytes_read2 == 0 {
            break;
        }

        if buf1[0] != buf2[0] {
            result = Err(io::Error::new(
                io::ErrorKind::Other,
                "Los archivos son diferentes",
            ));
            break;
        }
    }

    assert!(result.is_ok());

    result
}

#[test]
pub fn test_bomba_vuelve_en_desvio_pero_solo_se_resta_un_punto_de_vida() -> io::Result<()> {
    let comando: Vec<String> = vec![
        "".to_string(),
        "src/maze6.txt".to_string(),
        "tests/test_files/output_del_test/maze_6_output.txt".to_string(),
        "3".to_string(),
        "5".to_string(),
    ];
    let _result = inicializar::run(comando);

    let mut output_file = File::open("tests/test_files/output_del_test/maze_6_output.txt")?;
    let mut expected_output_file =
        File::open("tests/test_files/output_esperado/maze_6_output.txt")?;

    let mut buf1 = [0; 1];
    let mut buf2 = [0; 1];

    let mut result = Ok(());

    loop {
        let bytes_read1 = output_file.read(&mut buf1)?;
        let bytes_read2 = expected_output_file.read(&mut buf2)?;

        if bytes_read1 == 0 && bytes_read2 == 0 {
            break;
        }

        if buf1[0] != buf2[0] {
            result = Err(io::Error::new(
                io::ErrorKind::Other,
                "Los archivos son diferentes",
            ));
            break;
        }
    }

    assert!(result.is_ok());

    result
}

#[test]
pub fn error_en_coordenada_y_se_informa_en_archivo() -> io::Result<()> {
    let comando: Vec<String> = vec![
        "".to_string(),
        "src/maze6.txt".to_string(),
        "tests/test_files/output_del_test/coordenada_y_erronea_output.txt".to_string(),
        "3".to_string(),
        "y".to_string(),
    ];
    let _result = inicializar::run(comando);

    let mut output_file =
        File::open("tests/test_files/output_del_test/coordenada_y_erronea_output.txt")?;
    let mut expected_output_file =
        File::open("tests/test_files/output_esperado/coordenada_y_erronea_output.txt")?;

    let mut buf1 = [0; 1];
    let mut buf2 = [0; 1];

    let mut result = Ok(());

    loop {
        let bytes_read1 = output_file.read(&mut buf1)?;
        let bytes_read2 = expected_output_file.read(&mut buf2)?;

        if bytes_read1 == 0 && bytes_read2 == 0 {
            break;
        }

        if buf1[0] != buf2[0] {
            result = Err(io::Error::new(
                io::ErrorKind::Other,
                "Los archivos son diferentes",
            ));
            break;
        }
    }

    assert!(result.is_ok());

    result
}

#[test]
pub fn caracter_erroneo_se_informa_en_archivo() -> io::Result<()> {
    let comando: Vec<String> = vec![
        "".to_string(),
        "src/maze_fail.txt".to_string(),
        "tests/test_files/output_del_test/caracter_invalido_output.txt".to_string(),
        "3".to_string(),
        "4".to_string(),
    ];
    let _result = inicializar::run(comando);

    let mut output_file =
        File::open("tests/test_files/output_del_test/caracter_invalido_output.txt")?;
    let mut expected_output_file =
        File::open("tests/test_files/output_esperado/caracter_invalido_output.txt")?;

    let mut buf1 = [0; 1];
    let mut buf2 = [0; 1];

    let mut result = Ok(());

    loop {
        let bytes_read1 = output_file.read(&mut buf1)?;
        let bytes_read2 = expected_output_file.read(&mut buf2)?;

        if bytes_read1 == 0 && bytes_read2 == 0 {
            break;
        }

        if buf1[0] != buf2[0] {
            result = Err(io::Error::new(
                io::ErrorKind::Other,
                "Los archivos son diferentes",
            ));
            break;
        }
    }

    assert!(result.is_ok());

    result
}

#[test]
pub fn vida_erronea_en_enemigo_se_informa_en_archivo() -> io::Result<()> {
    let comando: Vec<String> = vec![
        "".to_string(),
        "src/maze_enemigo_fail.txt".to_string(),
        "tests/test_files/output_del_test/maze_enemigo_fail_output.txt".to_string(),
        "3".to_string(),
        "4".to_string(),
    ];
    let _result = inicializar::run(comando);

    let mut output_file =
        File::open("tests/test_files/output_del_test/maze_enemigo_fail_output.txt")?;
    let mut expected_output_file =
        File::open("tests/test_files/output_esperado/maze_enemigo_fail_output.txt")?;

    let mut buf1 = [0; 1];
    let mut buf2 = [0; 1];

    let mut result = Ok(());

    loop {
        let bytes_read1 = output_file.read(&mut buf1)?;
        let bytes_read2 = expected_output_file.read(&mut buf2)?;

        if bytes_read1 == 0 && bytes_read2 == 0 {
            break;
        }

        if buf1[0] != buf2[0] {
            result = Err(io::Error::new(
                io::ErrorKind::Other,
                "Los archivos son diferentes",
            ));
            break;
        }
    }

    assert!(result.is_ok());

    result
}

#[test]
pub fn bomba_fuera_de_rango_informa_en_archivo() -> io::Result<()> {
    let comando: Vec<String> = vec![
        "".to_string(),
        "src/maze.txt".to_string(),
        "tests/test_files/output_del_test/bomba_fuera_de_rango_output.txt".to_string(),
        "3".to_string(),
        "9".to_string(),
    ];
    let _result = inicializar::run(comando);

    let mut output_file =
        File::open("tests/test_files/output_del_test/bomba_fuera_de_rango_output.txt")?;
    let mut expected_output_file =
        File::open("tests/test_files/output_esperado/bomba_fuera_de_rango_output.txt")?;

    let mut buf1 = [0; 1];
    let mut buf2 = [0; 1];

    let mut result = Ok(());

    loop {
        let bytes_read1 = output_file.read(&mut buf1)?;
        let bytes_read2 = expected_output_file.read(&mut buf2)?;

        if bytes_read1 == 0 && bytes_read2 == 0 {
            break;
        }

        if buf1[0] != buf2[0] {
            result = Err(io::Error::new(
                io::ErrorKind::Other,
                "Los archivos son diferentes",
            ));
            break;
        }
    }

    assert!(result.is_ok());

    result
}

#[test]
pub fn matriz_no_cuadrada_informa_en_archivo() -> io::Result<()> {
    let comando: Vec<String> = vec![
        "".to_string(),
        "src/maze_rectangle.txt".to_string(),
        "tests/test_files/output_del_test/maze_rectangle_output.txt".to_string(),
        "3".to_string(),
        "0".to_string(),
    ];
    let _result = inicializar::run(comando);

    let mut output_file = File::open("tests/test_files/output_del_test/maze_rectangle_output.txt")?;
    let mut expected_output_file =
        File::open("tests/test_files/output_esperado/maze_rectangle_output.txt")?;

    let mut buf1 = [0; 1];
    let mut buf2 = [0; 1];

    let mut result = Ok(());

    loop {
        let bytes_read1 = output_file.read(&mut buf1)?;
        let bytes_read2 = expected_output_file.read(&mut buf2)?;

        if bytes_read1 == 0 && bytes_read2 == 0 {
            break;
        }

        if buf1[0] != buf2[0] {
            result = Err(io::Error::new(
                io::ErrorKind::Other,
                "Los archivos son diferentes",
            ));
            break;
        }
    }

    assert!(result.is_ok());

    result
}
