#[allow(unused)]
#[derive(Debug,PartialEq,Clone)]
struct Persona <'a>{
    nombre:&'a str,
    apellido:&'a str,
    direccion:&'a str,
    ciudad:&'a str,
    salario:f64,
    edad:u8,
}

#[allow(unused)]
pub trait AnalizadorPersonas{
    fn salario_mayor(&self,salario:f64)->bool;
    fn mayor_edad_en_ciudad(&self,edad:u8,ciudad:&str)->bool;
    fn misma_ciudad(&self,ciudad:&str)->bool;
}

impl<'a> AnalizadorPersonas for Persona<'a>{

    fn salario_mayor(&self,salario:f64)->bool{
        self.salario > salario  
    }

    fn mayor_edad_en_ciudad(&self,edad:u8,ciudad:&str)->bool{
        self.edad>edad && self.ciudad==ciudad
    }

    fn misma_ciudad(&self,ciudad:&str)->bool{
        self.ciudad==ciudad
    }
}

#[allow(unused)]
impl <'a>Persona<'a>{
    fn new (nombre:&'a str, apellido:&'a str, direccion:&'a str, ciudad:&'a str, salario:f64, edad:u8)->Persona<'a>{
        Persona{
            nombre,
            apellido,
            direccion,
            ciudad,
            salario,
            edad,
        }
    }
}

/*a- Escriba una función que reciba un vector de personas y otro parámetro que indica un
salario. Debe retornar un listado de personas donde el salario es mayor al parámetro
recibido. */

#[allow(unused)]
fn personas_mayor_salario<'a>(vector:&'a[Persona<'a>],salario:f64)->Vec<&'a Persona<'a>>{
    let personas=vector.iter()
                                    .filter(|p| p.salario_mayor(salario))
                                    .collect();
    personas
}

/*b- Escriba una función que reciba un vector de personas, edad y el nombre de una ciudad.
Debe retornar las personas mayores al parámetro edad y que viven en la ciudad pasada por
parámetro. */
#[allow(unused)]
fn mayores_edad_en_ciudad<'a>(vector:&'a[Persona<'a>],edad:u8,ciudad:&str)->Vec<&'a Persona<'a>>{
    let personas=vector.iter()
                                    .filter(|p|p.mayor_edad_en_ciudad(edad, ciudad))
                                    .collect();
    personas
}

/*c- Escriba una función que reciba un vector de personas y un nombre de una ciudad. Debe
retornar true si todas las personas viven en la ciudad pasada por parámetro, false caso
contrario. */
#[allow(unused)]
fn todos_viven_en_ciudad<'a>(vector:&'a[Persona<'a>],ciudad:&str)->bool{
    let habitantes=vector.iter()
                            .all(|p|p.misma_ciudad(ciudad));
    habitantes
}

/*d- Escriba una función que reciba un vector de personas y un nombre de una ciudad. Debe
retornar true si al menos vive una persona en la ciudad pasada por parámetro, false caso
contrario. */
#[allow(unused)]
fn alguna_vive_en_ciudad<'a>(vector:&'a[Persona<'a>],ciudad:&str)->bool{
    let habitantes=vector.iter()
                            .any(|p|p.misma_ciudad(ciudad));
    habitantes
}

/*e- Escriba una función que reciba un arreglo de personas y una persona. Debe retornar true
si la persona existe en el arreglo, false caso contrario. */
#[allow(unused)]
fn persona_existente<'a>(vector:&'a[Persona<'a>],persona:Persona)->bool{
    //vector.contains(&persona)
    vector.iter().any(|p| *p==persona)
}

/*f- Escriba una función que reciba un arreglo de personas. Debe retornar un arreglo con las
edades de las personas. */
#[allow(unused)]
fn listar_edades<'a>(vector:&'a[Persona<'a>])->Vec<u8>{
    vector.iter().map(|p|p.edad).collect()
}

