/*Importando "crate" (biblioteca) env por medio de use, que es la biblioteca estándar de Rust */
use std::env;

/*Función add toma como parametros a y b de tipo u64.
Regresa un valor de tipo u64, como lo indica el -> u64
u64 indica un entero sin signo de 64 bits de longitud.
los enteros son un tipo primitivo de Rust.
Aunque return existe en Rust, se usa si queremos regresar
un dato antes de que termine la función.
 */
fn add(a: u64, b: u64) -> u64
{
    a+b
}

/*Inicia función main */
fn main()
{
    /*Guardamos nombre que se pasó como argumento en name. args() regresa secuencia de datos 
    que se pasan como argumento. skip(1) para saltarnos el nombre del programa. next() para 
    mover el iterador al nombre */
    let name = env::args().skip(1).next();
    /*Match es similar a if else. Queremos comprobar si hay un dato almacenado en name y hacer
    algo a partir de ese hecho. */
    match name
    {
        /*Comprobamos que hay algo en name con Some. n es name.
        println! no es función, sino macro.
        {} es especificador de formato. Funciona igual que "%d" en C.
        Variable que se imprimirá va fuera de la cadena de texto separado por una coma */
        Some(n) => println!("Hi there ! {}", n),
        /*None se ejcuta si name no tiene datos. 
        Panic! también es un macro. Detiene el programa y lanza un mensaje de error. */
        None => panic!("Didn't receive any name ?")
    }
    let target = "world";   //Variable inmutable. Una vez que se ha declarado, su valor ya no puede cambiar
    let mut greeting = "Hello"; //Variable mutable, puede cambiar valor.
    println!("{},{}", greeting, target);
    greeting = "Goodbye cruel";
    println!("{},{}", greeting, target);   //El valor de greeting cambia
    let a: u64 = 17;
    let b = 3;
    let result = add(a,b);  //Usando la función add en main y guardando su resultado en result
    println!("Result: {}", result);
}