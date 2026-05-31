#[derive(Debug)]
#[allow(unused)]
struct Producto{
    nombre:String,
    precio_bruto:f32,
    id:u32,
}

#[allow(unused)]
impl Producto{

    pub fn new(nombre:String,precio_bruto:f32,id:u32)->Producto{
        Producto{
            nombre,
            precio_bruto,
            id,
        }
    }

    pub fn calcular_impuestos(&self, porcentaje_de_impuestos:f32)->f32{
        self.precio_bruto*porcentaje_de_impuestos
    }

    pub fn calcular_descuento(&self, porcentaje_de_descuento:f32)->f32{
        self.precio_bruto*porcentaje_de_descuento
    }

    pub fn calcular_precio_total(&self, porcentaje_de_impuestos:f32, porcentaje_descuento:f32)->f32{
        self.precio_bruto
        +self.calcular_impuestos(porcentaje_de_impuestos)
        -self.calcular_descuento(porcentaje_descuento)
    }

}

#[cfg(test)]
mod test{
    use crate::tp3::ej5::Producto;

    #[test]
    fn calcular_impuestos_test(){
        let producto=Producto::new("Chocolate".to_string(),1000.0,1);
        assert_eq!(producto.calcular_impuestos(0.21),210.0);
    }

    #[test]
    fn calcular_impuestos_sin_impuestos_test(){
        let producto=Producto::new("Chocolate".to_string(),1000.0,1);
        assert_eq!(producto.calcular_impuestos(0.0),0.0);
    }

    #[test]
    fn calcular_descuento_test(){
        let producto=Producto::new("Chocolate".to_string(),1000.0,1);
        assert_eq!(producto.calcular_descuento(0.1),100.0);
    }

    #[test]
    fn calcular_descuento_sin_descuentos_test(){
        let producto=Producto::new("Chocolate".to_string(),1000.0,1);
        assert_eq!(producto.calcular_descuento(0.0),0.0);
    }

    #[test]
    fn calcular_precio_total_test(){
        let producto=Producto::new("Chocolate".to_string(),1000.0,1);
        assert_eq!(producto.calcular_precio_total(0.21,0.1),1110.0);
    }

    #[test]
    fn calcular_precio_total_sin_impuestos_test(){
        let producto=Producto::new("Chocolate".to_string(),1000.0,1);
        assert_eq!(producto.calcular_precio_total(0.0,0.1),900.0);
    }
    #[test]
    fn calcular_precio_total_sin_descuentos_test(){
        let producto=Producto::new("Chocolate".to_string(),1000.0,1);
        assert_eq!(producto.calcular_precio_total(0.21,0.0),1210.0);
    }
    #[test]
    fn calcular_precio_total_sin_descuentos_ni_impuestos_test(){
        let producto=Producto::new("Chocolate".to_string(),1000.0,1);
        assert_eq!(producto.calcular_precio_total(0.0,0.0),1000.0);
    }
}

