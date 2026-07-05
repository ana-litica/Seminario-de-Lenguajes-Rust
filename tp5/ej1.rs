//El coverage del ejercicio original dio 100.00%
    use std::fmt::Debug;
    use std::{fs::File, io::Write};
    use std::{fmt::Display};
    use serde::{Serialize,Deserialize};


#[derive(Clone,Debug,Serialize,Deserialize)]
#[allow(unused)]
enum Colores{
    Rojo,
    Verde,
    Azul,
    Amarillo,
    Blanco,
    Negro,
}

#[derive(Clone,Debug,Serialize,Deserialize)]
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
    path_archivo:String, 
}

#[derive(Debug)]
enum Errores{
    ConcesionariaLlena,
    AutoExistente,
    ErrorDeArchivo,
}

impl Display for Errores{
     fn fmt(&self, f: &mut std::fmt::Formatter<'_>)-> std::fmt::Result{
        match self{
            Errores::ConcesionariaLlena=>write!(f,"Capacidad máxima del concesionario alcanzada"),
            Errores::AutoExistente=>write!(f,"El auto ya está en el concesionario"),
            Errores::ErrorDeArchivo=>write!(f,"No se pudo operar con el archivo"),
        }
     }
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

    pub fn es_el_mismo_auto(self,otro_auto:&Auto) ->bool{
        self.marca==otro_auto.marca && self.modelo==otro_auto.modelo &&
        self.año==otro_auto.año && self.precio_bruto==otro_auto.precio_bruto &&
        self.mismo_color(otro_auto.color.clone())
    }
}

#[allow(unused)]
impl ConcesionarioAuto{

    pub fn new(nombre:String,direccion:String,capacidad_maxima:usize)->ConcesionarioAuto{
        Self::inner_new(nombre, direccion, capacidad_maxima,"src/tp5/archivo_autos".to_string())
    }

    fn inner_new(nombre:String,direccion:String,capacidad_maxima:usize,path:String)->ConcesionarioAuto{
        ConcesionarioAuto {
            nombre,
            direccion,
            capacidad_maxima,
            autos:Vec::new(),
            path_archivo:path
        }
    }

    fn escribir_en_archivo(&self)->Result<(),Errores>{
        let mut archivo=File::create(&self.path_archivo).map_err(|e|Errores::ErrorDeArchivo)?;
        let auto_serializados=serde_json::to_string(&self.autos).map_err(|e|Errores::ErrorDeArchivo)?;
        archivo.write_all(&auto_serializados.as_bytes()).map_err(|e|Errores::ErrorDeArchivo)?;
        Ok(())
    }

    pub fn agregar_auto(&mut self, nuevo_auto:&Auto)->Result<bool,String>{
        if self.autos.len()<self.capacidad_maxima && self.buscar_auto(&nuevo_auto).is_none(){
            self.autos.push(nuevo_auto.clone()); 
            if let Err(er)=self.escribir_en_archivo(){
                self.autos.pop();
                return Err(er.to_string());
            }
            Ok(true)
        }else if self.autos.len()>=self.capacidad_maxima{
            Err(Errores::ConcesionariaLlena.to_string())
        }else{
            Err(Errores::AutoExistente.to_string())
        }
    }

    pub fn eliminar_auto(&mut self,auto:&Auto)->Result<bool,String>{
        let mut eliminado:bool=false;
        let mut indice:usize=0;
        
        while !eliminado && indice<self.autos.len(){
            if self.autos[indice].clone().es_el_mismo_auto(&auto){
                let back_up=self.autos[indice].clone();
                self.autos.remove(indice);
                if let Err(er)=self.escribir_en_archivo(){
                    self.autos.insert(indice, back_up);
                    return Err(er.to_string());
                }
                eliminado=true;

            }
            indice+=1;
        }
        return Ok(eliminado);
    }

    pub fn buscar_auto(&self,auto:&Auto)->Option<&Auto>{
        let mut indice:usize=0;
        while  indice<self.autos.len(){         
            if self.autos[indice].clone().es_el_mismo_auto(&auto){
                return Some(&self.autos[indice]);
            }
            indice+=1;
        }
            None
    }

}

