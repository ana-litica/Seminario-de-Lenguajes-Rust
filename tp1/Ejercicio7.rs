fn main(){
    let mut arreglo= [1, 2, 3, 4, 5, 6];
    const CONSTANTE:u32= 10;
    println!("El arreglo es: {:?}", arreglo);
    
    let sup=6;
    for indice in 1..sup{
        arreglo[indice]*=CONSTANTE;
    }
    /*arreglo[0]*= constante;
    arreglo[1]*= constante;
    arreglo[2]*= constante ;
    arreglo[3]*= constante;
    arreglo[4]*= constante; 
    arreglo[5]*= constante;*/

    println!("El arreglo es: {:?}", arreglo);

}
