//El coverage del ejercicio original me dio 97.37%
use std::collections::VecDeque;
use crate::tp5::fecha::Fecha;
use std::fmt::Debug;
use std::{fs::File, io::Write};
use serde::Serialize;

#[derive(Debug,Serialize)]
#[allow(unused)]
struct Veterinaria{
    nombre:String,
    direccion:String,
    id:u32,
    atenciones:Vec<Atencion>,
    cola_de_atencion:VecDeque<Mascota>,
    path_archivo:String,
}

#[derive(Debug,Clone,Serialize)]
#[allow(unused)]
enum Animal{
    Perro,
    Gato,
    Caballo,
    Otros,
}

#[derive(Debug,Clone,Serialize)]
#[allow(unused)]
struct Mascota{
    nombre:String,
    edad:u16,
    tipo_animal:Animal,
    dueño:Dueño,
}

#[derive(Debug,Clone,Serialize)]
#[allow(unused)]
struct Dueño{
    nombre:String,
    direccion:String,
    telefono:u64,
}

#[derive(Debug,Clone,Serialize)]
#[allow(unused)]
struct Atencion{
    paciente:Mascota,
    diagnostico:String,
    tratamiento:String,
    proxima_visita:Fecha,
}

#[derive(Debug)]
enum Errores{
    ErrorDeCreacionDeArchivo,
    ErrorDeEscrituraDeDatos,
    ErrorDeSerializacionDeDatos,
}

#[allow(unused)]
impl Mascota{

    pub fn new(nombre:String,edad:u16,tipo_animal:Animal,dueño:Dueño)->Mascota{
        Mascota { nombre, edad, tipo_animal, dueño }
    }

    fn mismo_tipo(self, tipo:&Animal)->bool{
        match (self.tipo_animal.clone(),tipo){
            (Animal::Perro,Animal::Perro)|(Animal::Gato,Animal::Gato)|
            (Animal::Caballo,Animal::Caballo)|(Animal::Otros,Animal::Otros)=>true,
            _=>false,
        }
    }

    fn misma_mascota(&self,mascota:&Mascota)->bool{
        self.nombre==mascota.nombre && self.edad==mascota.edad && self.clone().mismo_tipo(&mascota.tipo_animal)
        && self.clone().dueño.mismo_dueño(&mascota.dueño)
    }
}

#[allow(unused)]
impl Dueño{

    pub fn new(nombre:String,direccion:String,telefono:u64)->Dueño{
        Dueño { nombre, direccion, telefono }
    }

    fn mismo_dueño(self,dueño:&Dueño)->bool{
        self.nombre==dueño.nombre && self.direccion==dueño.direccion && self.telefono==dueño.telefono
    }
}

#[allow(unused)]
impl Atencion{

    pub fn new(paciente:Mascota, diagnostico:String,tratamiento:String,proxima_visita:Fecha)->Atencion{
        Atencion { paciente, diagnostico, tratamiento, proxima_visita }
    }

    pub fn misma_atencion(&self,atencion:&Atencion)->bool{
        self.paciente.misma_mascota(&atencion.paciente) && self.diagnostico==atencion.diagnostico &&
        self.tratamiento==atencion.tratamiento && self.proxima_visita.misma_fecha(&atencion.proxima_visita)
    }
}

#[allow(unused)]
impl Veterinaria{

    pub fn new(nombre:String, direccion:String,id:u32)->Veterinaria{
        Self::inner_new(nombre, direccion, id, "src/tp5/archivo_atenciones_veterinarias".to_string())
    }

    fn inner_new(nombre:String, direccion:String,id:u32,path:String)->Veterinaria{
        Veterinaria { nombre, direccion, id, atenciones: Vec::new(),cola_de_atencion:VecDeque::new(),path_archivo:path  }
    }


    pub fn agregar_mascota(&mut self,mascota:Mascota){
        self.cola_de_atencion.push_back(mascota);
    }

    pub fn agregar_mascota_con_prioridad(&mut self,mascota:Mascota){
        self.cola_de_atencion.push_front(mascota);
    }

    pub fn atender_mascota(&mut self,diagnostico:String,tratamiento:String,proxima_visita:Fecha)->Option<Atencion>{
        if let Some(paciente)=self.cola_de_atencion.pop_front(){
            let atencion:Atencion=Atencion::new(paciente, diagnostico, tratamiento, proxima_visita);
            Some(atencion)
        }else{
            None
        }
    }   

