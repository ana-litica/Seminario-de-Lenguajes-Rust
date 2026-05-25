#[derive(Clone,Debug)]
#[allow(unused)]
enum Colores{
    Rojo,
    Verde,
    Azul,
    Amarillo,
    Blanco,
    Negro,
}

#[derive(Clone,Debug)]
#[allow(unused)]
struct Auto{
    marca:String,
    modelo:String,
    año:u32,
    precio_bruto:f32,
    color:Colores,
}

#[derive(Debug)]
#[allow(unused)]
struct ConcesionarioAuto{
    nombre:String,
    direccion:String,
    capacidad_maxima:usize,
    autos:Vec<Auto>,
}

#[allow(unused)]
impl Auto{

    pub fn new(marca:String,modelo:String,año:u32,precio_bruto:f32,color:Colores)->Auto{
        Auto{
            marca,
            modelo,
            año,
            precio_bruto,
            color,
        }
    }

    fn es_primario(&self)->f32{   
        let adicional:f32=match self.color{
            Colores::Rojo|Colores::Amarillo|Colores::Azul=>self.precio_bruto*0.25,
            _=>-self.precio_bruto*0.1,
        };
        adicional
    }

    fn es_bmw(&self)->f32{
        let mut descuento:f32=0.0;
        if self.marca=="BMW".to_string(){
            descuento=0.15;
        }
        self.precio_bruto*descuento
    }

    fn recargo_año(&self)->f32{
        if self.año<2000{
            -self.precio_bruto*0.05
        }else{
            0.0
        }  
    }

    pub fn calcular_precio(&self)->f32{
        self.precio_bruto+self.es_primario()+self.es_bmw()+self.recargo_año()
    }

    fn mismo_color(self, color:Colores)->bool{
        match (self.color,color){
            (Colores::Amarillo,Colores::Amarillo)|(Colores::Azul,Colores::Azul)|
            (Colores::Rojo,Colores::Rojo)|(Colores::Verde,Colores::Verde)|(Colores::Blanco,Colores::Blanco)|
            (Colores::Negro,Colores::Negro)=>true,
            _=>false,
        }
    }

    pub fn es_el_mismo_auto(self,otro_auto:Auto) ->bool{
        self.marca==otro_auto.marca && self.modelo==otro_auto.modelo &&
        self.año==otro_auto.año && self.precio_bruto==otro_auto.precio_bruto &&
        self.mismo_color(otro_auto.color)
    }
}

#[allow(unused)]
impl ConcesionarioAuto{

    pub fn new(nombre:String,direccion:String,capacidad_maxima:usize)->ConcesionarioAuto{
        ConcesionarioAuto{
            nombre,
            direccion,
            capacidad_maxima,
            autos:Vec::new(),
        }
    }

    pub fn agregar_auto(&mut self, nuevo_auto:Auto)->bool{
        if self.autos.len()<self.capacidad_maxima{
            self.autos.push(nuevo_auto);
            true
        }else{
            false
        }
    }

    pub fn eliminar_auto(&mut self,auto:Auto){
        let mut eliminado:bool=false;
        let mut indice:usize=0;
        while !eliminado && indice<self.autos.len(){
            if self.autos[indice].clone().es_el_mismo_auto(auto.clone()){
                self.autos.remove(indice);
                eliminado=true;
            }
            indice+=1;
            }
    }

    pub fn buscar_auto(self,auto:Auto)->Auto{
        let mut indice:usize=0;
        while  indice<self.autos.len(){         
            if self.autos[indice].clone().es_el_mismo_auto(auto.clone()){
                return auto;
            }
            indice+=1;
        }
            panic!("El auto no se encuentra en la lista");
    }

}

#[cfg(test)]
mod test{
    use crate::tp3::ej7::ConcesionarioAuto;
    use crate::tp3::ej7::Auto;
    use crate::tp3::ej7::Colores;

    //Tests Auto
    //Test caclular_precio
    //es primario, es BMW y año menor a 2000
    #[test]
    fn calcular_precio_color_primario_bmw_año_menor_test(){
        let auto=Auto::new("BMW".to_string(),"320i".to_string(),1998,100000.0,Colores::Rojo);
        assert_eq!(auto.calcular_precio(),135000.0);
    }

