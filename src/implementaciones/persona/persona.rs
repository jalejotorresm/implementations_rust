use crate::implementaciones::persona::data_persona::Persona;

pub fn persona() {
    println!("\n--- Implementacion Structs y Metodos ---");
    print!("\n");

    let mut humano = Persona::new();

    humano.set_data();
}