/*g- Escriba una función que reciba un arreglo de personas. Debe retornar la persona con el
menor salario y la persona con el mayor salario, en caso de que haya más de una persona
en cada categoría desempatar por la edad más grande. */
#[allow(unused)]
fn menor_y_mayor_salario<'a>(vector:&'a[Persona<'a>])->[&'a Persona<'a>;2]{
    let minimo=vector.iter()
                                    .min_by(|x,y|x.salario.total_cmp(&y.salario)
                                    .then_with(|| y.edad.cmp(&x.edad))).expect("No hay personas");
    let maximo=vector.iter()
                                    .max_by(|x,y|x.salario.total_cmp(&y.salario)
                                    .then_with(|| x.edad.cmp(&y.edad))).expect("No hay personas");
    [minimo,maximo]
}

#[cfg(test)]
mod test{
    use crate::tp4::ej2::{Persona, alguna_vive_en_ciudad, listar_edades, mayores_edad_en_ciudad, menor_y_mayor_salario, persona_existente, personas_mayor_salario, todos_viven_en_ciudad};

    //test a
    #[test]
    fn personas_mayor_salario_test(){
        let persona1:Persona=Persona::new("Ana","Alfaro","333 333", "La Plata",60000.0,24);
        let persona2:Persona=Persona::new("Juan","Perez","333 333", "La Plata",40000.0,24);
        let persona3:Persona=Persona::new("Jose","Gomez","333 333", "La Plata",50000.0,24);
    
        let vector=[persona1.clone(),persona2,persona3];

        let resultado=personas_mayor_salario(&vector, 50000.0);

        assert_eq!(resultado[0],&persona1);
        assert_eq!(resultado.len(),1);

    }

    #[test]
    fn personas_mayor_salario_todos_mayores_test(){
        let persona1:Persona=Persona::new("Ana","Alfaro","333 333", "La Plata",60000.0,24);
        let persona2:Persona=Persona::new("Juan","Perez","333 333", "La Plata",80000.0,24);
        let persona3:Persona=Persona::new("Jose","Gomez","333 333", "La Plata",70000.0,24);
    
        let vector=[persona1.clone(),persona2.clone(),persona3.clone()];

        let resultado=personas_mayor_salario(&vector, 50000.0);

        assert_eq!(resultado[0],&persona1);
        assert_eq!(resultado[1],&persona2);
        assert_eq!(resultado[2],&persona3);
        assert_eq!(resultado.len(),3);
    }

    #[test]
    fn personas_mayor_salario_ningun_mayor_test(){
        let persona1:Persona=Persona::new("Ana","Alfaro","333 333", "La Plata",20000.0,24);
        let persona2:Persona=Persona::new("Juan","Perez","333 333", "La Plata",30000.0,24);
        let persona3:Persona=Persona::new("Jose","Gomez","333 333", "La Plata",40000.0,24);
    
        let vector=[persona1.clone(),persona2.clone(),persona3.clone()];

        let resultado=personas_mayor_salario(&vector, 50000.0);
        assert_eq!(resultado.len(),0);
    }

    #[test]
    fn personas_mayor_salario_vector_vacio_test(){
        let vector=[];

        let resultado=personas_mayor_salario(&vector, 50000.0);
        assert_eq!(resultado.len(),0);
    }

    //test b
    #[test]
    fn mayores_en_ciudad_test(){
        let persona1:Persona=Persona::new("Ana","Alfaro","333 333", "La Plata",60000.0,24);
        let persona2:Persona=Persona::new("Juan","Perez","333 333", "La Plata",80000.0,34);
        let persona3:Persona=Persona::new("Jose","Gomez","333 333", "La Plata",70000.0,36);
        let persona4:Persona=Persona::new("Pepe","Perez","333 333", "Santa Cruz",80000.0,30);
        let persona5:Persona=Persona::new("Selena","Gomez","333 333", "La Plata",80000.0,30);
    
        let vector=[persona1.clone(),persona2.clone(),persona3.clone(),persona4.clone(),persona5.clone()];

        let resultado=mayores_edad_en_ciudad(&vector, 30, "La Plata");
        assert_eq!(resultado[0],&persona2);
        assert_eq!(resultado[1],&persona3);
        assert_eq!(resultado.len(),2);
    }

