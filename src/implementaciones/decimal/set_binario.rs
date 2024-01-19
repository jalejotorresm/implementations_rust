pub fn set_binario() -> (String, String, bool) {
    println!("Ingresa el número a convertir:");

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
    }
}

fn verificar_bin(input: &str) -> bool {
    for c in input.chars() {
        if c != '0' && c != '1' {
            return false;
        }
    }
    true
}