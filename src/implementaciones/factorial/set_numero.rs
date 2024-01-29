use inquire::Text;
use regex::Regex;

pub fn set_numero() -> u128 {
    let mut input = Text::new("Dame un numero para calcular:\n")
        .prompt()
        .unwrap();

    let verificador = Regex::new(r"^[0-9]+$").unwrap();

    while !(verificador.is_match(&input)) {
        println!();
        input = Text::new("Respuesta errada. Dame un numero entero positivo por favor:\n")
            .prompt()
            .unwrap();
    }

    let numero = input.trim().parse().unwrap();

    numero
}
