use inquire::Text;
use regex::Regex;

pub fn secuencia() -> Vec<i32> {
    let mut input = Text::new("Cuantos numeros de la secuencia quieres ver:\n")
        .prompt()
        .unwrap();

    let verificador = Regex::new(r"^[0-9]+$").unwrap();

    while !(verificador.is_match(&input)) {
        println!();
        input = Text::new("Informacion errada. Necesito un numero positivo para continuar:\n")
            .prompt()
            .unwrap();
    }

    let input = input.trim().parse().unwrap();

    let mut numeros = Vec::new();

    for _ in 0..input {
        numeros.push(1);
    }

    numeros
}
