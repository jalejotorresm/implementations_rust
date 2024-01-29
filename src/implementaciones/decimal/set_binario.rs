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

/*println!("Ingresa el número a convertir:");

let mut input = String::new();
std::io::stdin()
    .read_line(&mut input)
    .expect("\nNecesito un numero para seguir");
let input = input.trim();

if !verificar_bin(&input) {
    return (String::from(""), String::from(""), false);
} else {
    let input = input.to_string();
    let base = input.clone();
    return (input, base, true);
}*/

/*fn verificar_bin(input: &str) -> bool {
    for c in input.chars() {
        if c != '0' && c != '1' {
            return false;
        }
    }
    true
}*/
