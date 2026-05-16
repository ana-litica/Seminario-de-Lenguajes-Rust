#![allow(unused)]
pub fn reemplazar_pares(arreglo:&mut [i32]){
    for i in arreglo{
        if *i % 2 == 0{
            *i=-1;
        }
    }
}

#[cfg(test)]
mod test_reemplazar_pares{
    use super::reemplazar_pares;

    #[test]
    fn test_reemplazar_algunos_pares(){
        let mut pares=[2,4,9,7,10];
        reemplazar_pares(&mut pares);
        assert_eq!(pares,[-1,-1,9,7,-1]);
    }

    #[test]
    fn test_reemplazar_pares_inexistentes(){
        let mut impares=[5,3,9,7,1];
        reemplazar_pares(&mut impares);
        assert_eq!(impares,[5,3,9,7,1]);
    }

    #[test]
    fn test_reemplazar_todos_pares(){
        let mut pares=[2,4,6,8];
        reemplazar_pares(&mut pares);
        assert_eq!(pares,[-1,-1,-1,-1]);
    }

    #[test]
    fn test_reemplazar_todos_arreglo_vacio(){
        let mut vacio=[];
        reemplazar_pares(&mut vacio);
        assert_eq!(vacio,[]);
    }
}