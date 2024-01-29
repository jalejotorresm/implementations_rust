use inquire::Text;
use regex::Regex;

pub struct Persona {
    pub nombre: String,
    pub edad: u8,
}

impl Persona {
    pub fn saludar(&self) {
        println!(
            "Hola, eres {} y tienes {} años de edad.\n",
            self.nombre, self.edad
        )
    }

    pub fn set_data(&mut self) {
        let mut nombre = Text::new("Dime tu nombre:\n").prompt().unwrap();

        let nombre_test = Regex::new(r"^[a-zA-ZÀ-ÿ\u00f1\u00d1]+$").unwrap();

        while !(nombre_test.is_match(&nombre)){
            println!();
            nombre = Text::new("Informacion incorrecta. Dime tu nombre por favor:\n").prompt().unwrap();
        }

        self.nombre = nombre.trim().to_string();

        println!();

        let mut edad = Text::new("Dime tu edad:\n").prompt().unwrap();

        let edad_test = Regex::new(r"^[0-9]+$").unwrap();

        while !(edad_test.is_match(&edad)) {
            println!();
            edad = Text::new("Informacion incorrecta. Dime tu edad por favor:\n").prompt().unwrap();
        }

        self.edad = edad.trim().parse().unwrap();

        println!();

        self.saludar()
    }
}
