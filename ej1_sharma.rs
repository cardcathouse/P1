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

/*Función que puede modificar sus argumentos de entrada.
val es una variable que puede cambiar su valor. Es un
entero sin signo de 32 bits, como lo indica u32. */
fn increase_by(mut val: u32, how_much: u32)
{
    val+= how_much;
    println!("You made {} points", val);
}

/*Función para demostrar etiquetado de loops. Ver más abajo. */
fn silly_sub(a: i32, b: i32) -> i32
{
    let mut result = 0;
    'increment: loop
    {
        if result == a
        {
            let mut dec = b;
            'decrement: loop
            {
                if dec == 0
                {
                    // breaks directly out of 'increment loop
                    break 'increment;
                }
                else
                {
                    result -= 1;
                    dec -= 1;
                }
            }
        } 
        else
        {
            result += 1;
        }
    }
            result
}

/*Inicia función main */
fn main()
{
    /*Guardamos nombre que se pasó como argumento en name. args() regresa secuencia de datos 
    que se pasan como argumento. skip(1) para saltarnos el nombre del programa. next() para 
    mover el iterador al nombre */
    let name = env::args().skip(1).next();
    /*Match es similar a switch en C. Queremos comprobar si hay un dato almacenado en name y hacer
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

    /*Usando función increase_by */
    let score = 2048;
    increase_by(score,1000);

    /*
    Utilizando closures.

    Son como funciones. Tienen más información del ámbito
    o ambiente en el que se declaran. Se asocian a una variable, a diferencia de una
    funcióm, que tiene un nombre asociado a ella. 

    Los parámetros del closure van dentro de las barras verticales ||. 

    El cuerpo del clousure puede contener una sola línea o varías dentro de corchetes.

    Se usan en funciones de órden superior como argumento. Por ejemplo, thread::spawn
    toma un closure como argumento en el que se incluye código que se quiera correr en un hilo.
    */

    let doubler = |x| x * 2;
    let value = 5;
    let twice = doubler(value);
    println!("{} doubled is {}", value, twice);
     /*Ahora vamos a crear un closure con más líneas dentro de su cuerpo */
    let big_closure = |b, c|
    {
        let z = b + c;
        z * twice
    };  // Este closure con instrucciones entre corchetes necesita terminar con ;
    let some_number = big_closure(1, 2);
    println!("Result from closure: {}", some_number);

    /*
    Estructuras condicionales

    La estructura if-else siempre regresa un valor por ser una expresión
    El valor que regresa puede estar vacío

    Lo que sea que haya en la última línea antes del corchete de cierre 
    en if o else se convierte en el valor de regreso.

    Los valores de retorno de if y else deben de ser del mismo tipo
    No se necesitan paréntesis para la condición de la estructura887
    */

    let result = if 1 == 1
    {
        "One equals one"
    }
    else
    {
        "This is not true"
    };
    println!("This is a fact: {}", result);
    
    //Ejemplo de if-else usando la estrctura por sí sola
    let mut result = 0;
    let mut phrase = "Nothing";
    if 1+1 == 2
    {
        result = 1;
        phrase = "Something"
    }
    else
    {
        result = 0;
        phrase = "This is wrong";
    }
    println!("This is the phrase: {}, and the number {}", phrase, result);

    /*
    Otro ejemplo de match.
    Match es similar a switch en C. 
    */

    let status = 400;
    match status    //Similar a switch(status)
    {
        /*
        Todos los casos van dentro de corchetes.
        200 => es lo mismo que case 200: en C. 

        => representa los posibles valores que la variable
        que estamos evaluando puede tomar

        Si después de => hay una sola instrucción, se delimita con una coma
        Si se requieren de más líneas, se encierran con {}
        */
        200 => println!("Success"), //Si el valor de status es 200, se imprimer "Success"
        404 => println("Not Found"),
        /*other equivale a default en C. Se ejecutan estas líneas si status no es 200 o 404. */
        other => {  
            println!("Request failed");
            let suma = 1 + 5;
            println("A sum to demonstrate multiple instructions in match case: {}", suma);
        }
    }

    /*
    Estructuras iterativas: loop

    loop equivale a while(true) en C. Representa un bucle infinito.
    Se puede utilizar break para salir del bucle.
    */
    let mut x = 5;
    loop {
    if x < 0
    {
        break;
    }
    println!("{} more runs to go", x);
    x -= 1;
    }

    /*
    Es posible etiquetar loops para que, por ejemplo, podamso salir de 
    un loop específico en casos en los que tengamos loops anidados. 
    */
    let a = 10;
    let b = 4;
    let result = silly_sub(a, b);
    println!("{} minus {} is {}", a, b, result);

    /*While: igual que en c. */

    let mut x = 1000;
    while x > 0
    {
        println!("{} more runs to go", x);
        x -= 1;
    }

    /*
    For en Rust es similar a for en Python.
    Solo funcionan para tipos que puedan convertirse
    en iteradores, por ejemplo, Range
    */

    // Este rango no incluye al 10
    print!("Normal ranges: ");
    for i in 0..10
    {
        print!("{},", i);
    }

    println!();
    // just a newline

    print!("Inclusive ranges: ");
    // Incluye al 10
    for i in 0..=10
    {
        print!("{},", i);
    }

}


}