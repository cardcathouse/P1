# Lista doblemente ligada en Rust

Este documento explicará las políticas de propiedad y mutabilidad utilizadas en la implementación de una lista doblemente ligada en Rust, como se vé en el archivo [doubleLinkedListRustExample.rs](doubleLinkedListRustExample.rs) del repositorio.

Primero, explicaremos algunos conceptos básicos que ayudarán a entender mejor el resto del documento.
### Propiedad
Uno de los conceptos más importantes en Rust es el de **propiedad**. Este concepto es integral para el manejo de memoria en Rust, ya sea en el _stack_ o en el _heap_.

Básicamente, indica que cada valor debe tener una una sola variable a la vez como dueña, y que, cuando la variable dueña ya no esté en su ámbito (por ejemplo, que la función en la que se utiliza ya se haya ejecutado), el valor se quita y la memoria que usó se libera.

### Mutabilidad
Por otro lado, el concepto de **mutabilidad** nos indica cómo se comporta una variable. Si una variable es **inmutable,** significa que el valor que tiene asignado ya no puede cambiar. Si la variable es **mutable**, el valor puede cambiar durante la ejecución del programa. 

Por defecto, las variables son inmutables. Aunque hay muchas razones para explicar el por qué de esto, una de las principales es que Rust tiene como prioridad la seguridad a la hora de compilar el programa. El compilador puede detectar intentos de asignación de valor en variables inmutables y alertar al usuario de ello.

Consideremos el siguiente escenario: tenemos una variable inmutable que contiene un vector. Los datos del vector se almacenan en el _heap_, mientras que los datos para el objeto de nuestro vector, incluyendo una referencia a la ubicación en el *heap* del contenido del vector, se guardan en el stack. Después, supongamos que queremos que una variable mutable contenga el vector que guardamos en `v1` y usamos la instrucción `let mut v2 = v` para ello. 

Esto traería problemas, ya que lo que esa instrucción realmente está haciendo es copiar sólo los contenidos del objeto del vector almacenados en el *stack* al espacio del *stack* asignado para `v2`, lo que querría decir que ambas variables tendrían la misma referencia a la región del *heap* en donde están los datos del vector. Esto implicaría un cambio de la variable asignada al vector, por lo que no podríamos usar `v` para acceder a él, además de que introduce una condición de carrera sobre los datos del vector, lo cual viola las garantías de seguridad de Rust.

### Préstamo
Podemos evitar este problema utilizando el concepto de préstamo. Utilizando referencias, podemos “pedir prestada” la propiedad de los datos una variable. Cuando una variable que tiene un valor que pidió prestado sale de su ámbito, el valor no pierde la memoria que tiene asignada y regresa a su variable propietaria original.

Podemos tener dos tipos de referencias: **mutables** e **inmutables**. La referencia **inmutable** `(&ref)` no puede cambiar el valor que pidió prestado, incluso dentro de su ámbito. La referencia **mutable** `(&mut ref)` permite cambiar el valor que se pidió prestado. 

Existen algunas reglas para el uso correcto del préstamo:
- Cualquier préstamo no debe tener un ámbito mayor que el de su variable dueña
- Sólo se puede tener un tipo de préstamo a la vez: o una o más referencias inmutables o sólo una referencia mutable.

La primera sirve para asegurarnos que la administración de propiedad de los datos se maneje correctamente, es decir, que se regresa la propiedad a su dueña cuando quién la pidió prestada la termine de utilizar. La segunda regla nos ayuda a evitar condiciones de carrera sobre los datos.

Con estos conceptos revisados, podemos pasar a hablar sobre las políticas específicas dentro del código de la lista ligada.

### Políticas de la lista ligada

Recordemos que en una lista doblemente ligada, un nodo tiene referencias al nodo que le antecede y al que le sucede. Además, la lista debe tener una cabeza y una cola que indiquen en dónde inicia y en dónde acaba.  Sin embargo, conseguir esto en Rust es algo más complicado que en otros lenguajes de programación por los conceptos que ya explicamos anteriormente. 

En otros lenguajes, bastaría con obtener las referencias a los objetos de los nodos anteriores y siguientes y guardarlas en el objeto del nodo con el que estamos trabajando. Para hacer operaciones sobre la lista, trabajamos sobre esas referencias para reflejar los cambios que hacemos en ella. Mientras sepamos manejar referencias en el lenguaje de nuestra elección, sólo tenemos que preocuparnos por asegurarnos que esas referencias se actualicen de manera correcta. 

En Rust, además de lo que ya descrbimos, tenemos que tener en cuenta la propiedad de nuestros nodos. Considerando la estructura de la lista, esta puede dar lugar a que se produzcan **ciclos de referencias**. Esto se produce cuando dos o más objetos tienen referencias fuertes entre sí. Una **referencia fuerte** es aquella que apunta a un objeto que tiene dueño. 

