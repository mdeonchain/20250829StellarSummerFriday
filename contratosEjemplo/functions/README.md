**Funciones en  Soroban**
## Calculator Contract - Soroban

Un contrato inteligente simple implementado en Soroban (Stellar) que proporciona operaciones básicas de calculadora y almacenamiento de resultados.

## 📋 Descripción

## Calculator Contract - Soroban

Un contrato inteligente simple implementado en Soroban (Stellar) que proporciona operaciones básicas de calculadora y almacenamiento de resultados.

## 📋 Descripción

Este contrato demuestra las capacidades fundamentales de Soroban, incluyendo:

* Operaciones matemáticas básicas
* Almacenamiento persistente de datos
* Interacción con el entorno de Soroban

## 🚀 Funcionalidades

### `greet()`

Retorna un saludo simple.

**Parámetros:** Ninguno

**Retorna:** `String` - "Hello Soroban!"

 

```plaintext
let greeting = client.greet(); // Resultado: "Hello Soroban!"
```

### `add(a: i32, b: i32)`

Suma dos números enteros.

**Parámetros:**

* `a`: Primer número (i32)
* `b`: Segundo número (i32)

**Retorna:** `i32` - La suma de a + b

```plaintext
let result = client.add(&5, &3); // Resultado: 8
```

### `multiply(a: i32, b: i32)`

Multiplica dos números enteros.

**Parámetros:**

* `a`: Primer número (i32)
* `b`: Segundo número (i32)

**Retorna:** `i32` - El producto de a \* b

```plaintext
let result = client.multiply(&4, &6); // Resultado: 24
```

### `calculate_and_store(a: i32, b: i32)`

Suma dos números y almacena el resultado en el storage del contrato.

**Parámetros:**

* `a`: Primer número (i32)
* `b`: Segundo número (i32)

**Retorna:** `i32` - La suma calculada

**Nota:** El resultado se guarda con la clave `last_calc` en el instance storage.

```plaintext
let result = client.calculate_and_store(&10, &20); // Resultado: 30 (también almacenado)
```

### `get_last_calculation()`

Recupera el último cálculo almacenado.

**Parámetros:** Ninguno

**Retorna:** `i32` - El último resultado almacenado, o 0 si no hay ninguno

 

```plaintext
let last_result = client.get_last_calculation(); // Resultado: el último valor almacenado
```

## 🧪 Tests

El contrato incluye una suite completa de tests que validan todas las funcionalidades:

### Ejecutar los tests

bash

```plaintext
cargo test -- --nocapture
```

### Tests incluidos

1. **test\_greet()** - Verifica que el saludo sea correcto
2. **test\_add()** - Valida la suma de dos números (5 + 3 = 8)
3. **test\_multiply()** - Valida la multiplicación (4 × 6 = 24)
4. **test\_calculate\_and\_store()** - Verifica que el cálculo y almacenamiento funcionen (10 + 20 = 30)
5. **test\_get\_last\_calculation()** - Confirma que se puede recuperar el último cálculo almacenado (15 + 25 = 40)

### Estructura de los tests

Todos los tests siguen el mismo patrón:

rust

```plaintext
#[test] fn test_example() {   
    // 1. Crear entorno de prueba     let env = Env::default();     
    // 2. Registrar el contrato     let contract_id = env.register(CalculatorContract, ());     
    // 3. Crear cliente del contrato     let client = CalculatorContractClient::new(&env, &contract_id);     
    // 4. Ejecutar función y verificar resultado     let result = client.some_function(&param1, &param2);     assert_eq!(result, expected_value); }
```

## 🔍 Estructura del Proyecto
```plaintext
calculator-contract/
├── src/
│   ├── lib.rs          # Implementación del contrato
│   └── test.rs         # Suite de tests
├── Cargo.toml
└── README.md
```
##


⬅️[**Contratos Ejemplo** ](../README.md) 
---
