#![allow(unused)]
pub fn ordenar_nombres<const DIMENSION:usize>(nombres: &mut[&str;DIMENSION]){
    for i in 0..nombres.len(){
        let mut j=i;
        while 0 < j && nombres[j] < nombres[j-1]{
            let auxiliar=nombres[j-1];
            nombres[j-1]=nombres[j];
            nombres[j]=auxiliar;
            j-=1;
        }
    }
}


#[cfg(test)]
mod test{
    use super::ordenar_nombres;

    #[test]
    fn ordenar_nombres_random_test(){
        let mut nombres=["Graciela","Roberto","Julian","Ana"];
        ordenar_nombres(&mut nombres);
        assert_eq!(nombres,["Ana","Graciela","Julian","Roberto"]);
    }

   #[test]
    fn ordenar_nombres_vacio_test(){
        let mut nombres:[&str;0]=[];
        let arregloresultado:[&str;0]=[];
        ordenar_nombres(&mut nombres);
        assert_eq!(nombres,arregloresultado);
    }
}