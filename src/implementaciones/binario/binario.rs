use crate::implementaciones::binario::{set_binario::set_binario, set_decimal::set_decimal};

pub fn binario() {
    println!("\n--- Conversor de Decimal a Binario ---\n");
    let numero = set_decimal();
    set_binario(numero);
}
