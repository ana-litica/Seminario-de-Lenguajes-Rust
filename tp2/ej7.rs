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
mod test_cantidad_de_mayores{
    use super::cantidad_de_mayores;

    #[test]
    fn test_cantidad_de_mayores_varios(){
        assert_eq!(cantidad_de_mayores(&[1,2,7,8,9,11],5),4);
    }

    #[test]
    fn test_cantidad_de_mayores_ninguno(){
        assert_eq!(cantidad_de_mayores(&[1,2,3,4,5,6],11),0);
    }

    #[test]
    fn test_cantidad_mayores_todos(){
        assert_eq!(cantidad_de_mayores(&[6,7,8,9,10,11],5),6);
    }
}