use crate::implementaciones::persona::data_persona::Persona;

pub fn persona() {
    println!("\n--- Implementacion Structs y Metodos ---");
    print!("\n");

    let mut humano = Persona {
        nombre: String::new(),
        edad: 0,
    };

    humano.set_data();
    humano.saludar();
}

