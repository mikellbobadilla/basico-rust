mod prestamo;
mod punteros;
mod hilos;

use prestamo::propiedad;

use punteros::{punteros, punteros_crudos};

use crate::hilos::hilo;

fn main() {
    println!("---------------- prestamos ----------------------");
    propiedad();
    println!("---------------- prestamos ----------------------");

    println!("---------------- punteros ----------------------");
    punteros_crudos();
    punteros();
    println!("---------------- punteros ----------------------");

    println!("---------------- hilos ----------------------");
    hilo();
    println!("---------------- hilos ----------------------");
    
}
