# FASE 1: Auditoría y Teoría Aplicada


# 2. Análisis de Rust ¿po rqué es necesario usar .take() en las funciones de rotación en lugar de una asignación directa?
En Rust, cada recurso en memoria tiene un único dueño. Durante las rotaciones de un árbol AVL, es necesario reestructurar los enlaces de los hijos de un nodo. Si intentáramos realizar una asignación directa (por ejemplo, nodo.izquierdo = otro_nodo), el compilador generaría un error grave de Ownership porque estaríamos intentando mover un dato que aún está anclado a la estructura original. La función take() soluciona esto al extraer el subárbol de forma segura, permitiendo su reubicación en la nueva posición sin necesidad de clonar datos en memoria.

# 3. Prueba de Escritorio 
Comenzamos con los primeros 3 datos 10,20,30
la rotacion sera a al izquierda sobre el nodo 10 
         [20]
       /      \
     [10]      [30]
insertamos el 5 y 2 seria 2,5,10 lo cual hace una rotacion a la derecha sobre el nodo 10
         [20]
       /      \
     [5]      [30]
    /   \     
  [2]  [10] 

insertamos el 25 
         [20]
       /      \
     [5]      [30]
    /   \     /
  [2]  [10] [25]
