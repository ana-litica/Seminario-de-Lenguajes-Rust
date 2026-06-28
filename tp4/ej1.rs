#![allow(unused)]
pub trait NumeroPrimo{
    fn es_primo(&self)->bool;
}

impl NumeroPrimo for i32{
    fn es_primo(&self)->bool{    
        if *self<2 {
            return false;
        }
        for i in 2..*self {
            if *self % i == 0 {
                return false;
            }
        }
        true
    }
}

fn contar_primos(vector:Vec<i32>)->usize{
    vector.iter().filter(|numero| numero.es_primo()).count()
}

#[cfg(test)]
mod test{
    use crate::tp4::ej1::contar_primos;

    #[test]
    fn contar_primos_test(){
        let vector=vec![2, 4, 5, 8, 11, 15];
        assert_eq!(contar_primos(vector),3);
    }

    #[test]
    fn contar_primos_todos_cumplen_test(){
        let vector=vec![2, 3, 5, 7, 11, 13];
        assert_eq!(contar_primos(vector),6);
    }

    #[test]
    fn contar_primos_niguno_cumple_test(){
        let vector=vec![1, 4, 6, 8, 9, 10, 12];
        assert_eq!(contar_primos(vector),0);
    }

    #[test]
    fn contar_primos_vector_vacio_test(){
        let vector=vec![];
        assert_eq!(contar_primos(vector),0);
    }

}
