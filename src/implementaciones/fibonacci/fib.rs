pub fn fib(numeros: &mut Vec<i32>) {
    println!(
        "\nLos primeros {} numeros de la secuencia Fibonacci son: ",
        numeros.len()
    );
    for i in 2..numeros.len() {
        numeros[i] = numeros[i - 2] + numeros[i - 1];
    }
    for i in 0..numeros.len() {
        if i < (numeros.len() - 2){
            print!("{}, ", numeros[i]);
        } else if i == (numeros.len()-2) {
            print!("{} y ", numeros[i])
        } else {
            print!("{}.", numeros[i])
        }
    }

    print!("\n")
}
