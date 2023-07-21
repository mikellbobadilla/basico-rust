use std::thread;

pub fn hilo() {
    let tarea_1 = thread::Builder::new()
        .name("Tarea 1".to_string())
        .stack_size(32 * 1024);
    tarea_1
        .spawn(|| {
            println!("EL proceso de este Hilo comenzó");
            let sum: i32 = 4 + 4;
            println!("El total de esta suma es: {}", sum);
            println!("El proceso de este Hilo terminó")
        })
        .unwrap();
}
