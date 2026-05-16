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
mod test_es_primo{
    use super::*;

    #[test]
    fn test_es_primo(){
        let primo: bool = es_primo(5);
        assert_eq!(primo,true);
    }    

    #[test]
    fn test_no_es_primo(){
        let no_primo: bool = es_primo(25);
        assert_eq!(no_primo,false);
    }
}