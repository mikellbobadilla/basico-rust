mod prestamo;
mod punteros;

use prestamo::propiedad;

use punteros::{punteros, punteros_crudos};

fn main() {
    println!("---------------- prestamos ----------------------");
    propiedad();
    println!("---------------- prestamos ----------------------");

    println!("---------------- punteros ----------------------");
    punteros_crudos();
    punteros();
    println!("---------------- punteros ----------------------");

}
