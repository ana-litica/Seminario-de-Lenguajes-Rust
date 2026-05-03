use std::io::stdin;

fn main(){
    const CADENA:&str="Soy una cadena";

    println!("Ingrese un caracter:");
    let mut caracter= String::new();
    stdin().read_line(&mut caracter).expect("Error al leer el caracter");

    let mut ocurrencias=0;

    for car in CADENA.chars(){
        if car.to_string() == caracter.trim() {
            ocurrencias+=1;
        }
    }

    println!("La cantidad de veces que el caracter {caracter} aparece en {CADENA} es {ocurrencias}");
}