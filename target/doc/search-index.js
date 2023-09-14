var searchIndex = JSON.parse('{\
"tp0":{"doc":"","t":"AAFRRDRRRRLLFFLLLFFLFFFFFLLLLRRRRRRRRDRRAMLLAAMLMLAMLLLLLLLLLLLLLLLLLAMLLLLLLLLLDNENMLLLLLLLLMMLLLLLLMLLLMLLLLLLLLDLLLLLLLLLLLLLMMDLLMMLMLLLLLDLMLLMLMLLLLLMDNNELLLLMLLLLLMLLLMLLLLLL","n":["inicializar","juego","main","BOMBA_DE_TRANSPASO","BOMBA_NORMAL","CustomError","DESVIO","ENEMIGO","PARED","ROCA","borrow","borrow_mut","cargar_juego","crear_archivo_en_ruta","fmt","fmt","from","inicializar_coordenada_de_la_bomba","inicializar_juego","into","procesar_bomba","procesar_desvio","procesar_enemigo","procesar_linea_de_configuracion","run","to_string","try_from","try_into","type_id","BOMBA_DE_TRANSPASO","BOMBA_NORMAL","DESVIO","DESVIO_ABAJO","DESVIO_ARRIBA","DESVIO_DERECHA","DESVIO_IZQUIERDA","ENEMIGO","Juego","PARED","ROCA","bomba","bombas","borrow","borrow_mut","coordenada","desvio","desvios","detonar_bomba","dimension","eliminar_enemigo","enemigo","enemigos","evaluar_abajo","evaluar_arriba","evaluar_casillero","evaluar_derecha","evaluar_izquierda","from","funcion_bomba","imprimir_tablero","imprimir_tablero_en_archivo","inicializar_bomba","inicializar_desvio","inicializar_dimension","inicializar_enemigo","inicializar_pared","inicializar_roca","into","new","obstaculo","obstaculos","posicionar_bombas","posicionar_desvios","posicionar_elementos_en_tablero","posicionar_enemigos","posicionar_obstaculos","realizar_jugada","try_from","try_into","type_id","Bomba","Normal","TipoDeBomba","Traspaso","alcance","borrow","borrow","borrow_mut","borrow_mut","clone","clone","clone_into","clone_into","coordenada","detonada","detonar","eq","equivalent","fmt","from","from","id","into","into","new","tipo","to_owned","to_owned","try_from","try_from","try_into","try_into","type_id","type_id","Coordenada","borrow","borrow_mut","clone","clone_into","fmt","from","into","is_equal_to","new","to_owned","try_from","try_into","type_id","x","y","Desvio","borrow","borrow_mut","coordenada","direccion","from","id","into","new","try_from","try_into","type_id","Enemigo","actualizar_lista_de_bombas","bombas_que_lo_impactaron","borrow","borrow_mut","coordenada","from","id","into","new","try_from","try_into","type_id","vida","Obstaculo","Pared","Roca","TipoDeObstaculo","borrow","borrow","borrow_mut","borrow_mut","coordenada","eq","equivalent","fmt","from","from","id","into","into","new","tipo","try_from","try_from","try_into","try_into","type_id","type_id"],"q":[[0,"tp0"],[3,"tp0::inicializar"],[29,"tp0::juego"],[80,"tp0::juego::bomba"],[114,"tp0::juego::coordenada"],[130,"tp0::juego::desvio"],[142,"tp0::juego::enemigo"],[156,"tp0::juego::obstaculo"]],"d":["Este módulo contiene las funciones y estructuras para …","","","","","","","","","","","","Carga el juego desde un archivo de configuración.","Crea un archivo en la ruta especificada.","","","Returns the argument unchanged.","Inicializa la coordenada de la bomba basándose en …","Inicializa elementos del juego basados en una palabra del …","Calls <code>U::from(self)</code>.","Procesa una bomba en el archivo de configuración.","Procesa una entrada que representa un desvío en el …","Procesa una entrada que representa un enemigo en el …","Procesa una línea del archivo de configuración y realiza …","Ejecuta el programa principal.","","","","","","","","","","","","","","","","","","","","","","","Detona una bomba en una coordenada dada y afecta el …","","Elimina un enemigo del juego y actualiza su estado.","","","Evalúa las posiciones hacia abajo desde la coordenada …","Evalúa las posiciones hacia arriba desde la coordenada …","Evalúa el contenido de un casillero en el tablero y …","Evalúa las posiciones hacia la derecha desde la …","Evalúa las posiciones hacia la izquierda desde la …","Returns the argument unchanged.","Evalúa el alcance de una bomba y su impacto en el tablero.","Imprime el tablero en la consola.","Imprime el tablero en un archivo.","Inicializa una bomba en el juego.","Inicializa un desvío en el juego.","Inicializa la dimensión del juego.","Inicializa un enemigo en el juego.","Inicializa una pared en el juego.","Inicializa una roca en el juego.","Calls <code>U::from(self)</code>.","","","","Posiciona las bombas en el tablero del juego.","Posiciona los desvíos en el tablero del juego.","Posiciona todos los elementos en el tablero del juego.","Posiciona los enemigos en el tablero del juego.","Posiciona los obstáculos en el tablero del juego.","Realiza una jugada en el juego y guarda el estado final en …","","","","Representa una bomba en el juego con un identificador, …","Representa una bomba de tipo “Normal”.","Enumeración que representa los tipos de bombas en el …","Representa una bomba de tipo “Traspaso”.","Alcance de la explosión de la bomba.","","","","","","","","","Coordenada en la que se encuentra la bomba en el tablero.","Estado de detonación de la bomba (true si ha detonado, …","Detona la bomba, cambiando su estado de detonación a true.","","","","Returns the argument unchanged.","Returns the argument unchanged.","Identificador único de la bomba.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Crea una nueva bomba con el tipo, alcance, identificador y …","Tipo de bomba, que puede ser “Normal” o “Traspaso”.","","","","","","","","","Representa una coordenada en el tablero del juego con …","","","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Comprueba si esta coordenada es igual a otra coordenada …","Crea una nueva coordenada con valores <code>x</code> e <code>y</code> especificados.","","","","","Valor de la coordenada en el eje X.","Valor de la coordenada en el eje Y.","Representa un objeto de desvío en el juego, que puede …","","","La coordenada en la que se encuentra el desvío en el …","La dirección en la que el desvío redirige las bombas.","Returns the argument unchanged.","El identificador único del desvío.","Calls <code>U::from(self)</code>.","Crea un nuevo objeto de desvío con la coordenada, …","","","","Representa un enemigo en el juego con una vida determinada …","Actualiza la lista de coordenadas de bombas que han …","Lista de coordenadas de bombas que han impactado a este …","","","La coordenada en la que se encuentra el enemigo en el …","Returns the argument unchanged.","El identificador único del enemigo.","Calls <code>U::from(self)</code>.","Crea un nuevo objeto de enemigo con la coordenada y vida …","","","","La cantidad de vida del enemigo, que determina cuántas …","Representa un obstáculo en el tablero del juego con una …","Representa un obstáculo de tipo “Pared”.","Representa un obstáculo de tipo “Roca”.","Enumeración que representa los tipos de obstáculos en el …","","","","","Coordenada en la que se encuentra el obstáculo en el …","","","","Returns the argument unchanged.","Returns the argument unchanged.","Identificador del obstáculo (por ejemplo, “W” para …","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Crea un nuevo obstáculo con el tipo y la coordenada …","Tipo de obstáculo, que puede ser “Pared” o “Roca”.","","","","","",""],"i":[0,0,0,0,0,0,0,0,0,0,5,5,0,0,5,5,5,0,0,5,0,0,0,0,0,5,5,5,5,0,0,0,0,0,0,0,0,0,0,0,0,3,3,3,0,0,3,3,3,3,0,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,0,3,3,3,3,3,3,3,3,3,3,0,16,0,16,15,16,15,16,15,16,15,16,15,15,15,15,16,16,16,16,15,15,16,15,15,15,16,15,16,15,16,15,16,15,0,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,0,18,18,18,18,18,18,18,18,18,18,18,0,19,19,19,19,19,19,19,19,19,19,19,19,19,0,20,20,0,21,20,21,20,21,20,20,20,21,20,21,21,20,21,21,21,20,21,20,21,20],"f":[0,0,[[],1],0,0,0,0,0,0,0,[[]],[[]],[2,[[1,[3]]]],[2,[[1,[4]]]],[[5,6],7],[[5,6],7],[[]],[[[9,[8]]],[[1,[10]]]],[[10,2,3],1],[[]],[[2,10,3],1],[[2,10,3],1],[[2,10,3],1],[[2,11,11,3],1],[[[12,[8]]],1],[[],8],[[],13],[[],13],[[],14],0,0,0,0,0,0,0,0,0,0,0,0,0,[[]],[[]],0,0,0,[[3,[12,[[12,[8]]]],10]],0,[[3,10,10]],0,0,[[3,10,[12,[[12,[8]]]],11,15]],[[3,10,[12,[[12,[8]]]],11,15]],[[3,10,[12,[[12,[8]]]],11,15,10]],[[3,10,[12,[[12,[8]]]],11,15]],[[3,10,[12,[[12,[8]]]],11,15]],[[]],[[3,15,[12,[[12,[8]]]]]],[[3,[12,[[12,[8]]]]]],[[3,4,[12,[[12,[8]]]]],1],[[3,10,11,16,8]],[[3,10,8,8]],[[3,11]],[[3,10,11]],[[3,10]],[[3,10]],[[]],[[],3],0,0,[[3,[9,[[12,[8]]]]]],[[3,[9,[[12,[8]]]]]],[3,[[12,[[12,[8]]]]]],[[3,[9,[[12,[8]]]]]],[[3,[9,[[12,[8]]]]]],[[3,4,10],1],[[],13],[[],13],[[],14],0,0,0,0,0,[[]],[[]],[[]],[[]],[16,16],[15,15],[[]],[[]],0,0,[15],[[16,16],17],[[],17],[[16,6],7],[[]],[[]],0,[[]],[[]],[[10,11,16,8],15],0,[[]],[[]],[[],13],[[],13],[[],13],[[],13],[[],14],[[],14],0,[[]],[[]],[10,10],[[]],[[10,6],7],[[]],[[]],[[10,10],17],[[11,11],10],[[]],[[],13],[[],13],[[],14],0,0,0,[[]],[[]],0,0,[[]],0,[[]],[[10,8,8],18],[[],13],[[],13],[[],14],0,[[19,10]],0,[[]],[[]],0,[[]],0,[[]],[[10,11],19],[[],13],[[],13],[[],14],0,0,0,0,0,[[]],[[]],[[]],[[]],0,[[20,20],17],[[],17],[[20,6],7],[[]],[[]],0,[[]],[[]],[[20,10],21],0,[[],13],[[],13],[[],13],[[],13],[[],14],[[],14]],"c":[],"p":[[6,"Result"],[15,"str"],[3,"Juego"],[3,"File"],[3,"CustomError"],[3,"Formatter"],[6,"Result"],[3,"String"],[15,"slice"],[3,"Coordenada"],[15,"i8"],[3,"Vec"],[4,"Result"],[3,"TypeId"],[3,"Bomba"],[4,"TipoDeBomba"],[15,"bool"],[3,"Desvio"],[3,"Enemigo"],[4,"TipoDeObstaculo"],[3,"Obstaculo"]]}\
}');
if (typeof window !== 'undefined' && window.initSearch) {window.initSearch(searchIndex)};
if (typeof exports !== 'undefined') {exports.searchIndex = searchIndex};