    pub fn retirar_mascota(&mut self,mascota:Mascota)->bool{
        let mut encontrado=false;
        let mut indice:usize=0;
        while !encontrado && indice<self.cola_de_atencion.len(){
            if self.cola_de_atencion[indice].misma_mascota(&mascota) {
                self.cola_de_atencion.remove(indice);
                encontrado=true;
            }
            indice+=1;
        }
        encontrado
    }

    fn escribir_en_archivo(&self)->Result<(),Errores>{
        let mut archivo=File::create(&self.path_archivo).map_err(|_|Errores::ErrorDeCreacionDeArchivo)?;
        let atenciones_serializadas=serde_json::to_string(&self.atenciones).map_err(|_|Errores::ErrorDeSerializacionDeDatos)?;
        archivo.write_all(atenciones_serializadas.as_bytes()).map_err(|_|Errores::ErrorDeEscrituraDeDatos)?;
        Ok(())
    }

    pub fn registrar_atencion(&mut self,atencion:&Atencion){
        self.atenciones.push(atencion.clone());
        if let Err(err)=self.escribir_en_archivo(){
            self.atenciones.pop();
        }
    }

    fn verificar(&self, i:usize,nombre_mascota:String, nombre_dueño:String,telefono:u64)->bool{
        self.atenciones[i].paciente.nombre==nombre_mascota &&
        self.atenciones[i].paciente.dueño.nombre==nombre_dueño &&
        self.atenciones[i].paciente.dueño.telefono==telefono
    }

    pub fn  buscar_atencion(&self, nombre_mascota:String, nombre_dueño:String,telefono:u64)->Option<Atencion>{
        let mut indice:usize=0;
        let mut encontrado=false;
        let mut atencion:Option<Atencion>=None;
        while !encontrado && indice<self.atenciones.len(){
            if self.verificar(indice,nombre_mascota.clone(),nombre_dueño.clone(),telefono){
                atencion=self.atenciones.get(indice).cloned();
                encontrado=true;
                return atencion;
            }
            indice+=1;
        }
        None
    }


    fn buscar_atencion_indice(&self,atencion:&Atencion)->Option<usize>{
        let mut indice:usize=0;
        let mut encontrado=false;
        while !encontrado && indice < self.atenciones.len(){
            if self.atenciones[indice].misma_atencion(&atencion){
                return Some(indice);
            }else{
                indice+=1;
            }
        }
        None
    }

    pub fn modificar_diagnostico(&mut self, diagnostico:String,atencion:Atencion)->bool{
        let posicion=self.buscar_atencion_indice(&atencion);
        if posicion.is_none(){
            return false;
        }
        let back_up_atenciones=self.atenciones.clone();
        self.atenciones[posicion.unwrap()].diagnostico=diagnostico;
        if let Err(_)=self.escribir_en_archivo(){
            self.atenciones=back_up_atenciones;
            return false;
        }
        true
    }

    pub fn modificar_visita(&mut self,fecha:Fecha,atencion:Atencion)->bool{
        let posicion=self.buscar_atencion_indice(&atencion);
        if posicion.is_none(){
            return false;
        }

        let back_up_atenciones=self.atenciones.clone();
        self.atenciones[posicion.unwrap()].proxima_visita=fecha;
        if let Err(_)=self.escribir_en_archivo(){
            self.atenciones=back_up_atenciones;
            return false;
        }
        true
    }

    pub fn eliminar_atencion(&mut self, atencion:&Atencion)->bool{
        let pos=self.buscar_atencion_indice(&atencion);
        if pos.is_none(){
            return false;
        }
        let pos_original=pos.unwrap();
        self.atenciones.remove(pos_original);
        if let Err(_)=self.escribir_en_archivo(){
            self.atenciones.insert(pos_original, atencion.clone());
            return false;
        }
        true
    }

}

#[cfg(test)]
mod test{
    use crate::tp5::fecha::Fecha;
    use crate::tp5::ej3::{Animal, Atencion, Dueño, Mascota, Veterinaria};
    use std::fs;

    #[test]
    fn misma_mascota_test(){
        let dueño:Dueño=Dueño::new("Ana".to_string(),"333 333".to_string(),222222222);
        let perli:Mascota=Mascota::new("Perlita".to_string(),10,Animal::Perro,dueño.clone());
        assert!(perli.misma_mascota(&perli));
    }

