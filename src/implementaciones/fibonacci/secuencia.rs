pub fn secuencia() -> Vec<i32> {
    let mut input = String::new();

    std::io::stdin()
        .read_line(&mut input)
        .expect("Error al leer tu inofrmacion.");

    let input: usize = input.trim().parse().expect("Entrada invalida.");

    let mut array = Vec::new();

    for _ in 0..input {
        //let elemento = i32::try_from(input).expect("Error en conversion");
        array.push(1);
    }

    array
}
