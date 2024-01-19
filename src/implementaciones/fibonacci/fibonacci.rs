use crate::implementaciones::fibonacci::secuencia::secuencia;
use crate::implementaciones::fibonacci::fib::fib;

pub fn fibonacci() {
    println!("\n--- Calculo de Secuencia Fibonacci ---\n");
    println!("Cuantos numeros de la secuencia quieres ver:");

    let mut secuencia = secuencia();
    fib(&mut secuencia);
}