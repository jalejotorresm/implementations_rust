//Implementacion de la secuencia Fibonacci
pub fn fibonacci () {
    println!("\n--- Implementacion Fibonacci ---");
    println!("Cuantos numeros de la secuencia quieres ver:");

    let mut secuencia = secuencia();
    fib(&mut secuencia);
}

fn fib (numeros: &mut Vec<i32>) {
    println!("Los primeros {} numeros de la secuencia Fibonacci son: ", numeros.len());
    for i in 2..numeros.len(){
        numeros[i] = numeros[i-2] + numeros[i-1];
    }
    for i in 0..numeros.len(){
        print!("{}, ", numeros[i]);
    }

    print!("\n")
}

fn secuencia () -> Vec<i32> {
    let mut input = String::new();
    
    std::io::stdin().read_line(&mut input).expect("Error al leer tu inofrmacion.");

    let input: usize = input.trim().parse().expect("Entrada invalida.");

    let mut array = Vec::new();

    for _ in 0..input {
        //let elemento = i32::try_from(input).expect("Error en conversion");
        array.push(1);
    }

    array
}

//Implementacion de instancia Struct y metodos
pub fn persona () {
    println!("\n--- Implementacion Structs y Metodos ---");
    print!("\n");

    let mut humano = Persona{
        nombre: String::new(),
        edad: 0
    };
    
    humano.set_data();
    humano.saludar();
}

struct Persona {
    nombre: String,
    edad: u8,
}

impl Persona {
    fn saludar (&self) {
        println!("Hola, eres {} y tienes {} años de edad.", self.nombre, self.edad)
    }

    fn set_data (&mut self) {
        println!("Dime tu nombre:");
    
        let mut nombre = String::new();
        std::io::stdin().read_line(&mut nombre).expect("Necesito un nombre para continuar");
        self.nombre = nombre.trim().to_string();
    
        print!("\n");
        println!("Dime tu edad:");
    
        let mut edad = String::new();
        std::io::stdin().read_line(&mut edad).expect("Necesito tu edad para continuar");
        self.edad = edad.trim().parse().expect("Tu edad debe ser un numero.");
    
        print!("\n");
    }
}