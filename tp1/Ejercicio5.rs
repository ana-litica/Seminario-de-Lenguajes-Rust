use std::io::stdin;

fn main(){
    let mut cadena:String = "Hola".to_string();
    println!("{cadena}");
    println!("Ingrese una nueva cadena:");
    let mut ingresada= String::new();
    stdin().read_line(&mut ingresada).expect("Error al leer la cadena");
    cadena+=ingresada.trim();
    println!("{cadena}");  
}