    #[test]
    fn misma_mascota_diferente_mascota_test(){
        let dueño:Dueño=Dueño::new("Ana".to_string(),"333 333".to_string(),222222222);
        let perli:Mascota=Mascota::new("Perlita".to_string(),10,Animal::Perro,dueño.clone());
        let otto:Mascota=Mascota::new("Otto".to_string(),3,Animal::Perro,dueño);
        assert!(!perli.misma_mascota(&otto));
    }

    #[test]
    fn agregar_mascota_test(){
        let dueño:Dueño=Dueño::new("Ana".to_string(),"333 333".to_string(),222222222);
        let perli:Mascota=Mascota::new("Perlita".to_string(),10,Animal::Perro,dueño.clone());
        let mut veterinaria:Veterinaria=Veterinaria::new("Hola Patitas".to_string(),"777 777".to_string(),12345);
        assert_eq!(veterinaria.cola_de_atencion.len(),0);
        veterinaria.agregar_mascota(perli.clone());
        assert!(veterinaria.cola_de_atencion[veterinaria.cola_de_atencion.len()-1].misma_mascota(&perli));
    }

    #[test]
    fn agregar_mascota_con_prioridad_test(){
        let dueño1:Dueño=Dueño::new("Juan Perez".to_string(), "555 555".to_string(), 44412342);
        let dueño2:Dueño=Dueño::new("Ana".to_string(),"333 333".to_string(),222222222);
        let otto:Mascota=Mascota::new("Otto".to_string(),3,Animal::Perro,dueño1.clone());
        let cook:Mascota=Mascota::new("Lila".to_string(),12,Animal::Perro,dueño2);
        let perli:Mascota=Mascota::new("Perlita".to_string(),10,Animal::Perro,dueño1);
        let mut veterinaria:Veterinaria=Veterinaria::new("Hola Patitas".to_string(),"777 777".to_string(),12345);
        veterinaria.agregar_mascota(otto);
        veterinaria.agregar_mascota(cook);
        veterinaria.agregar_mascota_con_prioridad(perli.clone());
        assert!(veterinaria.cola_de_atencion[0].misma_mascota(&perli));
    }

    #[test]
    fn atender_mascota_test() {
        let dueño:Dueño=Dueño::new("Juan Perez".to_string(), "555 555".to_string(), 44412342);
        let perli:Mascota=Mascota::new("Perlita".to_string(),10,Animal::Perro,dueño.clone());
        let otto:Mascota=Mascota::new("Otto".to_string(),3,Animal::Perro,dueño.clone());
        let mut veterinaria:Veterinaria=Veterinaria::new("Hola Patitas".to_string(),"777 777".to_string(),12345);
        veterinaria.agregar_mascota(perli.clone());
        veterinaria.agregar_mascota(otto);
        let prox_visita=Fecha::new(03, 06, 2026);
        let atencion:Atencion=Atencion::new(perli,"Alergia".to_string(), "Medicacion de alergia".to_string(),prox_visita.clone());
        let resultado=veterinaria.atender_mascota("Alergia".to_string(), "Medicacion de alergia".to_string(), prox_visita.clone());
        assert!(resultado.is_some());
        assert!(resultado.unwrap().misma_atencion(&atencion));
    }

    #[test]
    fn atender_mascota_cola_vacia_test(){
        let mut veterinaria:Veterinaria=Veterinaria::new("Hola Patitas".to_string(),"777 777".to_string(),12345);
        let prox_visita=Fecha::new(03, 06, 2026);
        assert!(veterinaria.atender_mascota("Alergia".to_string(), "Medicacion de alergia".to_string(), prox_visita).is_none());
    }

    #[test]
    fn retirar_mascota_test(){
        let dueño:Dueño=Dueño::new("Juan Perez".to_string(), "555 555".to_string(), 44412342);
        let perli:Mascota=Mascota::new("Perlita".to_string(),10,Animal::Perro,dueño.clone());
        let otto:Mascota=Mascota::new("Otto".to_string(),3,Animal::Perro,dueño.clone());
        let mut veterinaria:Veterinaria=Veterinaria::new("Hola Patitas".to_string(),"777 777".to_string(),12345);
        veterinaria.agregar_mascota(perli.clone());
        veterinaria.agregar_mascota(otto);
    }

