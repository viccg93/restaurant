//El codigo del crate root sea de lib o binary crea un modulo llamado crate 
//que se localiza en la raiz de la estructura de modulos del crate conocida como arbol de modulos
mod front_of_house;

mod serving {
    fn take_order(){}
    fn serve_order(){}
    fn take_payment(){}
}

//podemos importar el modulo hosting para no tener que usar toda la ruta cada vez que necesitemos un item de ese modulo
//use front_of_house::hosting; //de esta manera el import es privado al modulo actual
pub use front_of_house::hosting; // el import puede ser usado externamente, lo que se conoce como re-exporting

mod customer {
    pub fn eat_at_resto(){
        //estas rutas no compilan si el modulo hosting es privado
        //ya que todos los miembros sean modulos, funciones, metodos, structs enums y constantes
        //son privados por default, para volverlos publicos se usa pub
    
        //los parent no pueden ver los miembros de los children, a menos que sean pub
        //los children si pueden ver los item de los parent, ya que es su contexto
    
        //ruta aboluta (desde la raiz, que esta definida implicitamente como crate)
        crate::front_of_house::hosting::add_to_waitlist();
    
        //ruta relativa (desde el modulo actual)
        //front_of_house::hosting::add_to_waitlist(); // no compila al moverse de crate a customer
    
        //cuando importamos hosting con use
        //hosting::add_to_waitlist(); //no compila por el movimiento a customer
        //al usar super, estamos usando hosting que fue importado en create, que es el modulo parent de customer
        super::hosting::add_to_waitlist();
    }
}

//mediante super podemos acceder a un item del parent

//funcion perteneciente al modulo crate
fn deliver_order(){}

mod back_of_house;

pub fn breakfast_at_resto(){
    //podemos llamar al metodo summer del struct Breakfast que es parte del modulo back:back_of_house
    //la instancia la podemos hacer mutable, para tener acceso al campo toast
    let mut meal = back_of_house::Breakfast::summer("linaza");
    //cambiar el valor del campo toast esta permitido
    meal.toast = String::from("integral");

    println!("me gustaria ordenar un toast de {}", meal.toast);
    //meal.fruit = String::from("strawberries"); //no compila por que el campo es privado, no se puede acceder a su valor o modificarlo
}

pub fn dinner_at_resto(){
    let order1 = back_of_house::Apetizer::Salad;
    let order2 = back_of_house::Apetizer::Soup;
}

//algo que debemos tener en cuenta es que al importar la funcion mntuvimos su modulo, en lugar de importar solo la funcion
//esto es util por que resulta mas claro saber que esa funcion es de otro modulo
//cuanto importamos un item como un struct, enum, etc, importamos solo el item
//la excepcion a esta recomendacion es cuando existen items con el mismo nombre pero distinto modulo, asi podemos saber a que
//item nos estamos refiriendo, evitando ambiguedades y por ende errores  ya que existirian 2 items con el mismo nombre
//en el mismo scope, una alternativa es el uso de as, para definir alis de esos modulos


mod scope_de_prueba {
    use std::fmt::Result;
    use std::io::Result as IoResult;
}

//nested paths

//podemos usar nested paths para evitar el uso multiple de use sobre items que pertenecen al mismo crate
//podemos convertir los siguientes use

//use std::cmp::Ordering;
//use std::io;

//en 

use std::{cmp::Ordering,io::Read};

//se pueden usar las nested paths a cualquier nivel como cuando se quiere importar del mismo sub-modulo

use std::io::{self,Write};

//cuando se desea traer al scope todos los miembros de un modulo, se puede usar el operador glob (*)
//este operador se debe de usar con cuidado, ya que traer todos los elementos de un modulo
//puede ocasionar coliciones, generalmente se usa en pruebas o como parte del preludio

use std::collections::*;