    //no es primario, es BMW y año menor a 2000
    #[test]
    fn calcular_precio_color_no_primario_bmw_año_menor_test(){
        let auto=Auto::new("BMW".to_string(),"X5".to_string(),1998,100000.0,Colores::Blanco);
        assert_eq!(auto.calcular_precio(),100000.0);
    }

    //es primario, no es BMW y año menor a 2000
    #[test]
    fn calcular_precio_color_primario_no_bmw_año_menor_test(){
        let auto=Auto::new("Peugeot".to_string(),"X5".to_string(),1998,100000.0,Colores::Rojo);
        assert_eq!(auto.calcular_precio(),120000.0);
    }

    //no es primario, no es BMW y año menor a 2000
    #[test]
    fn calcular_precio_no_primario_no_bmw_año_menor_test(){
        let auto=Auto::new("Peugeot".to_string(),"X5".to_string(),1998,100000.0,Colores::Blanco);
        assert_eq!(auto.calcular_precio(),85000.0);
    }

    //es primario, no es BMW y año mayor a 2000
    #[test]
    fn calcular_precio_color_primario_no_bmw_año_mayor_test(){
        let auto=Auto::new("Peugeot".to_string(),"X5".to_string(),2007,100000.0,Colores::Rojo);
        assert_eq!(auto.calcular_precio(),125000.0);
    }

    //no es primario, no es BMW y año mayor a 2000
    #[test]
    fn calcular_precio_color_no_primario_no_bmw_año_mayor_test(){
        let auto=Auto::new("Peugeot".to_string(),"X5".to_string(),2007,100000.0,Colores::Blanco);
        assert_eq!(auto.calcular_precio(),90000.0);
    }

    //es primario, es BMW y año mayor a 2000
    #[test]
    fn calcular_precio_color_primario_bmw_año_mayor_test(){
        let auto=Auto::new("BMW".to_string(),"X5".to_string(),2007,100000.0,Colores::Rojo);
        assert_eq!(auto.calcular_precio(),140000.0);
    }

    //no es primario, es BMW y año mayor a 2000
    #[test]
    fn calcular_precio_color_no_primario_bmw_año_mayor_test(){
        let auto=Auto::new("BMW".to_string(),"X5".to_string(),2007,100000.0,Colores::Blanco);
        assert_eq!(auto.calcular_precio(),105000.0);
    }

    #[test]
    fn es_el_mismo_auto_igual_auto_test(){
        let auto:Auto=Auto::new("BMW".to_string(),"X5".to_string(),2007,100000.0,Colores::Blanco);
        assert!(auto.es_el_mismo_auto(Auto::new("BMW".to_string(),"X5".to_string(),2007,100000.0,Colores::Blanco)));
    }

    #[test]
    fn es_el_mismo_auto_diferente_auto_test(){
        let auto:Auto=Auto::new("BMW".to_string(),"X5".to_string(),2007,100000.0,Colores::Blanco);
        assert_eq!(auto.es_el_mismo_auto(Auto::new("Peugeot".to_string(),"X5".to_string(),2007,100000.0,Colores::Blanco)),false);
    }

    //tests ConcesionarioAuto
    //test agregar_auto
     #[test]
    fn agregar_auto_con_exito_test(){
        let mut concesionario=ConcesionarioAuto::new("Concesionaria".to_string(),"333 333".to_string(),25);
        let auto=Auto::new("BMW".to_string(),"X5".to_string(),2007,100000.0,Colores::Blanco);
        assert_eq!(concesionario.agregar_auto(auto),true);
    }

    #[test]
    fn agregar_auto_insercion_fallida_test(){
        let mut concesionario=ConcesionarioAuto::new("Concesionaria".to_string(),"333 333".to_string(),2);
        let auto1=Auto::new("BMW".to_string(),"X5".to_string(),2007,100000.0,Colores::Blanco);
        let auto2=Auto::new("BMW".to_string(),"X5".to_string(),2007,100000.0,Colores::Blanco);
        let auto3=Auto::new("BMW".to_string(),"X5".to_string(),2007,100000.0,Colores::Blanco);
        concesionario.agregar_auto(auto1);
        concesionario.agregar_auto(auto2);
        assert_eq!(concesionario.agregar_auto(auto3),false);
    }

