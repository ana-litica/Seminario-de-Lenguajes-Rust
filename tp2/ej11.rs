#![allow(unused)]
pub fn multiplicar_valores(arreglo: &mut [i32],factor:i32){
    for i in arreglo{
        *i*=factor;
    }
}

#[cfg(test)]
mod test_multiplicar_valores{
    use super::*;

    #[test]
    fn test_multiplicar_valores_modificados(){
        let mut arreglo=[1,2,3,4,5];
        multiplicar_valores(&mut arreglo,2);
        assert_eq!(arreglo,[2,4,6,8,10]);
    }

    #[test]
    fn test_mutiplicar_valores_no_modificados(){
        let mut arreglo=[1,2,3,4,5];
        multiplicar_valores(&mut arreglo,1);
        assert_eq!(arreglo,[1,2,3,4,5]);
    }

    #[test]
    fn test_multiplicar_valores_arreglo_vacio(){
        let mut arreglo=[];
        multiplicar_valores(&mut arreglo,4);
        assert_eq!(arreglo,[]);
    }
}