    #[test]
    fn retirar_mascota_ausente_test(){
        let dueño1:Dueño=Dueño::new("Juan Perez".to_string(), "555 555".to_string(), 44412342);
        let dueño2:Dueño=Dueño::new("Ana".to_string(),"333 333".to_string(),222222222);
        let otto:Mascota=Mascota::new("Otto".to_string(),3,Animal::Perro,dueño1.clone());
        let cook:Mascota=Mascota::new("Lila".to_string(),12,Animal::Perro,dueño2);
        let perli:Mascota=Mascota::new("Perlita".to_string(),10,Animal::Perro,dueño1);
        let mut veterinaria:Veterinaria=Veterinaria::new("Hola Patitas".to_string(),"777 777".to_string(),12345);
        veterinaria.agregar_mascota(cook);
        veterinaria.agregar_mascota(otto);
        assert!(!veterinaria.retirar_mascota(perli));
    }
    
    #[test]
    fn retirar_mascota_cola_vacia_test(){
        let mut veterinaria:Veterinaria=Veterinaria::new("Hola Patitas".to_string(),"777 777".to_string(),12345);
        let dueño1:Dueño=Dueño::new("Juan Perez".to_string(), "555 555".to_string(), 44412342);
        let otto:Mascota=Mascota::new("Otto".to_string(),3,Animal::Perro,dueño1.clone());
        assert!(!veterinaria.retirar_mascota(otto));
    }

    #[test]
    fn registrar_atencion_test(){
        let dueño:Dueño=Dueño::new("Juan Perez".to_string(), "555 555".to_string(), 44412342);
        let perli:Mascota=Mascota::new("Perlita".to_string(),10,Animal::Perro,dueño.clone());
        let mut veterinaria:Veterinaria=Veterinaria::new("Hola Patitas".to_string(),"777 777".to_string(),12345);
        veterinaria.agregar_mascota(perli);
        assert_eq!(veterinaria.atenciones.len(),0);
        let prox_visita=Fecha::new(03, 06, 2026);
        let atencion:Atencion=veterinaria.atender_mascota("Alergia".to_string(), "Medicacion de alergia".to_string(), prox_visita).unwrap();
        veterinaria.registrar_atencion(&atencion);
        assert_eq!(veterinaria.atenciones.len(),1);
    }

    #[test]
    fn buscar_atencion_test(){
        let dueño1:Dueño=Dueño::new("Juan Perez".to_string(), "555 555".to_string(), 44412342);
        let dueño2:Dueño=Dueño::new("Ana".to_string(),"333 333".to_string(),222222222);
        let otto:Mascota=Mascota::new("Otto".to_string(),3,Animal::Perro,dueño1.clone());
        let cook:Mascota=Mascota::new("Cook".to_string(),12,Animal::Perro,dueño2);
        let perli:Mascota=Mascota::new("Perlita".to_string(),10,Animal::Perro,dueño1);
        let mut veterinaria:Veterinaria=Veterinaria::new("Hola Patitas".to_string(),"777 777".to_string(),12345);
        veterinaria.agregar_mascota(cook);
        veterinaria.agregar_mascota(otto);
        veterinaria.agregar_mascota(perli);
        let atencion1:Atencion=veterinaria.atender_mascota("Alergia".to_string(), "Medicacion de alergia".to_string(), Fecha::new(03,06,2026)).unwrap();
        let atencion2:Atencion=veterinaria.atender_mascota("Gingivitis".to_string(), "Medicacion de Gingivitis".to_string(), Fecha::new(15,06,2026)).unwrap();
        let atencion3:Atencion=veterinaria.atender_mascota("Parasitos".to_string(), "Desparasitario".to_string(), Fecha::new(11,09,2026)).unwrap();
        veterinaria.registrar_atencion(&atencion1);
        veterinaria.registrar_atencion(&atencion2);
        veterinaria.registrar_atencion(&atencion3);
        let atencion_buscar=veterinaria.buscar_atencion("Otto".to_string(), "Juan Perez".to_string(), 44412342);

        assert!(atencion_buscar.is_some());
        assert!(atencion_buscar.unwrap().misma_atencion(&atencion2));
    }

