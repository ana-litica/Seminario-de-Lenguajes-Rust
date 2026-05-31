#![allow(unused)]
pub fn serie_geometrica<const TAMAÑO:usize>() ->[usize;TAMAÑO]{
    let mut arreglo=[1;TAMAÑO];
    for i in 1..TAMAÑO{
        arreglo[i]=arreglo[i-1]*2;
    }
    arreglo
}

#[cfg(test)]
mod test_serie_geometrica{
    use super::*;

    #[test]
    fn test_serie_geometrica(){
        assert_eq!(serie_geometrica(),[1,2,4,8]);
    }

    #[test]
    fn test_serie_geometrica_vacia(){
        assert_eq!(serie_geometrica(),[]);
    }
}