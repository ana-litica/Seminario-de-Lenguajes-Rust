use std::io::stdin;

fn main(){
    let mut booleano=true;
    println!("Ingrese el valor booleano:");
    let mut ingresado= String::new();
    stdin().read_line(&mut ingresado).expect("Error al leer el booleano");
    let nuevo_booleano= ingresado.trim()=="true";

    booleano=booleano && nuevo_booleano;
    println!("Resultado de aplicar AND {}",booleano);
    booleano=booleano || nuevo_booleano;
    println!("Resultado de aplicar OR {}",booleano);
}