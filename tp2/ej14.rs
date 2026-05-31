#![allow(unused)]
pub fn incrementar(numero: &mut f32){
    *numero+=1.0;
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn incrementar_valor_test(){
        let mut numero=1.5;
        incrementar(&mut numero);
        assert_eq!(numero,2.5);
    }
}