    #[test]
    fn mayores_en_ciudad_todos_cumplen_test(){
        let persona1:Persona=Persona::new("Ana","Alfaro","333 333", "La Plata",60000.0,42);
        let persona2:Persona=Persona::new("Juan","Perez","333 333", "La Plata",80000.0,34);
        let persona3:Persona=Persona::new("Jose","Gomez","333 333", "La Plata",70000.0,36);
       
        let vector=[persona1.clone(),persona2.clone(),persona3.clone()];

        let resultado=mayores_edad_en_ciudad(&vector, 30, "La Plata");
        assert_eq!(resultado[0],&persona1);
        assert_eq!(resultado[1],&persona2);
        assert_eq!(resultado[2],&persona3);
        assert_eq!(resultado.len(),3);
    }

    #[test]
    fn mayores_en_ciudad_ninguno_cumple_test(){
        let persona1:Persona=Persona::new("Ana","Alfaro","333 333", "La Plata",60000.0,24);
        let persona2:Persona=Persona::new("Juan","Perez","333 333", "Santa Cruz",80000.0,34);
        let persona3:Persona=Persona::new("Jose","Gomez","333 333", "Formosa",70000.0,36);
       
        let vector=[persona1.clone(),persona2.clone(),persona3.clone()];

        let resultado=mayores_edad_en_ciudad(&vector, 30, "La Plata");
        assert_eq!(resultado.len(),0);
    }

    #[test]
    fn mayores_en_ciudad_vector_vacio_test(){
        let persona1:Persona=Persona::new("Ana","Alfaro","333 333", "La Plata",60000.0,24);
        let persona2:Persona=Persona::new("Juan","Perez","333 333", "Santa Cruz",80000.0,34);
        let persona3:Persona=Persona::new("Jose","Gomez","333 333", "Formosa",70000.0,36);
       
        let vector=[persona1.clone(),persona2.clone(),persona3.clone()];

        let resultado=mayores_edad_en_ciudad(&vector, 30, "La Plata");
        assert_eq!(resultado.len(),0);
    }

    //test c
    #[test]
    fn todos_viven_en_ciudad_cumple_test(){
        let persona1:Persona=Persona::new("Ana","Alfaro","333 333", "La Plata",60000.0,42);
        let persona2:Persona=Persona::new("Juan","Perez","333 333", "La Plata",80000.0,34);
        let persona3:Persona=Persona::new("Jose","Gomez","333 333", "La Plata",70000.0,36);
       
        let vector=[persona1.clone(),persona2.clone(),persona3.clone()];

        let resultado=todos_viven_en_ciudad(&vector, "La Plata");
        assert!(resultado);
    }

    #[test]
    fn todos_viven_en_ciudad_ninguno_vive_test(){
        let persona1:Persona=Persona::new("Ana","Alfaro","333 333", "Formosa",60000.0,42);
        let persona2:Persona=Persona::new("Juan","Perez","333 333", "Santa Cruz",80000.0,34);
        let persona3:Persona=Persona::new("Jose","Gomez","333 333", "Entre Rios",70000.0,36);
       
        let vector=[persona1.clone(),persona2.clone(),persona3.clone()];

        let resultado=todos_viven_en_ciudad(&vector, "La Plata");
        assert!(!resultado);
    }

    #[test]
    fn todos_viven_en_ciudad_algunos_no_viven_test(){
        let persona1:Persona=Persona::new("Ana","Alfaro","333 333", "La Plata",60000.0,42);
        let persona2:Persona=Persona::new("Juan","Perez","333 333", "Santa Cruz",80000.0,34);
        let persona3:Persona=Persona::new("Jose","Gomez","333 333", "Entre Rios",70000.0,36);
       
        let vector=[persona1.clone(),persona2.clone(),persona3.clone()];

        let resultado=todos_viven_en_ciudad(&vector, "La Plata");
        assert!(!resultado);
    }

    #[test]
    fn todos_viven_en_ciudad_vector_vacio_test(){
        let vector=[];

        let resultado=todos_viven_en_ciudad(&vector, "La Plata");
        assert!(resultado);
    }

