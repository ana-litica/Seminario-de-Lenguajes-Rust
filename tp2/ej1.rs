#![allow(unused)]
pub fn es_par(numero: i32) -> bool{
    numero % 2 == 0 
}

#[cfg(test)]
mod test_es_par{
    use super::*;

    #[test]
    fn test_es_par_true(){
        let par:bool= es_par(4);
        assert_eq!(par,true);
    }

    #[test]
    fn test_es_par_false(){
        let impar: bool = es_par(3);
        assert_eq!(impar, false);
    }
}
