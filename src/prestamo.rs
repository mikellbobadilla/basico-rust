/**
 * Propiedad y Perstamo
 *
 * Es fundamental para garantizar la seguridad de memoria y evitar problemas
 * como las fugas de memoria o las condiciones de carrera.
 *
 * 1. Propiedad:
 * En Rust cada valor tiene una única propietaria en un momento dado.
 * Este es responsable de liberar la memoria asignada cuando ya no se necesita.
 * Cuando se transfiere la propiedad a otra variable, se dice que se ha realizado una "transferencia" de propiedad.
 * Esto se logra a través del operador de asignación (=) o mediante la llamada a funciones que aceptan el valor como parámetro.
 * Aquí un ejemplo:
 */
pub fn propiedad() {
    println!("Prestamo");
    // "s1" es la propietaria de la cadena
    let s1 = String::from("¡Hola¡");
    // Se realiza un transferencia de propiedad de s1 a s2;
    let s2 = s1;

    // Para evitar esto se puede duplicar el valor del String
    let s3 = String::clone(&s2);

    // Importante:
    // Si intentamos usar s1 aquí o después de una transferencia, generará un error de compilación
    // Porque s1 ya no es valida después de la transferencia de propiedad.

    // s2 es la nueva propietaria de la cadena
    println!("{}", s2);
    // s2 es liberado automáticamente al salir del ámbito.

    // En este caso s2 sigue siendo la propietaria del contenido de la variable
    println!("{}", s3);
}
