pub fn set_decimal() -> (i32, i32) {
    println!("Ingresa el número a convertir:");

    let mut numero = String::new();
    std::io::stdin()
        .read_line(&mut numero)
        .expect("\nNecesito un numero para seguir");
    let numero = numero.trim().parse().expect("\nEsto deberia ser un numero.");

    let base = numero;
    (base, numero)
}