use inquire::Text;
use regex::Regex;

pub fn set_decimal() -> (i32, i32) {
    let mut numero = Text::new("Ingresa el número a convertir:\n")
        .prompt()
        .unwrap();

    let verificador = Regex::new(r"^[0-9]+$").unwrap();

    while !(verificador.is_match(&numero)) {
        println!();
        numero = Text::new("Informacion incorrecta. Ingresa un numero positivo por favor:\n")
            .prompt()
            .unwrap();
    }

    let numero = numero.trim().parse().unwrap();

    let base = numero;
    (base, numero)
}
