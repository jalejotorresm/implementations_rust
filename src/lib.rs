use num_bigint::BigUint;
use regex::Regex;

//Selector de implementaciones
pub fn selector () {
    fn mensaje_selector () {
        println!("Estas son las opciones disponibles: \n");
        println!("1 - Calcula la Secuencia de Fibonacci");
        println!("2 - Crea un struct de persona con tu nombre y hazlo saludar");
        println!("3 - Convierte un numero decimal a binario");
        println!("4 - Convierte un numero binario a decimal");
        println!("5 - Calcula el factorial de un numero");
        println!("\nSelecciona la opcion que deseas explorar:");

        opciones()
    }

    fn opciones () {
        let mut input = String::new();

        std::io::stdin().read_line(&mut input).expect("Necesito una opcion para continuar");

        let input = input.trim();

        let regex = Regex::new(r"^[0-9]+$").unwrap();

        if !(regex.is_match(input)) {
            println!("\nLa opcion que me diste no es un numero. Intenta mas tarde.");
            std::process::exit(1)
        }

        let opcion = input.parse().expect("Esperaba un numero");

        match opcion {
            1 => fibonacci(),
            2 => persona(),
            3 => binario(),
            4 => decimal(),
            5 => factorial(),
            _ => opcion_invalida(),
        }

        fn opcion_invalida () {
            println!("La opcion que me indicaste no es la correcta. Intenta mas tarde.");
            std::process::exit(1)
        }
    }

    mensaje_selector()
}

//Implementacion de la secuencia Fibonacci
pub fn fibonacci() {
    fn imp_fibonacci() {
        println!("\n--- Calculo de Secuencia Fibonacci ---\n");
        println!("Cuantos numeros de la secuencia quieres ver:");

        let mut secuencia = secuencia();
        fib(&mut secuencia);
    }

    fn fib(numeros: &mut Vec<i32>) {
        println!(
            "\nLos primeros {} numeros de la secuencia Fibonacci son: ",
            numeros.len()
        );
        for i in 2..numeros.len() {
            numeros[i] = numeros[i - 2] + numeros[i - 1];
        }
        for i in 0..numeros.len() {
            print!("{}, ", numeros[i]);
        }

        print!("\n")
    }

    fn secuencia() -> Vec<i32> {
        let mut input = String::new();

        std::io::stdin()
            .read_line(&mut input)
            .expect("Error al leer tu inofrmacion.");

        let input: usize = input.trim().parse().expect("Entrada invalida.");

        let mut array = Vec::new();

        for _ in 0..input {
            //let elemento = i32::try_from(input).expect("Error en conversion");
            array.push(1);
        }

        array
    }

    imp_fibonacci()
}

//Implementacion de instancia Struct y metodos
pub fn persona() {
    fn imp_persona() {
        println!("\n--- Implementacion Structs y Metodos ---");
        print!("\n");

        let mut humano = Persona {
            nombre: String::new(),
            edad: 0,
        };

        humano.set_data();
        humano.saludar();
    }

    struct Persona {
        nombre: String,
        edad: u8,
    }

    impl Persona {
        fn saludar(&self) {
            println!(
                "Hola, eres {} y tienes {} años de edad.",
                self.nombre, self.edad
            )
        }

        fn set_data(&mut self) {
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

    imp_persona();
}

//Conversion de Decimal a Binario
pub fn binario() {
    fn a_binario() {
        println!("\n--- Conversor de Decimal a Binario ---\n");
        let numero = set_decimal();
        volver_bin(numero);
    }

    fn set_decimal() -> (i32, i32) {
        println!("Ingresa el número a convertir:");

        let mut numero = String::new();
        std::io::stdin()
            .read_line(&mut numero)
            .expect("\nNecesito un numero para seguir");
        let numero = numero.trim().parse().expect("\nEsto deberia ser un numero.");

        let base = numero;
        (base, numero)
    }

    fn volver_bin(decimal: (i32, i32)) {
        let mut base = decimal.0;
        let numero = decimal.1;
        let mut binario = String::new();

        while base > 0 {
            let digit = base % 2;
            binario.push_str(&digit.to_string());
            base /= 2;
        }

        let binario = binario.as_str().chars().rev().collect::<String>();

        println!("\nEl número {} convertido en binario es: {}", numero, binario);
    }

    a_binario()
}

//Conversion de Binario a Decimal
pub fn decimal() {
    fn a_decimal() {
        println!("\n--- Conversor de Binario a Decimal ---\n");
        let numero = set_binario();
        volver_dec(numero);
    }

    fn set_binario() -> (String, String, bool) {
        println!("Ingresa el número a convertir:");

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("\nNecesito un numero para seguir");
        let input = input.trim();

        if !verificar_bin(&input) {
            return (String::from(""), String::from(""), false);
        } else {
            let input = input.to_string();
            let base = input.clone();
            return (input, base, true);
        }
    }

    fn verificar_bin(input: &str) -> bool {
        for c in input.chars() {
            if c != '0' && c != '1' {
                return false;
            }
        }
        true
    }

    fn volver_dec(binario: (String, String, bool)) {
        if !binario.2 {
            println!("\nEl numero que ingresaste no es binario.");
            return;
        }

        let base = binario.0.chars().rev().collect::<String>();
        let numero = binario.1;

        let mut decimal = 0;

        for (i, c) in base.chars().enumerate() {
            if c == '0' {
                continue;
            }

            let potencia = i as u32;
            let numero = 2_i32.pow(potencia);
            decimal += numero;
        }

        println!("\nEl binario {} convertido a decimal es: {}", numero, decimal)
    }

    a_decimal();
}

//Calculo de un factorial
pub fn factorial () {
    fn hacer_factorial () {
        print!("\n--- Calculadora de Factoriales ---\n");

        print_factorial();
    }

    fn set_numero () -> u128 {
        println!("\nDame un numero para calcular: ");
        let mut input = String::new();

        std::io::stdin().read_line(&mut input).expect("\nNecesito informacion para comenzar");

        let input = input.trim();

        let verificador = Regex::new(r"^-[0-9]+$").unwrap();

        if verificador.is_match(&input) {
            println!("\nEste programa no recibe numeros negativos, letras y/o simbolos");
            std::process::exit(1)
        }

        let base = input.parse().expect("\nEsperaba un numero");

        base
    }

    fn set_factorial (numero: u128) -> BigUint {
        let base = BigUint::from(numero);

        let cero: BigUint = BigUint::from(0u8);

        let uno: BigUint = BigUint::from(1u8);

        match base {
            x if x == cero || x == uno => uno,
            _ => base * set_factorial(numero -1) 
        }
    }

    fn print_factorial () {
        let base = set_numero();
        let factorial = set_factorial(base);

        println!("\nEl factorial de {} es: {}\n", base, factorial);
    } 

    hacer_factorial();
}