    //test d
    #[test]
    fn alguna_vive_en_ciudad_cumple_test(){
        let persona1:Persona=Persona::new("Ana","Alfaro","333 333", "La Plata",60000.0,42);
        let persona2:Persona=Persona::new("Juan","Perez","333 333", "La Plata",80000.0,34);
        let persona3:Persona=Persona::new("Jose","Gomez","333 333", "La Plata",70000.0,36);
       
        let vector=[persona1.clone(),persona2.clone(),persona3.clone()];

        let resultado=alguna_vive_en_ciudad(&vector, "La Plata");
        assert!(resultado);
    }

    #[test]
    fn alguna_vive_en_ciudad_ninguna_vive_test(){
        let persona1:Persona=Persona::new("Ana","Alfaro","333 333", "Formosa",60000.0,42);
        let persona2:Persona=Persona::new("Juan","Perez","333 333", "Santa Cruz",80000.0,34);
        let persona3:Persona=Persona::new("Jose","Gomez","333 333", "Entre Rios",70000.0,36);
       
        let vector=[persona1.clone(),persona2.clone(),persona3.clone()];

        let resultado=alguna_vive_en_ciudad(&vector, "La Plata");
        assert!(!resultado);
    }

    #[test]
    fn alguna_vive_en_ciudad_algunos_no_viven_test(){
        let persona1:Persona=Persona::new("Ana","Alfaro","333 333", "La Plata",60000.0,42);
        let persona2:Persona=Persona::new("Juan","Perez","333 333", "Santa Cruz",80000.0,34);
        let persona3:Persona=Persona::new("Jose","Gomez","333 333", "Entre Rios",70000.0,36);
       
        let vector=[persona1.clone(),persona2.clone(),persona3.clone()];

        let resultado=alguna_vive_en_ciudad(&vector, "La Plata");
        assert!(resultado);
    }

    #[test]
    fn alguna_vive_en_ciudad_vector_vacio_test(){
        let vector=[];

        let resultado=alguna_vive_en_ciudad(&vector, "La Plata");
        assert!(!resultado);
    }

    //test e
    #[test]
    fn persona_existente_test(){
        let persona1:Persona=Persona::new("Pepe","Perez","333 333", "Santa Cruz",80000.0,30);
        let persona2:Persona=Persona::new("Juan","Perez","333 333", "La Plata",80000.0,34);
        let persona3:Persona=Persona::new("Jose","Gomez","333 333", "La Plata",70000.0,36);
        let persona4:Persona=Persona::new("Ana","Alfaro","333 333", "La Plata",60000.0,24);
        let persona5:Persona=Persona::new("Selena","Gomez","333 333", "La Plata",80000.0,30);
        
        let vector=[persona1.clone(),persona2.clone(),persona3.clone(),persona4.clone(),persona5.clone()];
        assert!(persona_existente(&vector, persona4));
    }

    #[test]
    fn persona_existente_no_esta_la_persona_test(){
        let persona1:Persona=Persona::new("Pepe","Perez","333 333", "Santa Cruz",80000.0,30);
        let persona2:Persona=Persona::new("Juan","Perez","333 333", "La Plata",80000.0,34);
        let persona3:Persona=Persona::new("Jose","Gomez","333 333", "La Plata",70000.0,36);
        let persona4:Persona=Persona::new("Ana","Alfaro","333 333", "La Plata",60000.0,24);
        let persona5:Persona=Persona::new("Selena","Gomez","333 333", "La Plata",80000.0,30);
        
        let vector=[persona1.clone(),persona2.clone(),persona3.clone(),persona4.clone()];
        assert!(!persona_existente(&vector, persona5));
    }

    #[test]
    fn persona_existente_vector_vacio_test(){
        let persona5:Persona=Persona::new("Selena","Gomez","333 333", "La Plata",80000.0,30);
        let vector=[];
        assert!(!persona_existente(&vector, persona5));
    }

    //test f
    #[test]
    fn listar_edades_test(){
        let persona1:Persona=Persona::new("Ana","Alfaro","333 333", "La Plata",20000.0,24);
        let persona2:Persona=Persona::new("Juan","Perez","333 333", "La Plata",30000.0,32);
        let persona3:Persona=Persona::new("Jose","Gomez","333 333", "La Plata",40000.0,67);
    
        let vector=[persona1.clone(),persona2.clone(),persona3.clone()];
        let resultado=listar_edades(&vector);

        assert_eq!(vector.len(),resultado.len());
        assert_eq!(resultado,vec![24,32,67]);
    }

