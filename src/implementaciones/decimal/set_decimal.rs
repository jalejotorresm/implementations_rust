pub fn set_decimal(binario: (String, String)) {
    let base = binario.0.chars().rev().collect::<String>();
    let numero = binario.1;

    let mut decimal = 0;

    for (i, c) in base.chars().enumerate() {
        if c == '0' {
            continue;
        }

        let potencia = i as u32;
        let numero = 2_i32.pow(potencia);
        decimal += numero;
    }

    println!(
        "\nEl binario {} convertido a decimal es: {}\n",
        numero, decimal
    )
}
