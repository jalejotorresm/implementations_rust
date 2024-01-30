use crate::implementaciones::{
    binario::binario::binario, decimal::decimal::decimal, factorial::factorial::factorial,
    fibonacci::fibonacci::fibonacci, persona::persona::persona,
};
use crate::selector::opciones::opciones;

pub fn selector() {
    let seleccion = opciones();

    match seleccion {
        "fibonacci" => fibonacci(),
        "persona" => persona(),
        "binario" => binario(),
        "decimal" => decimal(),
        "factorial" => factorial(),
        _ => todo!(),
    }
}