    #[test]
    fn buscar_atencion_mascota_ausente_test(){
        let dueño1:Dueño=Dueño::new("Juan Perez".to_string(), "555 555".to_string(), 44412342);
        let dueño2:Dueño=Dueño::new("Ana".to_string(),"333 333".to_string(),222222222);
        let otto:Mascota=Mascota::new("Otto".to_string(),3,Animal::Perro,dueño1.clone());
        let cook:Mascota=Mascota::new("Cook".to_string(),12,Animal::Perro,dueño2);
        let perli:Mascota=Mascota::new("Perlita".to_string(),10,Animal::Perro,dueño1);
        let mut veterinaria:Veterinaria=Veterinaria::new("Hola Patitas".to_string(),"777 777".to_string(),12345);
        veterinaria.agregar_mascota(cook);
        veterinaria.agregar_mascota(otto);
        veterinaria.agregar_mascota(perli);
        let atencion1:Atencion=veterinaria.atender_mascota("Alergia".to_string(), "Medicacion de alergia".to_string(), Fecha::new(03,06,2026)).unwrap();
        let atencion2:Atencion=veterinaria.atender_mascota("Gingivitis".to_string(), "Medicacion de Gingivitis".to_string(), Fecha::new(15,06,2026)).unwrap();
        let atencion3:Atencion=veterinaria.atender_mascota("Parasitos".to_string(), "Desparasitario".to_string(), Fecha::new(11,09,2026)).unwrap();
       
        veterinaria.registrar_atencion(&atencion1);
        veterinaria.registrar_atencion(&atencion2);
        veterinaria.registrar_atencion(&atencion3);

        assert!(veterinaria.buscar_atencion("Garfield".to_string(), "Juan Perez".to_string(), 44412342).is_none());
    }

    #[test]
    fn buscar_atencion_sin_atenciones_test(){
        let veterinaria:Veterinaria=Veterinaria::new("Hola Patitas".to_string(),"777 777".to_string(),12345);
        assert!(veterinaria.buscar_atencion("Garfield".to_string(), "Juan Perez".to_string(), 44412342).is_none());
    }

    #[test]
    fn modificar_diagnostico_test(){
        let dueño:Dueño=Dueño::new("Ana".to_string(),"333 333".to_string(),222222222);
        let cook:Mascota=Mascota::new("Cook".to_string(),12,Animal::Perro,dueño);
        
        let mut veterinaria:Veterinaria=Veterinaria::new("Hola Patitas".to_string(),"777 777".to_string(),12345);
        veterinaria.agregar_mascota(cook);
        
        let atencion:Atencion=veterinaria.atender_mascota("Alergia".to_string(), "Medicacion de alergia".to_string(), Fecha::new(03,06,2026)).unwrap();
        
        veterinaria.registrar_atencion(&atencion);
        

        let resultado= veterinaria.modificar_diagnostico("Sarna".to_string(), atencion.clone());
        assert!(resultado);
        assert_eq!(veterinaria.atenciones[0].diagnostico,"Sarna".to_string());
    }

    #[test]
    fn modificar_diagnostico_atencion_inexistente_test(){
        let dueño:Dueño=Dueño::new("Ana".to_string(),"333 333".to_string(),222222222);
        let cook:Mascota=Mascota::new("Cook".to_string(),12,Animal::Perro,dueño);
        let dueño1:Dueño=Dueño::new("Juan Perez".to_string(), "555 555".to_string(), 44412342);
        let perli:Mascota=Mascota::new("Perlita".to_string(),10,Animal::Perro,dueño1);
        let mut veterinaria:Veterinaria=Veterinaria::new("Hola Patitas".to_string(),"777 777".to_string(),12345);
        veterinaria.agregar_mascota(cook);
        veterinaria.agregar_mascota(perli);
        
        let atencion:Atencion=veterinaria.atender_mascota("Alergia".to_string(), "Medicacion de alergia".to_string(), Fecha::new(03,06,2026)).unwrap();
        let atencion2:Atencion=veterinaria.atender_mascota("Parasitos".to_string(), "Desparasitario".to_string(), Fecha::new(11,09,2026)).unwrap();
        veterinaria.registrar_atencion(&atencion);

        assert!(!veterinaria.modificar_diagnostico("Sarna".to_string(), atencion2));

    }

