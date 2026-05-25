#[derive(Debug)]
enum TipoTriangulo{
    Equilatero, 
    Isosceles,
    Escaleno,
}

#[derive(Debug)]
struct Triangulo{
    lado_1:f32,
    lado_2:f32,
    lado_3:f32,
}

#[allow(unused)]
impl Triangulo{

    pub fn new(lado_1:f32,lado_2:f32,lado_3:f32) ->Triangulo{
        if (lado_1>0.0 && lado_2>0.0 && lado_3>0.0) && 
        ((lado_1+lado_2>lado_3) && (lado_1+lado_3>lado_2) && (lado_3+lado_2>lado_1)){
            Triangulo{
                lado_1,
                lado_2,
                lado_3
            }
       }else{
            panic!("Los valores no corresponden a un triángulo válido");
       }
    }

    fn lados_isosceles(self)->bool{
        (self.lado_1==self.lado_2 && self.lado_1 != self.lado_3) || 
        (self.lado_1==self.lado_3 && self.lado_1 != self.lado_2) ||
        (self.lado_2==self.lado_3 && self.lado_2 != self.lado_1)
    }

    pub fn determinar_tipo(self)->TipoTriangulo{
        if self.lado_1==self.lado_2 && self.lado_1==self.lado_3{
            TipoTriangulo::Equilatero
        }else if self.lados_isosceles(){
            TipoTriangulo::Isosceles
        }else{
            TipoTriangulo::Escaleno
        }
    }
    
    pub fn calcular_area(&self)->f32{
        let s:f32=self.calcular_perimetro()/2.0;
        let rad:f32=s*(s-self.lado_1)*(s-self.lado_2)*(s-self.lado_3);
        rad.sqrt()
    }

    pub fn calcular_perimetro(&self)->f32{
        self.lado_1+self.lado_2+self.lado_3
    }
}


#[cfg(test)]
mod test{
    use crate::tp3::ej4::Triangulo;
    use crate::tp3::ej4::TipoTriangulo;

    #[test]
    fn new_test(){
        let _=Triangulo::new(5.0,5.0,5.0);
    }
    
    #[test]
    #[should_panic(expected = "Los valores no corresponden a un triángulo válido")]
    fn new_panic_test(){
        let _triangulo=Triangulo::new(1.0,2.0,3.0);
    }

    #[test]
    fn determinar_tipo_equilatero_test(){
        let triangulo=Triangulo::new(5.0,5.0,5.0);
        let es_equilatero:bool= match triangulo.determinar_tipo(){
            TipoTriangulo::Equilatero=> true,
            _=>false,
        };
        assert!(es_equilatero);
    }
    
    
    #[test]
    fn determinar_tipo_isosceles_test(){
        let triangulo=Triangulo::new(5.0,5.0,3.0);
        let es_isosceles:bool= match triangulo.determinar_tipo(){
            TipoTriangulo::Isosceles=> true,
            _=>false,
        };
        assert!(es_isosceles);
    }
    
    #[test]
    fn determinar_tipo_escaleno_test(){
        let triangulo=Triangulo::new(3.0,4.0,5.0);
        let es_escaleno:bool= match triangulo.determinar_tipo(){
            TipoTriangulo::Escaleno=> true,
            _=>false,
        };
        assert!(es_escaleno);
    }

    #[test]
    fn calcular_area_equilatero_test(){
        let triangulo=Triangulo::new(5.0,5.0,5.0);
        assert_eq!(triangulo.calcular_area(),10.825317);
    }

    #[test]
    fn calcular_area_isosceles_test(){
        let triangulo=Triangulo::new(5.0,5.0,3.0);
        assert_eq!(triangulo.calcular_area(),7.154544);
    }

    #[test]
    fn calcular_area_escaleno_test(){
        let triangulo=Triangulo::new(3.0,4.0,5.0);
        assert_eq!(triangulo.calcular_area(),6.0);
    }

    #[test]
    fn calcular_perimetro_equilatero_test(){
        let triangulo=Triangulo::new(5.0,5.0,5.0);
        assert_eq!(triangulo.calcular_perimetro(),15.0);
    }

    #[test]
    fn calcular_perimetro_isosceles_test(){
        let triangulo=Triangulo::new(5.0,5.0,3.0);
        assert_eq!(triangulo.calcular_perimetro(),13.0);
    }

    #[test]
    fn calcular_perimetro_escaleno_test(){
        let triangulo=Triangulo::new(3.0,4.0,5.0);
        assert_eq!(triangulo.calcular_perimetro(),12.0);
    }
}

