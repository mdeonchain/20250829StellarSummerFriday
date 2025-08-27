**Fundamentos de RUST 🦀**

Todo el código lo correremos en el playground de Rust, no necesitamos instalar absolutamente nada

[**https://play.rust-lang.org/**](https://play.rust-lang.org/)

Primeros pasos en Rust.

**Pimer hola mundo:**

```plaintext
fn main() {
    println!("Hello, world!");
}
```

**Declaración de Variables**

```plaintext
fn main(){
   // Variable inmutable (por defecto)
   let nombre = "Juan";
   println!("nombre {}",nombre);

   // Variable mutable
   let mut edad = 25;
   println!("edad: {}",edad);
   edad = 26; // ✅ Permitido
   println!("nueva edad: {}",edad);

   // Constante
   const PI: f64 = 3.14159;
   println!("PI: {}",PI);
}
```

**Tipos Básicos**

```plaintext
fn main(){
  // Números enteros
  let numero: i32 = 42;
  let grande: u64 = 1000000;

  // Números decimales
  let precio: f64 = 19.99;

  // Texto
  let mensaje: String = String::from("Hola Soroban");
  let letra: char = 'A';

  // Booleano
  let activo: bool = true;
  println!("numero: {} grande: {} precio: {}",numero,grande,precio);
  println!("mensaje: {} letra: {} activo: {}",mensaje,letra,activo);
}
```

**Funciones**

```plaintext
// Función simple 
fn saludar() {
  println!("¡Hola mundo!"); 
}

// Función con parámetros 
fn sumar(a: i32, b: i32) -> i32 {
  // Sin punto y coma = return 
    a + b 
}

fn main() { 
       saludar(); 
       let resultado = sumar(5, 3); 
       println!("5 + 3 = {}", resultado);
 }
```

**If/Else**

```plaintext
fn evaluar_edad(edad: u32) {
   if edad < 13{
        println!("Eres menor de edad");
   }
   else if edad >= 13 && edad < 18{ 
             println!("Eres adolescente"); 
   } 
   else { 
         println!("Eres mayor de edad");  
   }
 }

fn main(){
  evaluar_edad(18);
}
```

**Structs (Estructuras)**  
Sirven para agrupar datos relacionados en un solo tipo.

```plaintext
struct Persona {
   nombre: String,
   edad: u32,
}
fn main() {
   // Crear una instancia de Persona
   let juan = Persona {
       nombre: String::from("Juan"),
       edad: 30,
   };
   println!("Nombre: {}, Edad: {}", juan.nombre, juan.edad);
}
```

## Enums (Enumeraciones)

Se usan para definir un conjunto de opciones posibles.

```plaintext
enum Estado {
   Activo,
   Inactivo,
   Pendiente,
}
fn imprimir_estado(estado: Estado) {
   match estado {
       Estado::Activo => println!("El estado es ACTIVO ✅"),
       Estado::Inactivo => println!("El estado es INACTIVO ❌"),
       Estado::Pendiente => println!("El estado está PENDIENTE ⏳"),
   }
}
fn main() {
   let estado_usuario = Estado::Pendiente;
   imprimir_estado(estado_usuario);
}
```

## Vec y HashMap

`Vec` es una lista dinámica.  
`HashMap` almacena pares clave-valor.

```plaintext
use std::collections::HashMap;
fn main() {
   // Vector
   let mut numeros: Vec<i32> = Vec::new();
   numeros.push(10);
   numeros.push(20);
   numeros.push(30);
   println!("Vector: {:?}", numeros);
   // HashMap
   let mut edades: HashMap<&str, u32> = HashMap::new();
   edades.insert("Juan", 30);
   edades.insert("Ana", 25);
   println!("Edad de Juan: {:?}", edades.get("Juan"));
}
```

## Result\<T, E>

Se usa para manejar errores.

```plaintext
fn dividir(a: i32, b: i32) -> Result<i32, String> {
   if b == 0 {
       Err(String::from("No se puede dividir entre 0"))
   } else {
       Ok(a / b)
   }
}

fn main() {
   match dividir(10, 2) {
       Ok(resultado) => println!("Resultado: {}", resultado),
       Err(error) => println!("Error: {}", error),
   }
   match dividir(5, 0) {
       Ok(resultado) => println!("Resultado: {}", resultado),
       Err(error) => println!("Error: {}", error),
   }
}
```

## Option\<T>

Se usa para valores que pueden existir o no (`Some` o `None`).

```plaintext
fn buscar_usuario(id: u32) -> Option<&'static str> {
   if id == 1 {
       Some("Juan")
   } else {
       None
   }
}
fn main() {
   match buscar_usuario(1) {
       Some(nombre) => println!("Usuario encontrado: {}", nombre),
       None => println!("Usuario no encontrado"),
   }
   match buscar_usuario(2) {
       Some(nombre) => println!("Usuario encontrado: {}", nombre),
       None => println!("Usuario no encontrado"),
   }
}
```

---

📚 **Más información de Rust:**

[👉 The Rust Book en español](https://book.rustlang-es.org)
