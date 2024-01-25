use crate::implementaciones::{
    binario::binario::binario, decimal::decimal::decimal, factorial::factorial::factorial,
    fibonacci::fibonacci::fibonacci, persona::persona::persona,
};
use regex::Regex;

pub fn opciones() {
    let mut input = String::new();

    std::io::stdin()
        .read_line(&mut input)
        .expect("Necesito una opcion para continuar");

    let input = input.trim();

    let regex = Regex::new(r"^[0-9]+$").unwrap();

    if !(regex.is_match(input)) {
        println!("\nLa opcion que me diste no es un numero. Intenta mas tarde.");
        std::process::exit(1)
    }

    let opcion = input.parse().expect("Esperaba un numero");

    match opcion {
        1 => fibonacci(),
        2 => persona(),
        3 => binario(),
        4 => decimal(),
        5 => factorial(),
        _ => opcion_invalida(),
    }

    fn opcion_invalida() {
        println!("La opcion que me indicaste no es la correcta. Intenta mas tarde.");
        std::process::exit(1)
    }
}
