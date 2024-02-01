use inquire::Text;
use regex::Regex;

pub fn set_binario() -> (String, String) {
    let mut input = Text::new("Ingresa el número a convertir:\n")
        .prompt()
        .unwrap();

    let verificador = Regex::new(r"^[01]+$").unwrap();

    while !(verificador.is_match(&input)) {
        println!();
        input = Text::new("Informacion errada. Dame un numero binario valido por favor:\n")
            .prompt()
            .unwrap();
    }

    let base = input.clone();

    (input, base)
}
