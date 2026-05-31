#![allow(unused)]
pub fn es_par(numero: i32) -> bool{
    numero % 2 == 0 
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn es_par_numero_par_test(){
        let par:bool= es_par(4);
        assert_eq!(par,true);
    }

    #[test]
    fn es_par_numero_impar_test(){
        let impar: bool = es_par(3);
        assert_eq!(impar, false);
    }
}
