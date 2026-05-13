# FASE 1: Auditoría y Teoría Aplicada

# 1. Documentación Técnica 
1.Box<Nodo>: Es un puntero inteligente que almacena el nodo en la memoria Heap (dinámica). Se utiliza aquí porque los árboles AVL son estructuras recursivas y su tamaño no se conoce en tiempo de compilación; Box otorga un tamaño fijo a la referencia.

2.as_ref(): Permite inspeccionar el valor interno de un Option (si es Some o None) devolviendo una referencia al dato guardado, evitando que el compilador transfiera la propiedad (ownership) y destruya el objeto original.

3.take(): Extrae el valor dentro de un Option dejando un None en su lugar de forma temporal. Es fundamental para desvincular un nodo de su posición actual sin romper las reglas de propiedad de Rust.

# 2. Análisis de Rust ¿po rqué es necesario usar .take() en las funciones de rotación en lugar de una asignación directa?
En Rust, cada recurso en memoria tiene un único dueño. Durante las rotaciones de un árbol AVL, es necesario reestructurar los enlaces de los hijos de un nodo. Si intentáramos realizar una asignación directa (por ejemplo, nodo.izquierdo = otro_nodo), el compilador generaría un error grave de Ownership porque estaríamos intentando mover un dato que aún está anclado a la estructura original. La función take() soluciona esto al extraer el subárbol de forma segura, permitiendo su reubicación en la nueva posición sin necesidad de clonar datos en memoria.

# 3. Prueba de Escritorio 
Orden de inserción de ISBNs: [10, 20, 30, 5, 2, 25]

1. Insertar 10: Se crea el nodo raíz.
2. Insertar 20: Se ubica a la derecha de 10.
3. Insertar 30: Se ubica a la derecha de 20. El árbol se desbalancea en el nodo 10 (factor de balance = -2). *Ocurre una Rotación Simple a la Izquierda en el nodo 10.
   * Estado temporal: Raíz [20], Hijo Izquierdo [10], Hijo Derecho [30].
4. Insertar 5: Se ubica a la izquierda de 10.
5. Insertar 2: Se ubica a la izquierda de 5. El subárbol se desbalancea en el nodo 10 (factor de balance = 2). *Ocurre una Rotación Simple a la Derecha en el nodo 10.
   * Estado temporal: El nodo 5 pasa a ser el padre de 2 y de 10.
6. Insertar 20: Se ubica a la izquierda de 30. El árbol final cumple con la propiedad AVL.

Estado Final del Árbol:
```text
         [20]
       /      \
     [5]      [30]
    /   \     /
  [2]  [10] [25]
