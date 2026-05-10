#![allow(unused)]
pub fn suma_pares(arreglo: &[i32]) -> i32{
    let mut suma:i32=0;
    for i in arreglo {
        if i % 2 ==0  { 
            suma+=i;
        }
    }
    suma
}

#[cfg(test)]
mod test_suma_pares{
    use super::*;
    //use super::suma_pares;

    #[test]
    fn test_suma_pares(){
        let suma: i32 = suma_pares(&[1,2,3,4,5]);
        assert_eq!(suma, 6);

    }

    #[test]
    fn test_suma_sin_pares(){
       /* let suma = suma_pares(&[3,5,7,9,11]);
        assert_eq!(suma,0);*/
        assert_eq!(suma_pares(&[3,5,7,9,11]),0);
    }

    #[test]
    fn test_suma_pares_todos(){
        assert_eq!(suma_pares(&[2,4,6]),12);
    }

    #[test]
    fn test_suma_pares_vacio(){
    assert_eq!(suma_pares(&[]),0);
    }
}