    #[test]
    fn listar_edades_vector_vacio_test(){
        let vec:Vec<Persona>=vec![];
        assert_eq!(vec,[]);
    }

    //test g
    #[test]
    fn menor_y_mayor_salario_test(){
        let persona1:Persona=Persona::new("Ana","Alfaro","333 333", "La Plata",30000.0,24);
        let persona2:Persona=Persona::new("Juan","Perez","333 333", "La Plata",50000.0,40);
        let persona3:Persona=Persona::new("Selena","Gomez","333 333", "La Plata",80000.0,50);
    
        let vector=[persona1.clone(),persona2.clone(),persona3.clone()];

        let resultado: [&Persona<'_>; 2]=menor_y_mayor_salario(&vector);

        assert_eq!(resultado.len(),2);
        assert_eq!(&vector[0],resultado[0]);
        assert_eq!(&vector[2],resultado[1]);
    }

    #[test]
    fn menor_y_mayor_salario_empate_minimo_test(){
        let persona1:Persona=Persona::new("Ana","Alfaro","333 333", "La Plata",30000.0,24);
        let persona2:Persona=Persona::new("Juan","Perez","333 333", "La Plata",30000.0,50);
        let persona3:Persona=Persona::new("Selena","Gomez","333 333", "La Plata",80000.0,60);
    
        let vector=[persona1.clone(),persona2.clone(),persona3.clone()];

        let resultado: [&Persona<'_>; 2]=menor_y_mayor_salario(&vector);

        assert_eq!(resultado.len(),2);
        assert_eq!(&vector[1],resultado[0]);
        assert_eq!(&vector[2],resultado[1]);
    }

    #[test]
    fn menor_y_mayor_salario_empate_maximo_test(){
        let persona1:Persona=Persona::new("Ana","Alfaro","333 333", "La Plata",30000.0,24);
        let persona2:Persona=Persona::new("Juan","Perez","333 333", "La Plata",80000.0,50);
        let persona3:Persona=Persona::new("Selena","Gomez","333 333", "La Plata",80000.0,60);
    
        let vector=[persona1.clone(),persona2.clone(),persona3.clone()];

        let resultado: [&Persona<'_>; 2]=menor_y_mayor_salario(&vector);

        assert_eq!(resultado.len(),2);
        assert_eq!(&vector[0],resultado[0]);
        assert_eq!(&vector[2],resultado[1]);
    }

    #[test]
    fn menor_y_mayor_salario_empate_ambos_extremos_test(){
        let persona1:Persona=Persona::new("Ana","Alfaro","333 333", "La Plata",30000.0,24);
        let persona2:Persona=Persona::new("Juan","Perez","333 333", "La Plata",30000.0,50);
        let persona3:Persona=Persona::new("Selena","Gomez","333 333", "La Plata",80000.0,50);
        let persona4:Persona=Persona::new("Pepe","Gomez","333 333", "La Plata",80000.0,60);
    
        let vector=[persona1.clone(),persona2.clone(),persona3.clone(),persona4.clone()];

        let resultado: [&Persona<'_>; 2]=menor_y_mayor_salario(&vector);

        assert_eq!(resultado.len(),2);
        assert_eq!(&vector[1],resultado[0]);
        assert_eq!(&vector[3],resultado[1]);
    }

    #[test]
    fn menor_y_mayor_salario_unico_elemento_test(){
        let persona1:Persona=Persona::new("Ana","Alfaro","333 333", "La Plata",30000.0,24);

        let vector=[persona1.clone()];

        let resultado: [&Persona<'_>; 2]=menor_y_mayor_salario(&vector);

        assert_eq!(resultado.len(),2);
        assert_eq!(&vector[0],resultado[0]);
        assert_eq!(&vector[0],resultado[1]);
    }

    #[test]
    #[should_panic(expected= "No hay personas")]
    fn menor_y_mayor_salario_vector_vacio_test(){
        let vector=vec![];
        let _=menor_y_mayor_salario(&vector);
    }
}