Si tenemos un ciclo de referencias, corremos el riesgo de una fuga de memoria, en donde la memoria ocupada por esos objetos no se libera, incluso hasta cuando haya terminado de correr el programa. En programas en los que se utiliza mucha memoria y se presenta esta situación, el sistema incluso puede caerse.

Recordando las características del sistema de propiedad, podemos ver que esto es en definitiva un problema. Afortunadamente podemos arreglarlo utilizando **apuntadores inteligentes** y **referencias débiles**. Así, podemos generar la estructura de la lista a la vez que nos aseguramos que estamos utilizando la memoria conforme los estándares de Rust.

#### Apuntadores inteligentes: `Rc<T>`, `RefCell<T>` y `Weak<T>`

Para implementar la lista y sus operaciones, necesitamos que un nodo tenga referencias a los nodos a sus lados, y que todos ellos puedan modificar las referencias que usan para saber quiénes están a su alrededor en todo momento. Cuando eliminamos un nodo, debemos de actualizar las referencias relacionadas para mostrar ese cambio. 

En términos del sistema de propiedad, lo podemos ver de la siguiente manera: cada nodo tiene un dueño, pero además, los nodos que están a su alrededor también necesitan ser dueños de él para saber que esta ahí y poder acceder a su dato. Podríamos conseguir esto por medio de referencias, pero las referencias normales de Rust tienen limitantes que no permitirían esto. Para sobrepasar esto, haremos uso de los apuntadores inteligentes `Rc<T>` y `RefCell<T>`

`Rc<T>` o Apuntador de referencias contadas es una estructura que nos permite que un valor tenga múltiples dueños. Esto es útil cuando queremos compartir un valor entre múltiples variables, pero no sabemos cuál de todas será la última en hacerlo. Utilizándolo en conjunto con la función `clone`, creamos un nuevo apuntador a la misma región en el *heap* en donde se encuentra nuestra dato a compartir.

Cada vez que creemos un nuevo apuntador, incrementamos un contador interno que indica cuántos apuntadores en total están compartiendo propiedad del dato. Cuando todos los apuntadores terminan de usar el dato (salen de su ámbito y son destruidos), el valor se quita y se libera la memoria. Esta función nos es útil para la lista porque nos permite que los nodos puedan tener referencias de aquellos que están a su alrededor. 

Si usaramos, por ejemplo, `Box<T>` para crear los nodos en la lista, tendríamos el problema de que la propiedad de los nodos se movería entre ellos cada vez que agregaramos uno nuevo y actualizaramos las referencias.

En cambio, al hacer al nodo del tipo `Rc<T>` y usar `clone` en los nodos que necesitan tener referencia de él, podemos cumplir con las reglas de propiedad y con las características de una lista ligada.

Ya pudimos crear la estructura básica de la lista ligada. Sin embargo, llegará el punto en el que tengamos que añadir o eliminar nodos y, por lo tanto, cambiar referencias. `Rc<T>` se apega a las reglas de mutabilidad de Rust, por lo que no podemos modificar los elementos de la lista por defecto. Para solucionar eso, usaremos la estructura `RefCell<T>`.

`RefCell<T>` es una estructura que trabaja con el patrón de diseño de mutabilidad interior, el cuál permite mutar valores aunque este tenga referencias inmutables. El método `borrow_mut()` nos permitirá pedir prestado los valores por medio de referencias mutables. Este método también lleva un contador de las referencias que se le hacen a un valor. Adicionalmente, contiene métodos que dereferencian automáticamente para evtar conflictos de propiedad. 

Nuestros nodos usaran ambas estructuras para tener nodos mutables con múltiples dueños. De esta forma, podremos realizar todas las operaciones de la lista que requieran manipular los nodos y sus referencias.

Aunque con estos elementos ya podríamos comenzar a implementar la lista, nos falta resolver otro problema: el del ciclo de referencias. Recordemos que si tenemos un ciclo de referencias, corremos el riesgo de no poder liberar la memoria que ocupan. 

Esto sería un problema cuando quisieramos eliminar un nodo en la lista, ya que si nuestros nodos son del tipo `Rc<T>`, no podremos eliminarlos hasta que todas las referencias que sean dueñas de él hayan terminado de usarlo. Rust no se encarga de prevenir esto, por lo que debemos de ocuparnos nosotros de resolverlo usando **referencias débiles**. 

Una referencia débil es aquella que no toma propiedad del objeto al que apunta. En Rust, esto está implementado en el apuntador inteligente `Weak<T>`. Podemos convertir una referencia `Rc<T>`, que es fuerte por defecto para tener posesión sobre un dato, a una débil utilizando la instrucción `Rc::downgrade`. Así, podemos eliminar nodos sin importar las condiciones en las que se encuentre.