
#[derive(Debug, Clone)]
struct Libro {
    isbn: u32,
    titulo: String,
}

struct Nodo {
    libro: Libro,
    izquierdo: Option<Box<Nodo>>,
    derecho: Option<Box<Nodo>>,
    altura: i32,
}

impl Nodo {
    fn nuevo(libro: Libro) -> Self {
        Nodo {
            libro,
            izquierdo: None,
            derecho: None,
            altura: 1,
        }
    }
}

// --- FUNCIONES DE UTILIDAD EXISTENTES ---

fn obtener_altura(nodo: &Option<Box<Nodo>>) -> i32 {
    nodo.as_ref().map_or(0, |n| n.altura)
}

fn actualizar_altura(nodo: &mut Nodo) {
    nodo.altura = 1 + std::cmp::max(
        obtener_altura(&nodo.izquierdo),
        obtener_altura(&nodo.derecho),
    );
}

fn obtener_balance(nodo: &Nodo) -> i32 {
    obtener_altura(&nodo.izquierdo) - obtener_altura(&nodo.derecho)
}

// --- ROTACIONES ---

fn rotar_derecha(mut y: Box<Nodo>) -> Box<Nodo> {
    let mut x = y.izquierdo.take().expect("Hijo izquierdo ausente");
    y.izquierdo = x.derecho.take();
    actualizar_altura(&mut y);
    x.derecho = Some(y);
    actualizar_altura(&mut x);
    x
}

fn rotar_izquierda(mut x: Box<Nodo>) -> Box<Nodo> {
    let mut y = x.derecho.take().expect("Hijo derecho ausente");
    x.derecho = y.izquierdo.take();
    actualizar_altura(&mut x);
    y.izquierdo = Some(x);
    actualizar_altura(&mut y);
    y
}

// --- FASE 2: MOTOR DE CONSULTA ---

fn buscar(nodo: &Option<Box<Nodo>>, isbn: u32) -> Option<&Libro> {
    let n = nodo.as_ref()?; // Retorna None si el nodo es None
    if isbn == n.libro.isbn {
        Some(&n.libro)
    } else if isbn < n.libro.isbn {
        buscar(&n.izquierdo, isbn)
    } else {
        buscar(&n.derecho, isbn)
    }
}

// --- FASE 3: MANTENIMIENTO (ELIMINACIÓN) ---

fn buscar_minimo(nodo: &Nodo) -> &Libro {
    match &nodo.izquierdo {
        None => &nodo.libro,
        Some(izq) => buscar_minimo(izq),
    }
}

fn eliminar(nodo_opt: Option<Box<Nodo>>, isbn: u32) -> Option<Box<Nodo>> {
    let mut nodo = nodo_opt?;

    if isbn < nodo.libro.isbn {
        nodo.izquierdo = eliminar(nodo.izquierdo.take(), isbn);
    } else if isbn > nodo.libro.isbn {
        nodo.derecho = eliminar(nodo.derecho.take(), isbn);
    } else {
        // Encontramos el nodo a eliminar
        if nodo.izquierdo.is_none() {
            return nodo.derecho;
        } else if nodo.derecho.is_none() {
            return nodo.izquierdo;
        } else {
            // Caso 2 hijos: Obtener sucesor in-orden (mínimo del subárbol derecho)
            let sucesor_libro = buscar_minimo(nodo.derecho.as_ref().unwrap()).clone();
            nodo.libro = sucesor_libro;
            nodo.derecho = eliminar(nodo.derecho.take(), nodo.libro.isbn);
        }
    }

    // Actualizar altura y balancear
    // aca tras cada elimiacion recalculamos el balanceo, si el valor absoluto es >1, 
    // aplicamos rotaciones simples o dobles segun la direccion del desbalance
    actualizar_altura(&mut nodo);
    let balance = obtener_balance(&nodo);

    // Rebalanceo (Lógica similar a la inserción)
    if balance > 1 {
        if obtener_balance(nodo.izquierdo.as_ref().unwrap()) >= 0 {
            return Some(rotar_derecha(nodo));
        } else {
            let izq = nodo.izquierdo.take().unwrap();
            nodo.izquierdo = Some(rotar_izquierda(izq));
            return Some(rotar_derecha(nodo));
        }
    }
    if balance < -1 {
        if obtener_balance(nodo.derecho.as_ref().unwrap()) <= 0 {
            return Some(rotar_izquierda(nodo));
        } else {
            let der = nodo.derecho.take().unwrap();
            nodo.derecho = Some(rotar_derecha(der));
            return Some(rotar_izquierda(nodo));
        }
    }

    Some(nodo)
}
// Aca utilice el sucesor in-orden para mantener la propiedad de busqueda, uso la clonacion porque es mas sencillo porque transfiere los datos al noddo actual antes de proceder con la eliminacion del nodo duplicado en el suarbol derecho
// --- FASE 4: FUNCIONALIDADES (OPCIÓN A: RANGO) ---

