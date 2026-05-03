fn main(){
    let tupla=("Hola",[4,5,6]);

    println!("{}",tupla.0);

    let mut suma=0;

    for elemento in tupla.1{
        suma+=elemento;
    }

    println!("Suma: {suma}");
}