/**
 * Los punteros crudos("*const T y *mut T") en Rust son similares
 * a los punteros de otros lenguajes de programación, ya que almacenan
 * direcciones de memoria. Sin embargo, a diferencia de otros lenguajes,
 * Rust impone reglas estrictas para garantizar la seguridad de memoria
 *
 * Punteros Crudos:
 * Permiten eludir las restricciones de seguridad de Rust, lo que significa
 * que puedes acceder a y modificar la memoria de forma directa. Esto puede
 * ser necesario en situaciones especificas, como la interacción con código C
 * o el acceso a memoria externa controlada por hardware, pero la mayoría
 * de los casos no se recomienda su uso.
 * Ejemplo:
 */
pub fn punteros_crudos() {
    let x: i32 = 42;
    let ptr: *const i32 = &x;

    // Esto es necesario para indicar que se está accediendo directamente
    // a la memoria a través del puntero crudo. Finalmente se desreferencia el puntero
    // usando '*ptr' para acceder al valor almacenado en la dirección de memoria.
    unsafe {
        println!("Valor almacenado en la dirección de memoria: {}", *ptr);
    }
}

/**
 * Uso de referencias en Rust. La referencias ('&T' y '&mut T') son la forma
 * preferida de trabajar con datos en Rust, ya que garantizan la seguridad de memoria en tiempo
 * de compilación mediante el sistema de préstamos de referencias.
 * Ejemplo:
 */
pub fn punteros() {
    let mut x: i32 = 10;
    let ref1: &i32 = &x;
    println!("Valor de ref1: {}", *ref1);

    let ref2: &mut i32 = &mut x;
    *ref2 += 5;
    println!("Nuevo valor de x: {}", x);

    // Valor de las posiciones de memoria (formato Hexadecimal)
    let hex:i64 = 0xe762cff2b4;

    println!("Numero Hexadecimal: {:0x}", hex);

    // Puntero inmutable
    let v: i32 = 20;
    let ref_v: &i32 = &v;
    println!("Valor de v: {}", v);

    println!("Dirección de memoria donde esta v: {:p}", ref_v);
}   
