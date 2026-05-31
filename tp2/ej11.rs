#![allow(unused)]
pub fn multiplicar_valores(arreglo: &mut [i32],factor:i32){
    for i in arreglo{
        *i*=factor;
    }
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn multiplicar_valores_modificados_test(){
        let mut arreglo=[1,2,3,4,5];
        multiplicar_valores(&mut arreglo,2);
        assert_eq!(arreglo,[2,4,6,8,10]);
    }

    #[test]
    fn mutiplicar_valores_no_modificados_test(){
        let mut arreglo=[1,2,3,4,5];
        multiplicar_valores(&mut arreglo,1);
        assert_eq!(arreglo,[1,2,3,4,5]);
    }

    #[test]
    fn multiplicar_valores_arreglo_vacio_test(){
        let mut arreglo=[];
        multiplicar_valores(&mut arreglo,4);
        assert_eq!(arreglo,[]);
    }
}