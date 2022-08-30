fn fix_order (){
    cook_order();
    super::deliver_order(); //ruta relativa
    crate::deliver_order(); //equivalente en ruta absoluta
}

fn cook_order (){}

//cuando hacemos un struct pub, tenemos que hacer los campos publicos segun sea necesario
//en el siguiente caso donde le cliente puede elegir el toast, pero no la fruta (que es de temporada y fijada por el chef)

pub struct Breakfast {
    pub toast: String,
    fruit: String,
}

impl Breakfast {
    //gracias a esta funcion podemos obtener una instancia de Breakfast
    //ya que como no todos sus campos son publicos, instanciar desde el exterior directamente no esta permitido
    pub fn summer(toast_kind: &str) -> Self {
        Breakfast {
            toast: String::from(toast_kind),
            fruit: String::from("peaches"),
        }
    }
}

//contrario a los structs, cuando los enum se hacen pub, todas sus variantes se vuelven pub

pub enum Apetizer {
    Soup,
    Salad,
}