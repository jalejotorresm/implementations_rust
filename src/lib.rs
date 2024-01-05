pub fn fibonacci () {
    println!("\n--- Implementacion Fibonacci ---");
    println!("Cuantos numeros de la secuencia quieres ver:");

    let mut secuencia = secuencia();
    fib(&mut secuencia);
}

fn fib (numeros: &mut Vec<i32>) {
    println!("Los primeros {} numeros de la secuencia Fibonacci son: ", numeros.len());
    for i in 2..numeros.len(){
        numeros[i] = numeros[i-2] + numeros[i-1];
    }
    for i in 0..numeros.len(){
        print!("{}, ", numeros[i]);
    }

    print!("\n")
}

fn secuencia () -> Vec<i32> {
    let mut input = String::new();
    
    std::io::stdin().read_line(&mut input).expect("Error al leer tu inofrmacion.");

    let input: usize = input.trim().parse().expect("Entrada invalida.");

    let mut array = Vec::new();

    for _ in 0..input {
        //let elemento = i32::try_from(input).expect("Error en conversion");
        array.push(1);
    }

    array
}
