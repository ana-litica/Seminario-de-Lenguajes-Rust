#![allow(unused)]
pub fn longitud_de_cadenas<const DIMENSION:usize>(cadenas:&[String]) ->[usize;DIMENSION]{
    let mut arreglo:[usize;DIMENSION]=[0;DIMENSION];
    let limite = cadenas.len();

    for i in 0..limite {
        arreglo[i]=cadenas[i].len();
    }

    arreglo
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn longitud_de_cadenas_test(){
        let mut resultado=longitud_de_cadenas(&["hola".to_string(),"chau".to_string(),"tp2".to_string(),"ej6".to_string()]);

        assert_eq!(resultado,[4,4,3,3]);
    }

    #[test]
    fn longitud_de_cadenas_vacias_test(){
        let mut resultado:  [usize; 0]=longitud_de_cadenas(&[]);
        let arreglo: [usize;0]=[];
        assert_eq!(resultado,arreglo);
    }
}