    #[test]
    fn modificar_diagnostico_sin_atenciones_test(){
        let dueño:Dueño=Dueño::new("Ana".to_string(),"333 333".to_string(),222222222);
        let cook:Mascota=Mascota::new("Cook".to_string(),12,Animal::Perro,dueño);
        let mut veterinaria:Veterinaria=Veterinaria::new("Hola Patitas".to_string(),"777 777".to_string(),12345);
        veterinaria.agregar_mascota(cook);
        let atencion:Atencion=veterinaria.atender_mascota("Parasitos".to_string(), "Desparasitario".to_string(), Fecha::new(11,09,2026)).unwrap();
        assert!(!veterinaria.modificar_diagnostico("Sarna".to_string(), atencion));
    }


    #[test]
    fn modificar_visita_test(){
        let dueño:Dueño=Dueño::new("Ana".to_string(),"333 333".to_string(),222222222);
        let cook:Mascota=Mascota::new("Cook".to_string(),12,Animal::Perro,dueño);
        let mut veterinaria:Veterinaria=Veterinaria::new("Hola Patitas".to_string(),"777 777".to_string(),12345);
        veterinaria.agregar_mascota(cook);
        
        let atencion:Atencion=veterinaria.atender_mascota("Alergia".to_string(), "Medicacion de alergia".to_string(), Fecha::new(03,06,2026)).unwrap();
        
        veterinaria.registrar_atencion(&atencion);
        
        let nueva_fecha=Fecha::new(15,06,2026);
        let resultado=veterinaria.modificar_visita(nueva_fecha.clone(),atencion.clone());
        assert!(resultado);
        assert!(veterinaria.atenciones[0].proxima_visita.misma_fecha(&nueva_fecha));
    }

    #[test]
    fn modificar_visita_atencion_inexistente_test(){
        let dueño:Dueño=Dueño::new("Ana".to_string(),"333 333".to_string(),222222222);
        let cook:Mascota=Mascota::new("Cook".to_string(),12,Animal::Perro,dueño);
        let dueño1:Dueño=Dueño::new("Juan Perez".to_string(), "555 555".to_string(), 44412342);
        let perli:Mascota=Mascota::new("Perlita".to_string(),10,Animal::Perro,dueño1);

        let mut veterinaria:Veterinaria=Veterinaria::new("Hola Patitas".to_string(),"777 777".to_string(),12345);
        veterinaria.agregar_mascota(cook);
        veterinaria.agregar_mascota(perli);
        
        let atencion:Atencion=veterinaria.atender_mascota("Alergia".to_string(), "Medicacion de alergia".to_string(), Fecha::new(03,06,2026)).unwrap();
        let atencion2:Atencion=veterinaria.atender_mascota("Parasitos".to_string(), "Desparasitario".to_string(), Fecha::new(11,09,2026)).unwrap();
        veterinaria.registrar_atencion(&atencion);

        let nueva_fecha=Fecha::new(15,06,2026);
        assert!(!veterinaria.modificar_visita(nueva_fecha.clone(),atencion2.clone()));
        
    }

    #[test]
    fn modificar_visita_sin_atenciones_test(){
        let dueño:Dueño=Dueño::new("Ana".to_string(),"333 333".to_string(),222222222);
        let cook:Mascota=Mascota::new("Cook".to_string(),12,Animal::Perro,dueño);
        let mut veterinaria:Veterinaria=Veterinaria::new("Hola Patitas".to_string(),"777 777".to_string(),12345);
        veterinaria.agregar_mascota(cook);

        let atencion:Atencion=veterinaria.atender_mascota("Parasitos".to_string(), "Desparasitario".to_string(), Fecha::new(11,09,2026)).unwrap();
        let fecha=Fecha::new(15,06,2026);
        assert!(!veterinaria.modificar_visita(fecha, atencion));
    }

