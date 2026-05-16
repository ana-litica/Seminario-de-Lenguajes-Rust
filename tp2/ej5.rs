#![allow(unused)]
pub fn duplicar_valores<const DIMENSION:usize>(arreglo:&[f32;DIMENSION]) -> [f32;DIMENSION]{
    let mut resultado=[0.0;DIMENSION];
    for i in 0..arreglo.len(){
        resultado[i]=arreglo[i]*2.0;
    }
    resultado
}


#[cfg(test)]
mod test_duplicar_valores{
    use super::*;

    #[test]
    fn test_duplicar_valores(){
        // let mut duplicado: &[f32]= duplicar_valores(&[1.0,2.0,3.0,4.0,5.0]);
        // assert_eq!(duplicado,[2.0,4.0,6.0,8.0,10.0]);
        assert_eq!(duplicar_valores(&[1.0,2.0,3.0,4.0,5.0]),[2.0,4.0,6.0,8.0,10.0]);
    }

    #[test]
    fn test_duplicar_valores_arreglo_vacio(){
        assert_eq!(duplicar_valores(&[]),[]);
    }
}