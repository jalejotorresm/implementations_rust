pub fn set_binario(decimal: (i32, i32)) {
    let mut base = decimal.0;
    let numero = decimal.1;
    let mut binario = String::new();

    while base > 0 {
        let digit = base % 2;
        binario.push_str(&digit.to_string());
        base /= 2;
    }

    let binario = binario.as_str().chars().rev().collect::<String>();

    println!("\nEl número {} convertido en binario es: {}", numero, binario);
}