use std::io::stdin;

fn main(){
    let mut numero:u32= 4;
    println!("Ingrese un valor: ");
    let mut valor_ingresado=String::new();
    stdin().read_line(&mut valor_ingresado).expect("Error al leer el valor");
    let nuevo_numero:u32=valor_ingresado.trim().parse().unwrap();

    numero+=nuevo_numero;
    println!("{numero}^2 = {}",numero*numero);
}