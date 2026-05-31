#![allow(unused)]
pub fn es_primo(valor: u32) -> bool {
    for i in 2..valor {
        if valor % i == 0 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn es_primo_test(){
        let primo: bool = es_primo(5);
        assert_eq!(primo,true);
    }    

    #[test]
    fn no_es_primo_test(){
        let no_primo: bool = es_primo(25);
        assert_eq!(no_primo,false);
    }
}