pub struct Persona {
    pub nombre: String,
    pub edad: u8,
}

impl Persona {
    pub fn saludar(&self) {
        println!(
            "Hola, eres {} y tienes {} años de edad.",
            self.nombre, self.edad
        )
    }

    pub fn set_data(&mut self) {
        println!("Dime tu nombre:");

        let mut nombre = String::new();
        std::io::stdin()
            .read_line(&mut nombre)
            .expect("Necesito un nombre para continuar");
        self.nombre = nombre.trim().to_string();

        print!("\n");
        println!("Dime tu edad:");

        let mut edad = String::new();
        std::io::stdin()
            .read_line(&mut edad)
            .expect("Necesito tu edad para continuar");
        self.edad = edad.trim().parse().expect("Tu edad debe ser un numero.");

        print!("\n");
    }
}
