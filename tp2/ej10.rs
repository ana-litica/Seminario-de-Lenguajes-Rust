#![allow(unused)]
pub fn cantidad_de_cadenas_mayor_a(arreglo:&[&str],limite:i32) -> i32{
    let mut cantidad=0;

    for i in arreglo{
        if i.len() as i32 > limite{
            cantidad+=1;
        }
    }

    cantidad
}

#[cfg(test)]
mod test_cantidad_de_cadenas_mayor_a{
    use super::*;

    #[test]
    fn test_cantidad_de_cadenas_mayor_a_limite(){
        assert_eq!(cantidad_de_cadenas_mayor_a(&["hola","chau","buen día","buenas noches"],3),4);
    }

    #[test]
    fn test_cantidad_de_cadenas_mayor_a_ninguno(){
        assert_eq!(cantidad_de_cadenas_mayor_a(&["hola","chau","buen día","buenas noches"],30),0);
    }

    #[test]
    fn test_cantidad_de_cadenas_mayor_a_algunos(){
        assert_eq![cantidad_de_cadenas_mayor_a(&["Hola","Si","Chau","No"],3),2];
    }

    #[test]
    fn cantidad_de_cadenas_mayor_arreglo_vacio(){
        assert_eq!(cantidad_de_cadenas_mayor_a(&[],3),0);
    }
}