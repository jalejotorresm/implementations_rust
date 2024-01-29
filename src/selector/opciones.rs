use inquire::Select;

pub fn opciones() -> &'static str {
    let opciones = vec![
        "Calculo de Secuencia de Fibonacci",
        "Implementacion de Persona",
        "Conversor Decimal a Binario",
        "Conversor Binario a Decimal",
        "Calculadora de Factoriales",
    ];

    let seleccion = Select::new("Selecciona la opcion que deseas explorar:", opciones)
        .without_help_message()
        .prompt()
        .unwrap();

    let eleccion = match seleccion {
        "Calculo de Secuencia de Fibonacci" => "fibonacci",
        "Implementacion de Persona" => "persona",
        "Conversor Decimal a Binario" => "binario",
        "Conversor Binario a Decimal" => "decimal",
        "Calculadora de Factoriales" => "factorial",
        _ => todo!(),
    };

    eleccion
}
