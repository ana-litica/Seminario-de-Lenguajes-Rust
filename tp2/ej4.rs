#![allow(unused)]
pub fn cantidad_impares(arreglo:&[i32]) -> i32{
    let mut cantidad_impares=0;
    for i in arreglo{
        if i % 2 != 0{
            cantidad_impares+=1;
        }
    }
    cantidad_impares
}


#[cfg(test)]
mod test_cantidad_impares{
    use super::cantidad_impares;

    #[test]
    fn test_cantidad_impares(){
        let resultado:i32 =cantidad_impares(&[1, 2, 3, 4, 5]);
        assert_eq!(resultado, 3);
    }

    #[test]
    fn test_cantidad_sin_impares(){
        let resultado:i32 =cantidad_impares(&[2,4,6,8,10]);
        assert_eq!(resultado, 0);
    }

    #[test]
    fn test_cantidad_todos_impares(){
        let resultado:i32=cantidad_impares(&[1,3,5,7]);      
        assert_eq!(resultado,4);
    }

    #[test]
    fn test_cantidad_impares_arreglo_vacio(){
        let resultado:i32=cantidad_impares(&[]);
        assert_eq!(resultado,0);
    }
}