    #[test]
    fn eliminar_atencion_test(){
        let dueño1:Dueño=Dueño::new("Juan Perez".to_string(), "555 555".to_string(), 44412342);
        let dueño2:Dueño=Dueño::new("Ana".to_string(),"333 333".to_string(),222222222);
        let otto:Mascota=Mascota::new("Otto".to_string(),3,Animal::Perro,dueño1.clone());
        let cook:Mascota=Mascota::new("Cook".to_string(),12,Animal::Perro,dueño2);
        let perli:Mascota=Mascota::new("Perlita".to_string(),10,Animal::Perro,dueño1);
        let mut veterinaria:Veterinaria=Veterinaria::new("Hola Patitas".to_string(),"777 777".to_string(),12345);
        veterinaria.agregar_mascota(cook);
        veterinaria.agregar_mascota(otto);
        veterinaria.agregar_mascota(perli);
        let atencion1:Atencion=veterinaria.atender_mascota("Alergia".to_string(), "Medicacion de alergia".to_string(), Fecha::new(03,06,2026)).unwrap();
        let atencion2:Atencion=veterinaria.atender_mascota("Gingivitis".to_string(), "Medicacion de Gingivitis".to_string(), Fecha::new(15,06,2026)).unwrap();
        let atencion3:Atencion=veterinaria.atender_mascota("Parasitos".to_string(), "Desparasitario".to_string(), Fecha::new(11,09,2026)).unwrap();
       
        veterinaria.registrar_atencion(&atencion1);
        veterinaria.registrar_atencion(&atencion2);
        veterinaria.registrar_atencion(&atencion3);

        assert_eq!(veterinaria.atenciones.len(),3);
        veterinaria.eliminar_atencion(&atencion1);
        assert_eq!(veterinaria.atenciones.len(),2);    
        assert!(veterinaria.atenciones[0].misma_atencion(&atencion2));    
    }

    #[test]
    fn eliminar_atencion_inexistente_test(){
        let dueño:Dueño=Dueño::new("Ana".to_string(),"333 333".to_string(),222222222);
        let cook:Mascota=Mascota::new("Cook".to_string(),12,Animal::Perro,dueño);
        let mut veterinaria:Veterinaria=Veterinaria::new("Hola Patitas".to_string(),"777 777".to_string(),12345);
        veterinaria.agregar_mascota(cook);
        
        let atencion:Atencion=veterinaria.atender_mascota("Sarna".to_string(), "Medicacion de alergia".to_string(), Fecha::new(03,06,2026)).unwrap();
        veterinaria.registrar_atencion(&atencion);

        let dueño1:Dueño=Dueño::new("Juan Perez".to_string(), "555 555".to_string(), 44412342);
        let perli:Mascota=Mascota::new("Perlita".to_string(),10,Animal::Perro,dueño1);
        let atencion2=Atencion::new(perli, "Alergia".to_string(), "Cremas".to_string(), Fecha::new(15,06,2026));

        assert!(!veterinaria.eliminar_atencion(&atencion2));
    }

    #[test]
    fn eliminar_atencion_sin_atenciones_test(){
        let mut veterinaria:Veterinaria=Veterinaria::new("Hola Patitas".to_string(),"777 777".to_string(),12345);
        let dueño1:Dueño=Dueño::new("Juan Perez".to_string(), "555 555".to_string(), 44412342);
        let perli:Mascota=Mascota::new("Perlita".to_string(),10,Animal::Perro,dueño1);
        let atencion=Atencion::new(perli, "Alergia".to_string(), "Cremas".to_string(), Fecha::new(15,06,2026));
        
        let resultado=veterinaria.eliminar_atencion(&atencion);
        assert!(!resultado);
    }

    #[test]
    fn registrar_atencion_fallida_por_creacion_de_archivo_fallida_test(){
        let path="src/tp5/testeo_de_errores_registrar_atencion";
        let _ = std::fs::remove_file(path);
        let _ = std::fs::remove_dir(path);

        //El archivo existe pero es un directorio, no se pueden escribir atenciones en él
        fs::create_dir(path).unwrap();

        let dueño:Dueño=Dueño::new("Juan Perez".to_string(), "555 555".to_string(), 44412342);
        let perli:Mascota=Mascota::new("Perlita".to_string(),10,Animal::Perro,dueño.clone());
        let mut veterinaria:Veterinaria=Veterinaria::inner_new("Hola Patitas".to_string(),"777 777".to_string(),12345,path.to_string());
        veterinaria.agregar_mascota(perli);
        assert_eq!(veterinaria.atenciones.len(),0);
        let prox_visita=Fecha::new(03, 06, 2026);
        let atencion:Atencion=veterinaria.atender_mascota("Alergia".to_string(), "Medicacion de alergia".to_string(), prox_visita).unwrap();
        veterinaria.registrar_atencion(&atencion);
        
        assert_eq!(veterinaria.atenciones.len(),0);

        let _=std::fs::remove_dir(path);

    }

