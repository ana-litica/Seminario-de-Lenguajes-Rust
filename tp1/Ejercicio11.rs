use std::io::stdin;

fn main(){
    let arreglo=["Hola","Buen día","Buenas Tardes","Buenas Noches","Chau"];

    let mut cadena = String::new();
    println!("Ingrese una cadena");
    stdin().read_line(&mut cadena).expect("Error al leer la cadena");

    //let mut indice=0;
    let mut igual = false;
    cadena = cadena.trim().to_string();

   for elemento in arreglo{
        if elemento.to_string() == cadena{
            igual=true;
            break;
        }
    }

    /*while indice<5 && !igual{     
        if arreglo[indice]/*.trim()*/ == cadena.trim() {
            igual=true;
        }
       // println!("{}",igual.to_string());
        indice+=1;
    }*/

    if igual{
        println!("La cadena {cadena} se encuentra en el arreglo");
    }
    else{
        println!("La cadena {cadena} no se encuentra en el arreglo");
    }


}