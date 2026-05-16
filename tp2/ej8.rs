#![allow(unused)]
pub fn sumar_arreglos<const DIMENSION:usize>(arreglo1:&[f32;DIMENSION],arreglo2:&[f32;DIMENSION]) -> [f32;DIMENSION]{
    let mut resultado:[f32;DIMENSION]=[0.0;DIMENSION];

    for i in 0..arreglo1.len(){
        resultado[i]=arreglo1[i]+arreglo2[i];
    }

    resultado
}

#[cfg(test)]
mod test_sumar_arreglos{
    use super::*;

    #[test]
    
    fn test_sumar_arreglos(){
        assert_eq!(sumar_arreglos(&[1.0,3.0,5.0,7.0],&[7.0,5.0,3.0,1.0]),[8.0,8.0,8.0,8.0]);
    }

    #[test]
    fn test_sumar_arreglos_vacios(){
        assert_eq!(sumar_arreglos(&[],&[]),[]);
    }
}