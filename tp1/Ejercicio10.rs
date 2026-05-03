fn main(){
    let arreglo1=[1,2,3,4,5];
    let arreglo2=[6,7,8,9,10];
    let mut resultado=[0; 5];

    for indice in 0..5{
        resultado[indice] = arreglo1[indice] + arreglo2[indice];
    }

    println!("Arreglo 1: {:?}",arreglo1);
    println!("Arreglo 2: {:?}",arreglo2);
    println!("Arreglo suma: {:?}",resultado);
}