    #[test]
    fn modificar_diagnostico_fallidad_por_imposibilidad_de_crear_el_archivo_test(){
        let path="src/tp5/testeo_de_errores_modificar_diagnostico";
        let _ = std::fs::remove_file(path);
        let _ = std::fs::remove_dir(path);

        //El archivo existe pero es un directorio, no se pueden escribir atenciones en él
        fs::create_dir(path).unwrap();
        let dueño:Dueño=Dueño::new("Juan Perez".to_string(), "555 555".to_string(), 44412342);
        let perli:Mascota=Mascota::new("Perlita".to_string(),10,Animal::Perro,dueño.clone());
        let mut veterinaria:Veterinaria=Veterinaria::inner_new("Hola Patitas".to_string(),"777 777".to_string(),12345,path.to_string());
        veterinaria.agregar_mascota(perli);
        let prox_visita=Fecha::new(03, 06, 2026);
        let atencion:Atencion=veterinaria.atender_mascota("Alergia".to_string(), "Medicacion de alergia".to_string(), prox_visita).unwrap();
        
        //pusheo para que la falle en el modificar y no antes
        veterinaria.atenciones.push(atencion.clone());

        assert!(veterinaria.atenciones.len()==1);
        assert!(!veterinaria.modificar_diagnostico("Sarna".to_string(), atencion));
        assert_eq!(veterinaria.atenciones[0].diagnostico,"Alergia");

        let _=std::fs::remove_dir(path);

    }

    #[test]
    fn modificar_visita_fallidad_por_imposibilidad_de_crear_el_archivo_test(){
        let path="src/tp5/testeo_de_errores_modificar_visita";
        let _ = std::fs::remove_file(path);
        let _ = std::fs::remove_dir(path);

        //El archivo existe pero es un directorio, no se pueden escribir atenciones en él
        fs::create_dir(path).unwrap();
        let dueño:Dueño=Dueño::new("Juan Perez".to_string(), "555 555".to_string(), 44412342);
        let perli:Mascota=Mascota::new("Perlita".to_string(),10,Animal::Perro,dueño.clone());
        let mut veterinaria:Veterinaria=Veterinaria::inner_new("Hola Patitas".to_string(),"777 777".to_string(),12345,path.to_string());
        veterinaria.agregar_mascota(perli);
        let prox_visita=Fecha::new(03, 06, 2026);
        let atencion:Atencion=veterinaria.atender_mascota("Alergia".to_string(), "Medicacion de alergia".to_string(), prox_visita).unwrap();
        
        //pusheo para que la falle en el modificar y no antes
        veterinaria.atenciones.push(atencion.clone());

        assert!(veterinaria.atenciones.len()==1);
        assert!(!veterinaria.modificar_visita(Fecha::new(07,06,2026), atencion));
        assert_eq!(veterinaria.atenciones[0].diagnostico,"Alergia");

        let _=std::fs::remove_dir(path);
    }

    #[test]
    fn eliminar_atencion_fallida_por_imposibilidad_de_drear_archivo_test(){
        let path="src/tp5/testeo_de_errores_eliminar_atencion";
        let _ = std::fs::remove_file(path);
        let _ = std::fs::remove_dir(path);

        //El archivo existe pero es un directorio, no se pueden escribir atenciones en él
        fs::create_dir(path).unwrap();
        let dueño:Dueño=Dueño::new("Juan Perez".to_string(), "555 555".to_string(), 44412342);
        let perli:Mascota=Mascota::new("Perlita".to_string(),10,Animal::Perro,dueño.clone());
        let mut veterinaria:Veterinaria=Veterinaria::inner_new("Hola Patitas".to_string(),"777 777".to_string(),12345,path.to_string());
        veterinaria.agregar_mascota(perli);
        let prox_visita=Fecha::new(03, 06, 2026);
        let atencion:Atencion=veterinaria.atender_mascota("Alergia".to_string(), "Medicacion de alergia".to_string(), prox_visita).unwrap();
        
        //pusheo para que la falle en el modificar y no antes
        veterinaria.atenciones.push(atencion.clone());
        assert!(!veterinaria.eliminar_atencion(&atencion));
        assert!(veterinaria.atenciones.len()==1);

        let _=std::fs::remove_dir(path);
    }
}