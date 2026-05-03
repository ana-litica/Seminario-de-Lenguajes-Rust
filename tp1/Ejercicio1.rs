use std::io::stdin;

fn main(){
    let numero= 1.5;
    println!("Ingrese un número:");
    let mut ingresado= String::new();
    stdin().read_line(&mut ingresado).expect("Error al leer el número");
    let nuevo_valor: f64 = ingresado.trim().parse().unwrap();

    println!("1.5 + {nuevo_valor} = {}",numero+nuevo_valor);
    println!("1.5 - {nuevo_valor} = {}",numero-nuevo_valor);
    println!("1.5 * {nuevo_valor} = {}",numero*nuevo_valor);
    println!("1.5 / {nuevo_valor} = {}",numero/nuevo_valor);
}