#[derive(Debug)]
struct Persona{
    nombre: String,
    edad: u32,
    direccion: Option<String>,
}
#[allow(unused)]
impl Persona{

    //pasando los parámetros correspondientes, crea una Persona y la retorna.
    pub fn new(nombre: String, edad: u32, direccion: Option<String>) -> Persona{
        Persona{
            nombre,
            edad,
            direccion,
        }
    }

    //que retorna un string con los datos de la persona concatenados sobre el mensaje ejecutado por ej:
    // person.to_string(), donde person es una variable del tipo Persona.

    pub fn to_string(&self) -> String{
        let dir= match &self.direccion{
            None=> "No se registró dirección",
            Some(direccion)=> direccion,
        };
        
        format!("Nombre: {}, edad: {}, direccion: {}", self.nombre, self.edad, dir)
    }

    //retorna la edad de la persona.
    pub fn obtener_edad(&self) -> u32{
        self.edad
    }

    //
    fn actualizar_direccion(&mut self, nueva_direccion:Option<String>){
        self.direccion=nueva_direccion;
    }

}

#[cfg(test)]
mod test{
    use crate::tp3::ej1::Persona;

    //tests de to_string
    #[test]    
    fn to_string_persona_con_direccion_test(){
        let persona=Persona::new("Ana".to_string(),24,Some("333 333".to_string()));
        assert_eq!(persona.to_string(),"Nombre: Ana, edad: 24, direccion: 333 333");
    }

    #[test]
    fn to_string_persona_sin_direccion_test(){
        let persona=Persona::new("Ana".to_string(),24,None);
        assert_eq!(persona.to_string(),"Nombre: Ana, edad: 24, direccion: No se registró dirección");
    }
    
    //Tests de obtener_edad
    #[test]
    fn obtener_edad_test(){
        let persona=Persona::new("Ana".to_string(),24,Some("333 333".to_string()));
        assert_eq!(persona.obtener_edad(),24);
    }
    
    //Test de actualizar direccion 
    #[test]
    fn actualizar_direccion_cambiar_direccion_test(){
        let mut persona=Persona::new("Ana".to_string(),24,Some("333 333".to_string()));
        assert_eq!(persona.direccion,Some("333 333".to_string()));
        persona.actualizar_direccion(Some("44 444".to_string()));
        assert_eq!(persona.direccion,Some("44 444".to_string()));
    }
    
    #[test]
    fn actualizar_direccion_quitar_direccion_test(){
        let mut persona=Persona::new("Ana".to_string(),24,Some("333 333".to_string()));
        assert_eq!(persona.direccion,Some("333 333".to_string()));
        persona.actualizar_direccion(None);
        assert_eq!(persona.direccion,None);
    }

    #[test]
    fn actualizar_direccion_cargar_direccion_test(){
        let mut persona=Persona::new("Ana".to_string(),24,None);
        assert_eq!(persona.direccion,None);
        persona.actualizar_direccion(Some("333 333".to_string()));
        assert_eq!(persona.direccion,Some("333 333".to_string()));
    }

}


