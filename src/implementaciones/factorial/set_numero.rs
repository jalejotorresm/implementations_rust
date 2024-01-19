use regex::Regex;

pub fn set_numero () -> u128 {
    println!("\nDame un numero para calcular: ");
    let mut input = String::new();

    std::io::stdin().read_line(&mut input).expect("\nNecesito informacion para comenzar");

    let input = input.trim();

    let verificador = Regex::new(r"^-[0-9]+$").unwrap();

    if verificador.is_match(&input) {
        println!("\nEste programa no recibe numeros negativos, letras y/o simbolos");
        std::process::exit(1)
    }

    let base = input.parse().expect("\nEsperaba un numero");

    base
}