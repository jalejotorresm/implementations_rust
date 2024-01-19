use crate::implementaciones::binario::{set_decimal::set_decimal, set_binario::set_binario};

pub fn binario() {
    println!("\n--- Conversor de Decimal a Binario ---\n");
    let numero = set_decimal();
    set_binario(numero);
}