fn buscar_rango<'a>(nodo: &'a Option<Box<Nodo>>, min: u32, max: u32, resultados: &mut Vec<&'a Libro>) {
    if let Some(n) = nodo {
        if n.libro.isbn > min {
            buscar_rango(&n.izquierdo, min, max, resultados);
        }
        if n.libro.isbn >= min && n.libro.isbn <= max {
            resultados.push(&n.libro);
        }
        if n.libro.isbn < max {
            buscar_rango(&n.derecho, min, max, resultados);
        }
    }
}

// --- INSERCIÓN (EXISTENTE) ---

fn insertar(nodo_opt: Option<Box<Nodo>>, libro: Libro) -> Box<Nodo> {
    let mut nodo = match nodo_opt {
        None => return Box::new(Nodo::nuevo(libro)),
        Some(n) => n,
    };

    let isbn_nuevo = libro.isbn;

    if isbn_nuevo < nodo.libro.isbn {
        nodo.izquierdo = Some(insertar(nodo.izquierdo.take(), libro));
    } else if isbn_nuevo > nodo.libro.isbn {
        nodo.derecho = Some(insertar(nodo.derecho.take(), libro));
    } else {
        return nodo; 
    }

    actualizar_altura(&mut nodo);
    let balance = obtener_balance(&nodo);

    if balance > 1 && isbn_nuevo < nodo.izquierdo.as_ref().unwrap().libro.isbn {
        return rotar_derecha(nodo);
    }
    if balance < -1 && isbn_nuevo > nodo.derecho.as_ref().unwrap().libro.isbn {
        return rotar_izquierda(nodo);
    }
    if balance > 1 && isbn_nuevo > nodo.izquierdo.as_ref().unwrap().libro.isbn {
        let hijo_izq = nodo.izquierdo.take().unwrap();
        nodo.izquierdo = Some(rotar_izquierda(hijo_izq));
        return rotar_derecha(nodo);
    }
    if balance < -1 && isbn_nuevo < nodo.derecho.as_ref().unwrap().libro.isbn {
        let hijo_der = nodo.derecho.take().unwrap();
        nodo.derecho = Some(rotar_derecha(hijo_der));
        return rotar_izquierda(nodo);
    }
    nodo
}

fn imprimir(nodo: &Option<Box<Nodo>>, nivel: usize) {
    if let Some(n) = nodo {
        imprimir(&n.derecho, nivel + 1);
        println!("{:indent$}[ISBN: {}] {}", "", n.libro.isbn, n.libro.titulo, indent = nivel * 4);
        imprimir(&n.izquierdo, nivel + 1);
    }
}

// --- MAIN CON VALIDACIONES ---

fn main() {
    let mut raiz: Option<Box<Nodo>> = None;
    let datos = vec![
        (10, "El Quijote"), (20, "1984"), (30, "Hamlet"),
        (5, "Fahrenheit 451"), (2, "La Odisea"), (25, "El Principito"),
    ];

    for (isbn, titulo) in datos {
        let libro = Libro { isbn, titulo: titulo.to_string() };
        raiz = Some(insertar(raiz.take(), libro));
    }

    println!("--- Árbol Inicial ---");
    imprimir(&raiz, 0);

    // Validación Fase 2: Búsqueda
    println!("\n--- Fase 2: Pruebas de Búsqueda ---");
    match buscar(&raiz, 20) {
        Some(libro) => println!("Encontrado: {}", libro.titulo),
        None => println!("Libro 20 no encontrado"),
    }
    if buscar(&raiz, 99).is_none() {
        println!("Búsqueda de ISBN 99: Correctamente no encontrado.");
    }

    // Validación Fase 3: Eliminación
    println!("\n--- Fase 3: Eliminación de ISBN 20 (Nodo con hijios ) ---");
    raiz = eliminar(raiz, 20);
    imprimir(&raiz, 0);

    // Validación Fase 4: Rango
    println!("\n--- Fase 4: Búsqueda por Rango (ISBN 5 al 25) ---");
    let mut rango = Vec::new();
    buscar_rango(&raiz, 5, 25, &mut rango);
    for libro in rango {
        println!("En rango: [ISBN {}] {}", libro.isbn, libro.titulo);
    }
}