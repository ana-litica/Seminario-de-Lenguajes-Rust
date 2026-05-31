#[derive(Debug)]
#[allow(unused)]
struct Examen{
    nombre_materia:String,
    nota:u32,
}

#[derive(Debug)]
#[allow(unused)]
struct Estudiante{
    nombre:String,
    id:u32,
    examenes:Vec<Examen>,
}

#[allow(unused)]
impl Examen{

    pub fn new(nombre_materia:String,nota:u32)->Examen{
        Examen{
            nombre_materia,
            nota,
        }
    }
}

#[allow(unused)]
impl Estudiante{

    pub fn new(nombre:String,id:u32)->Estudiante{
        Estudiante{
            nombre,
            id,
            examenes:Vec::new(),
        }
    }

    pub fn agregar_examen(&mut self,examen:Examen){
        self.examenes.push(examen);
    }

    pub fn obtener_promedio(self)->f32{
        let mut promedio:f32=0.0;
        for e in &self.examenes{
            promedio+=e.nota as f32;
        }
        if self.examenes.len()>0{
            promedio=promedio/self.examenes.len() as f32;
        }
        promedio
    }

    pub fn obtener_calificacion_mas_alta(&self)->u32{
        let mut max=u32::MIN;
        for e in &self.examenes{
            if e.nota>max{
                max=e.nota
            }
        }
        max
    }

    pub fn obtener_calificacion_mas_baja(&self)->u32{
        let mut min=u32::MAX;
        for e in &self.examenes{
            if e.nota<min{
                min=e.nota
            }
        }
        min
    }
}

#[cfg(test)]
mod test{
    use crate::tp3::ej6::Estudiante;
    use crate::tp3::ej6::Examen;

    #[test]
    fn agregar_examen_test(){
        let mut estudiante=Estudiante::new("Ana".to_string(),23333);
        assert_eq!(estudiante.examenes.len(),0);
        let examen=Examen::new("Seminario".to_string(),5);
        estudiante.agregar_examen(examen);
        assert_eq!(estudiante.examenes.len(),1);
    }

    #[test]
    fn obtener_promedio_test(){
        let mut estudiante=Estudiante::new("Ana".to_string(),23333);
        let examen1=Examen::new("Seminario".to_string(),8);
        let examen2=Examen::new("OO2".to_string(),9);
        let examen3=Examen::new("Redes".to_string(),7);
        estudiante.agregar_examen(examen1);
        estudiante.agregar_examen(examen2);
        estudiante.agregar_examen(examen3);
        assert_eq!(estudiante.obtener_promedio(),8.0);
    }

    #[test]
    fn obtener_promedio_sin_examenes_test(){
        let estudiante=Estudiante::new("Ana".to_string(),23333);
        assert_eq!(estudiante.obtener_promedio(),0.0);
    }

    #[test]
    fn obtener_calificacion_mas_alta_test(){
        let mut estudiante=Estudiante::new("Ana".to_string(),23333);
        let examen1=Examen::new("Seminario".to_string(),8);
        let examen2=Examen::new("OO2".to_string(),9);
        let examen3=Examen::new("Redes".to_string(),7);
        estudiante.agregar_examen(examen1);
        estudiante.agregar_examen(examen2);
        estudiante.agregar_examen(examen3);
        assert_eq!(estudiante.obtener_calificacion_mas_alta(),9);
    }

    #[test]
    fn obtener_calificacion_mas_alta_sin_examenes_test(){
        let estudiante=Estudiante::new("Ana".to_string(),23333);
        assert_eq!(estudiante.obtener_calificacion_mas_alta(),u32::MIN);
    }
    
    #[test]
    fn obtener_calificacion_mas_baja_test(){
        let mut estudiante=Estudiante::new("Ana".to_string(),23333);
        let examen1=Examen::new("Seminario".to_string(),8);
        let examen2=Examen::new("OO2".to_string(),9);
        let examen3=Examen::new("Redes".to_string(),7);
        estudiante.agregar_examen(examen1);
        estudiante.agregar_examen(examen2);
        estudiante.agregar_examen(examen3);
        assert_eq!(estudiante.obtener_calificacion_mas_baja(),7);
    }

    #[test]
    fn obtener_calificacion_mas_baja_sin_examenes_test(){
        let estudiante=Estudiante::new("Ana".to_string(),23333);
        assert_eq!(estudiante.obtener_calificacion_mas_baja(),u32::MAX);
    }
    
}


