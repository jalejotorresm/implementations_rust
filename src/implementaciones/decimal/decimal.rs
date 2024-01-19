use crate::implementaciones::decimal::{set_binario::set_binario, set_decimal::set_decimal};

pub fn decimal() {
    println!("\n--- Conversor de Binario a Decimal ---\n");
    let numero = set_binario();
    set_decimal(numero);
}