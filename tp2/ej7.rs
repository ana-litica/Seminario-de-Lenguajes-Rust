#![allow(unused)]
pub fn cantidad_de_mayores(arreglo:&[i32], limite:i32) -> i32{
    let mut mayores=0;
    for i in arreglo{
        if limite < *i{
            mayores+=1;
        } 
    }
    mayores
}

#[cfg(test)]
mod test{
    use super::cantidad_de_mayores;

    #[test]
    fn cantidad_de_mayores_varios_test(){
        assert_eq!(cantidad_de_mayores(&[1,2,7,8,9,11],5),4);
    }

    #[test]
    fn cantidad_de_mayores_ninguno_test(){
        assert_eq!(cantidad_de_mayores(&[1,2,3,4,5,6],11),0);
    }

    #[test]
    fn cantidad_de_mayores_todos_test(){
        assert_eq!(cantidad_de_mayores(&[6,7,8,9,10,11],5),6);
    }
}