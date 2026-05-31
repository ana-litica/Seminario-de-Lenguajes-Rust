#[derive(Debug)]
struct Rectangulo{
    longitud: f32,
    ancho:f32,
}

#[allow(unused)]
impl Rectangulo{

    pub fn new(longitud:f32, ancho:f32) -> Rectangulo{
        Rectangulo{
            longitud,
            ancho
        }
    }

    pub fn calcular_area(&self) -> f32{
        self.longitud*self.ancho
    }

    pub fn calcular_perimetro(&self) ->f32{
        2.0*self.longitud+2.0*self.ancho
    }

    pub fn es_cuadrado(&self)->bool{
        let mut cuadrado:bool=false;
        if(self.longitud==self.ancho){
            cuadrado=true;
        }
        cuadrado
    }

}

#[cfg(test)]
mod test{
    use crate::tp3::ej2::Rectangulo;

    #[test]
    fn calcular_area_test(){
        let rectangulo=Rectangulo::new(5.0,2.0);
        assert_eq!(rectangulo.calcular_area(),10.0);
    }

    #[test]
    fn calcular_perimetro_test(){
        let rectangulo=Rectangulo::new(5.0,2.0);
        assert_eq!(rectangulo.calcular_perimetro(),14.0);
    }

    #[test]
    fn es_cuadrado_ancho_alto_iguales_test(){
        let rectangulo=Rectangulo::new(5.0,2.0);
        assert_eq!(rectangulo.es_cuadrado(),false);
    }
    
    #[test]
    fn es_cuadrado_ancho_alto_diferentes_test(){
        let rectangulo=Rectangulo::new(5.0,5.0);
        assert_eq!(rectangulo.es_cuadrado(),true);
    }

}