#[cfg(test)]
mod test{
    use std::fs;
    use crate::tp5::ej1::ConcesionarioAuto;
    use crate::tp5::ej1::Auto;
    use crate::tp5::ej1::Colores;

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
        let auto=Auto::new("Peugeot".to_string(),"X5".to_string(),1998,100000.0,Colores::Azul);
        assert_eq!(auto.calcular_precio(),120000.0);
    }

    //no es primario, no es BMW y año menor a 2000
    #[test]
    fn calcular_precio_no_primario_no_bmw_año_menor_test(){
        let auto=Auto::new("Peugeot".to_string(),"X5".to_string(),1998,100000.0,Colores::Verde);
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
        let auto=Auto::new("BMW".to_string(),"X5".to_string(),2007,100000.0,Colores::Verde);
        assert_eq!(auto.calcular_precio(),105000.0);
    }

    #[test]
    fn es_el_mismo_auto_igual_auto_test(){
        let auto:Auto=Auto::new("BMW".to_string(),"X5".to_string(),2007,100000.0,Colores::Blanco);
        assert!(auto.es_el_mismo_auto(&Auto::new("BMW".to_string(),"X5".to_string(),2007,100000.0,Colores::Blanco)));
    }

    #[test]
    fn es_el_mismo_auto_diferente_auto_test(){
        let auto:Auto=Auto::new("BMW".to_string(),"X5".to_string(),2007,100000.0,Colores::Blanco);
        assert_eq!(auto.es_el_mismo_auto(&Auto::new("Peugeot".to_string(),"X5".to_string(),2007,100000.0,Colores::Blanco)),false);
    }

    //tests ConcesionarioAuto
    //test agregar_auto
     #[test]
    fn agregar_auto_con_exito_test(){
        let mut concesionario=ConcesionarioAuto::new("Concesionaria".to_string(),"333 333".to_string(),25);
        let auto=Auto::new("BMW".to_string(),"X5".to_string(),2007,100000.0,Colores::Blanco);
        assert_eq!(concesionario.agregar_auto(&auto),Ok(true));
    }

    #[test]
    fn agregar_auto_insercion_fallida_por_capacidad_superada_test(){
        let mut concesionario=ConcesionarioAuto::new("Concesionaria".to_string(),"333 333".to_string(),2);
        let auto1=Auto::new("BMW".to_string(),"X76".to_string(),2007,100000.0,Colores::Blanco);
        let auto2=Auto::new("BMW".to_string(),"X5".to_string(),2007,100000.0,Colores::Amarillo);
        let auto3=Auto::new("Fiat".to_string(),"X5".to_string(),2007,100000.0,Colores::Blanco);
        let _=concesionario.agregar_auto(&auto1);
        let _=concesionario.agregar_auto(&auto2);
        assert_eq!(concesionario.agregar_auto(&auto3),Err("Capacidad máxima del concesionario alcanzada".to_string()));
    }

    #[test]
    fn agregar_auto_insercion_fallida_por_auto_existente_test(){
        let mut concesionario=ConcesionarioAuto::new("Concesionaria".to_string(),"333 333".to_string(),15);
        let auto1=Auto::new("BMW".to_string(),"X5".to_string(),2007,100000.0,Colores::Blanco);
        let auto2=Auto::new("Fiat".to_string(),"X5".to_string(),2007,100000.0,Colores::Blanco);
        let auto3=Auto::new("Ferrari".to_string(),"X5".to_string(),2007,100000.0,Colores::Blanco);
        
        let _=concesionario.agregar_auto(&auto1);
        let _=concesionario.agregar_auto(&auto2);
        let _=concesionario.agregar_auto(&auto3);
        assert_eq!(concesionario.agregar_auto(&auto3),Err("El auto ya está en el concesionario".to_string()));
    }

      #[test]
    fn eliminar_auto_test(){
        let mut concesionario=ConcesionarioAuto::new("Concesionaria".to_string(),"333 333".to_string(),25);
        let auto1=Auto::new("BMW".to_string(),"X5".to_string(),2007,100000.0,Colores::Blanco);
        let auto2=Auto::new("Fiat".to_string(),"X5".to_string(),2007,100000.0,Colores::Blanco);
        let auto3=Auto::new("Peugeot".to_string(),"X5".to_string(),2007,100000.0,Colores::Blanco);
        let _=concesionario.agregar_auto(&auto1);
        let _=concesionario.agregar_auto(&auto2.clone());
        let _=concesionario.agregar_auto(&auto3);
        assert_eq!(concesionario.autos.len(),3);
        let _=concesionario.eliminar_auto(&auto2);
        assert_eq!(concesionario.autos.len(),2);
    }

    #[test]
    fn eliminar_auto_inexistente_test(){
        let mut concesionario=ConcesionarioAuto::new("Concesionaria".to_string(),"333 333".to_string(),25);
        let auto1=Auto::new("BMW".to_string(),"X5".to_string(),2007,100000.0,Colores::Blanco);
        let auto2=Auto::new("Fiat".to_string(),"X5".to_string(),2007,100000.0,Colores::Blanco);
        let auto3=Auto::new("Peugeot".to_string(),"X5".to_string(),2007,100000.0,Colores::Blanco);
        let auto_a_eliminar=Auto::new("Fiat".to_string(),"X3".to_string(),2007,100000.0,Colores::Blanco);

        let _=concesionario.agregar_auto(&auto1);
        let _=concesionario.agregar_auto(&auto2);
        let _=concesionario.agregar_auto(&auto3);
        let _=concesionario.eliminar_auto(&auto_a_eliminar);
        assert_eq!(concesionario.autos.len(),3);
    }

    #[test]
    fn eliminar_auto_lista_vacia_test(){
        let mut concesionario=ConcesionarioAuto::new("Concesionaria".to_string(),"333 333".to_string(),25);
        let auto_a_eliminar=Auto::new("BMW".to_string(),"X5".to_string(),2007,100000.0,Colores::Blanco);
        let _=concesionario.eliminar_auto(&auto_a_eliminar);
        assert_eq!(concesionario.autos.len(),0);
    }

      #[test]
    fn buscar_auto_test(){
        let mut concesionario=ConcesionarioAuto::new("Concesionaria".to_string(),"333 333".to_string(),25);
        let auto1: Auto=Auto::new("BMW".to_string(),"X5".to_string(),2007,100000.0,Colores::Blanco);
        let auto2: Auto=Auto::new("Fiat".to_string(),"X5".to_string(),2007,100000.0,Colores::Blanco);
        let auto3: Auto=Auto::new("Peugeot".to_string(),"X5".to_string(),2007,100000.0,Colores::Blanco);
        let _=concesionario.agregar_auto(&auto1);
        let _=concesionario.agregar_auto(&auto2.clone());
        let _=concesionario.agregar_auto(&auto3);
        let auto_buscar: Auto=Auto::new("Fiat".to_string(),"X5".to_string(),2007,100000.0,Colores::Blanco);
        let resultado:&Auto=concesionario.buscar_auto(&auto2).unwrap();
        assert!(auto_buscar.clone().es_el_mismo_auto(&resultado));
    }

    #[test]
    fn buscar_auto_inexistente_test(){
        let mut concesionario=ConcesionarioAuto::new("Concesionaria".to_string(),"333 333".to_string(),25);
        let auto1=Auto::new("BMW".to_string(),"X5".to_string(),2007,100000.0,Colores::Blanco);
        let auto2=Auto::new("Fiat".to_string(),"X5".to_string(),2007,100000.0,Colores::Blanco);
        let auto3=Auto::new("Peugeot".to_string(),"X5".to_string(),2007,100000.0,Colores::Blanco);
        let _=concesionario.agregar_auto(&auto1);
        let _=concesionario.agregar_auto(&auto2);
        let _=concesionario.agregar_auto(&auto3);
        let auto_buscar=Auto::new("Fiat".to_string(),"X3".to_string(),2007,100000.0,Colores::Blanco);
        let resultado=concesionario.buscar_auto(&auto_buscar);
        assert!(resultado.is_none());
    }

    #[test]
    fn buscar_auto_lista_vacia_test(){
        let concesionario=ConcesionarioAuto::new("Concesionaria".to_string(),"333 333".to_string(),25);
        let auto1=Auto::new("BMW".to_string(),"X5".to_string(),2007,100000.0,Colores::Blanco);
        let resultado=concesionario.buscar_auto(&auto1);
        assert!(resultado.is_none());
    }

    #[test]
    fn agregar_auto_insercion_fallida_por_imposibilidad_de_crear_el_archivo_test(){
        let path="src/tp5/testeo_de_errores_creacion_autos";
        let _ = std::fs::remove_file(path);
        let _ = std::fs::remove_dir(path);
        //El archivo existe pero es un directorio, no se pueden escribir autos en él

        fs::create_dir(path).unwrap();

        let mut concesionario=ConcesionarioAuto::inner_new("Concesionaria".to_string(),
                "333 333".to_string(),25,
                path.to_string());
        let auto=Auto::new("BMW".to_string(),"X5".to_string(),2007,100000.0,Colores::Blanco);
        assert_eq!(concesionario.agregar_auto(&auto),Err("No se pudo operar con el archivo".to_string()));

        let _=std::fs::remove_dir(path);
    }

    #[test]
    fn eliminar_auto_eliminacion_fallida_por_imposibilidad_de_crear_el_archivo_test(){
        let path="src/tp5/testeo_de_errores_autos";
        let _ = std::fs::remove_file(path);
        let _ = std::fs::remove_dir(path);
        //El archivo existe pero es un directorio, no se pueden escribir autos en él

        fs::create_dir(path).unwrap();

        let mut concesionario=ConcesionarioAuto::inner_new("Concesionaria".to_string(),
                "333 333".to_string(),25,
                path.to_string());
        let auto=Auto::new("BMW".to_string(),"X5".to_string(),2007,100000.0,Colores::Blanco);
        //agregué con push porque sino el agregar falla
        concesionario.autos.push(auto.clone());
        assert_eq!(concesionario.eliminar_auto(&auto),Err("No se pudo operar con el archivo".to_string()));

        let _=std::fs::remove_dir(path);
    }

}

