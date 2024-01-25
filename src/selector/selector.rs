use crate::selector::opciones::opciones;

pub fn selector() {
    println!("Estas son las opciones disponibles: \n");
    println!("1 - Calcula la Secuencia de Fibonacci");
    println!("2 - Crea un struct de persona con tu nombre y edad y hazlo saludar");
    println!("3 - Convierte de un numero decimal a binario");
    println!("4 - Convierte de un numero binario a decimal");
    println!("5 - Calcula el Factorial de un numero");
    println!("\nSelecciona la opcion que deseas explorar:");

    opciones()
}
