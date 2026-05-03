fn main(){
    let arreglo=[2, 5, 1, 8, 4];
    let mut suma=0;

    let sup=5;
    for indice in 1..sup{
        suma+=arreglo[indice];
    }

    println!("La suma de los valores del arreglo es {}",suma);
}