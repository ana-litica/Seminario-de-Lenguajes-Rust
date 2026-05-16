#![allow(unused)]
pub fn cantidad_en_rango(arreglo:&[i32],inferior:i32, superior:i32) -> i32{
    let mut cantidad:i32=0;

    for i in arreglo{
        if inferior<=*i && *i<=superior{
            cantidad+=1;
        }
    }

    cantidad
}

#[cfg(test)]
mod test_cantidad_en_rango{
    use super::*;

    #[test]
    fn test_cantidad_en_rango_3(){
        assert_eq!(cantidad_en_rango(&[2,11,3,8,10],3,10),3);
    }

    #[test]
    fn test_cantidad_en_rango_inferior(){
        assert_eq!(cantidad_en_rango(&[1,2,3,8,9],10,20),0);
    }

    #[test]
    fn test_cantidad_en_rango_superior(){
        assert_eq!(cantidad_en_rango(&[6,7,8,9,11],2,5),0);
    }

}