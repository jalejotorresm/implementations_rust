use crate::implementaciones::factorial::set_numero::set_numero;
use num_bigint::BigUint;

fn set_factorial(numero: u128) -> BigUint {
    let base = BigUint::from(numero);

    let cero: BigUint = BigUint::from(0u8);

    let uno: BigUint = BigUint::from(1u8);

    match base {
        x if x == cero || x == uno => uno,
        _ => base * set_factorial(numero - 1),
    }
}

pub fn print_factorial() {
    let base = set_numero();
    let factorial = set_factorial(base);

    println!("\nEl factorial de {} es: {}\n", base, factorial);
}