      #[test]
    fn eliminar_auto_test(){
        let mut concesionario=ConcesionarioAuto::new("Concesionaria".to_string(),"333 333".to_string(),25);
        let auto1=Auto::new("BMW".to_string(),"X5".to_string(),2007,100000.0,Colores::Blanco);
        let auto2=Auto::new("Fiat".to_string(),"X5".to_string(),2007,100000.0,Colores::Blanco);
        let auto3=Auto::new("Peugeot".to_string(),"X5".to_string(),2007,100000.0,Colores::Blanco);
        concesionario.agregar_auto(auto1);
        concesionario.agregar_auto(auto2);
        concesionario.agregar_auto(auto3);
        assert_eq!(concesionario.autos.len(),3);
        concesionario.eliminar_auto(Auto::new("Fiat".to_string(),"X5".to_string(),2007,100000.0,Colores::Blanco));
        assert_eq!(concesionario.autos.len(),2);
    }

    #[test]
    fn eliminar_auto_inexistente_test(){
        let mut concesionario=ConcesionarioAuto::new("Concesionaria".to_string(),"333 333".to_string(),25);
        let auto1=Auto::new("BMW".to_string(),"X5".to_string(),2007,100000.0,Colores::Blanco);
        let auto2=Auto::new("Fiat".to_string(),"X5".to_string(),2007,100000.0,Colores::Blanco);
        let auto3=Auto::new("Peugeot".to_string(),"X5".to_string(),2007,100000.0,Colores::Blanco);
        concesionario.agregar_auto(auto1);
        concesionario.agregar_auto(auto2);
        concesionario.agregar_auto(auto3);
        concesionario.eliminar_auto(Auto::new("Fiat".to_string(),"X3".to_string(),2007,100000.0,Colores::Blanco));
        assert_eq!(concesionario.autos.len(),3);
    }

    #[test]
    fn eliminar_auto_lista_vacia_test(){
        let mut concesionario=ConcesionarioAuto::new("Concesionaria".to_string(),"333 333".to_string(),25);
        concesionario.eliminar_auto(Auto::new("BMW".to_string(),"X5".to_string(),2007,100000.0,Colores::Blanco));
        assert_eq!(concesionario.autos.len(),0);
    }

      #[test]
    fn buscar_auto_test(){
        let mut concesionario=ConcesionarioAuto::new("Concesionaria".to_string(),"333 333".to_string(),25);
        let auto1: Auto=Auto::new("BMW".to_string(),"X5".to_string(),2007,100000.0,Colores::Blanco);
        let auto2: Auto=Auto::new("Fiat".to_string(),"X5".to_string(),2007,100000.0,Colores::Blanco);
        let auto3: Auto=Auto::new("Peugeot".to_string(),"X5".to_string(),2007,100000.0,Colores::Blanco);
        concesionario.agregar_auto(auto1);
        concesionario.agregar_auto(auto2);
        concesionario.agregar_auto(auto3);
        let auto_buscar: Auto=Auto::new("Fiat".to_string(),"X5".to_string(),2007,100000.0,Colores::Blanco);
        let resultado:Auto=concesionario.buscar_auto(Auto::new("Fiat".to_string(),"X5".to_string(),2007,100000.0,Colores::Blanco));
        assert!(auto_buscar.clone().es_el_mismo_auto(resultado));
    }

    #[test]
    #[should_panic(expected = "El auto no se encuentra en la lista")]
    fn buscar_auto_inexistente_test(){
        let mut concesionario=ConcesionarioAuto::new("Concesionaria".to_string(),"333 333".to_string(),25);
        let auto1=Auto::new("BMW".to_string(),"X5".to_string(),2007,100000.0,Colores::Blanco);
        let auto2=Auto::new("Fiat".to_string(),"X5".to_string(),2007,100000.0,Colores::Blanco);
        let auto3=Auto::new("Peugeot".to_string(),"X5".to_string(),2007,100000.0,Colores::Blanco);
        concesionario.agregar_auto(auto1);
        concesionario.agregar_auto(auto2);
        concesionario.agregar_auto(auto3);
        let auto_buscar=Auto::new("Fiat".to_string(),"X3".to_string(),2007,100000.0,Colores::Blanco);
        concesionario.buscar_auto(auto_buscar);
    }

    #[test]
    #[should_panic(expected = "El auto no se encuentra en la lista")]
    fn buscar_auto_lista_vacia_test(){
        let concesionario=ConcesionarioAuto::new("Concesionaria".to_string(),"333 333".to_string(),25);
        let auto1=Auto::new("BMW".to_string(),"X5".to_string(),2007,100000.0,Colores::Blanco);
        concesionario.buscar_auto(auto1);
    }

}

