use std::{collections::HashMap, vec};
use crate::tp3::ej3::Fecha;

#[derive(Debug,Clone)]
#[allow(unused)]
struct Biblioteca{
    nombre:String,
    direccion:String,
    copias_libros:HashMap<String,u16>,
    prestamos:Vec<Prestamo>,
}

#[derive(Debug,Clone)]
#[allow(unused)]
struct Libro{
    isbn:String,
    titulo:String,
    autor:String,
    paginas:u32,
    genero:Genero,
}

#[derive(Debug,Clone)]
#[allow(unused)]
enum Genero{
    Novela,
    Infantil,
    Tecnico,
    Otros,
}

#[derive(Debug,Clone)]
#[allow(unused)]
struct Prestamo{
    libro:Libro,
    cliente:Cliente,
    vencimiento:Fecha,
    devolucion:Option<Fecha>,
    estado:String,
}

#[derive(Debug,Clone)]
#[allow(unused)]
struct Cliente{
    nombre:String,
    telefono:u64,
    correo_electronico:String,
}

#[allow(unused)]
impl Cliente{

    pub fn new(nombre:String,telefono:u64,correo_electronico:String)->Cliente{
        Cliente{ nombre, telefono, correo_electronico}
    }

    pub fn mismo_cliente(self,cliente:&Cliente)->bool{
        self.nombre==cliente.nombre && self.telefono==cliente.telefono && self.correo_electronico==cliente.correo_electronico
    }
}

#[allow(unused)]
impl Libro{
    pub fn new(isbn:String,titulo:String,autor:String,paginas:u32,genero:Genero)->Libro{
        Libro { isbn, titulo, autor, paginas, genero }
    }

    fn mismo_genero(self, libro:&Libro)->bool{
        match(self.genero.clone(),libro.genero.clone()){
            (Genero::Infantil,Genero::Infantil)|(Genero::Novela,Genero::Novela)|
            (Genero::Tecnico,Genero::Tecnico)|(Genero::Otros,Genero::Otros)=>true,
            _=>false,
        }
    }

    pub fn mismo_libro(self,libro:&Libro)->bool{
        self.isbn==libro.isbn && self.titulo==libro.titulo && self.autor==libro.autor &&
        self.paginas==libro.paginas && self.clone().mismo_genero(libro)
    }
}

#[allow(unused)]
impl Prestamo {
    pub fn new(libro:Libro,cliente:Cliente,vencimiento:Fecha,estado:String)->Prestamo{
        Prestamo{libro, cliente, vencimiento, devolucion:None, estado}
    }

    fn misma_devolucion(self,fecha:Option<Fecha>)->bool{
        (self.devolucion.is_some() && fecha.is_some() && self.devolucion.clone().unwrap().misma_fecha(&fecha.clone().unwrap()))||
        (self.devolucion.is_none() && fecha.is_none())
    }

    fn mismo_prestamo(self,prestamo:&Prestamo)->bool{
        self.libro.clone().mismo_libro(&prestamo.libro) && self.cliente.clone().mismo_cliente(&prestamo.cliente) &&
        self.vencimiento.misma_fecha(&prestamo.vencimiento) && self.clone().misma_devolucion(prestamo.devolucion.clone()) &&
        self.estado==prestamo.estado
    }

}

#[allow(unused)]
impl Biblioteca{
    pub fn new(nombre:String,direccion:String)->Biblioteca{
        Biblioteca { nombre, direccion, copias_libros: HashMap::new(), prestamos: Vec::new() }
    }

    pub fn agregar_libro(&mut self, libro:&Libro,cantidad:u16){
        self.copias_libros.insert(libro.isbn.clone(), cantidad);
    }

    pub fn obtener_cantidad_copias(&self,libro:&Libro)->Option<&u16>{
        self.copias_libros.get(&libro.isbn)
    }

    pub fn decrementar_disposicion(&mut self, libro:&Libro)->bool{
        let libr:Option<&mut u16>=self.copias_libros.get_mut(&libro.isbn);
        match libr{
            Some(libr)=> {
                if *libr==0{
                    return false;
                }
                *libr-=1;
            },
                
            None=>return false,
        }
        true
    }

    pub fn incrementar_disposicion(&mut self, libro:&Libro)->bool{
        let libr:Option<&mut u16>=self.copias_libros.get_mut(&libro.isbn);
        match libr{
            Some(libr)=> *libr+=1,
            None=>return false,
        }
        true
    }

    pub fn contar_prestamos_cliente(&self,cliente:Cliente)->u32{
        let mut cantidad:u32=0;
        for prestamo in self.prestamos.clone(){
            if prestamo.cliente.mismo_cliente(&cliente) && prestamo.estado=="en préstamo"{
                cantidad+=1;
            }
        }
        cantidad
    }

    pub fn realizar_prestamo(&mut self, cliente:&Cliente,libro:&Libro,mut fecha:Fecha,cantidad_dias:u32)->bool{
        let cant=self.obtener_cantidad_copias(&libro);
        if cant.is_none(){
            return false;
        }
        if self.contar_prestamos_cliente(cliente.clone())<5 && *cant.unwrap()>=1 {
            fecha.sumar_dias(cantidad_dias);
            let nuevo_prestamo=Prestamo::new(libro.clone(),cliente.clone(),fecha,"en préstamo".to_string());
            self.prestamos.push(nuevo_prestamo);
            self.decrementar_disposicion(&libro);
            return true;
        }else{
            false
        }
    }

    pub fn proximos_a_vencer(&self,dias:u32,mut fecha:Fecha)->Vec<Prestamo>{
        let mut por_vencer:Vec<Prestamo>   = vec![];
        let fecha_aux=fecha.clone();
        fecha.sumar_dias(dias);
        for prest in self.prestamos.clone(){
            if !prest.vencimiento.es_mayor(fecha.clone()) && prest.vencimiento.es_mayor(fecha_aux.clone()){
                por_vencer.push(prest);
            }
        }
        por_vencer
    }

    pub fn pretamos_vencidos(&self,fecha_actual:Fecha)->Vec<Prestamo>{
        let mut lista:Vec<Prestamo>=vec![];
        for prest in self.prestamos.clone(){
            if fecha_actual.es_mayor(prest.clone().vencimiento) && prest.estado=="en préstamo"{
                lista.push(prest);
            }
        }
        lista
    }

    pub fn buscar_prestamo(&self,libro:&Libro,cliente:&Cliente)->Option<&Prestamo>{
        let mut indice=0;
        while indice<self.prestamos.len(){
            if self.prestamos[indice].cliente.clone().mismo_cliente(&cliente) && self.prestamos[indice].libro.clone().mismo_libro(&libro){
                return Some(&self.prestamos[indice]);
            }
            indice+=1;
        }
        None
    }

    fn buscar_prestamo_indice(&self,libro:&Libro,cliente:&Cliente)->Option<usize>{
        let mut indice:usize=0;
        let mut encontrado=false;
        while !encontrado && indice < self.prestamos.len(){
            if self.prestamos[indice].cliente.clone().mismo_cliente(&cliente) 
            && self.prestamos[indice].libro.clone().mismo_libro(&libro) {
                return Some(indice);
            }else{
                indice+=1;
            }
        }
        None
    }

    pub fn devolver_libro(&mut self,libro:&Libro,cliente:&Cliente,fecha_actual:Fecha)->bool{
        let pos=self.buscar_prestamo_indice(&libro,&cliente);
        if pos.is_none(){
            return false;
        }
        let posicion=pos.unwrap();
        let mut devuelto=false;
        self.prestamos[posicion].estado="devuelto".to_string();
        self.prestamos[posicion].devolucion=Some(fecha_actual);
        true
    }

}

#[cfg(test)]
mod test{
    use crate::tp3::{ej3::Fecha,ej10::{Biblioteca,Libro,Genero,Prestamo,Cliente}};

    #[test]
    fn obtener_cantidad_copias_test(){
        let libro1=Libro::new("9788497592208".to_string(),"Don Quijote de la Mancha".to_string(),"Miguel de Cervantes".to_string(),1376,Genero::Novela);
        let libro2=Libro::new("9788437604947".to_string(),"El principito".to_string(),"Antoine de Saint-Exupéry".to_string(),96,Genero::Infantil);
        let libro3=Libro::new("9781718500440".to_string(),"The Rust Programming Language".to_string(),"Steve Klabnik y Carol Nichols".to_string(),560,Genero::Tecnico);
        let libro4=Libro::new("9780307474278".to_string(),"Sapiens".to_string(),"Yuval Noah Harari".to_string(),498,Genero::Otros);
    
        let mut biblio=Biblioteca::new("Biblioteca Central La Plata".to_string(),"Calle 50 N° 742, La Plata".to_string());
    
        biblio.agregar_libro(&libro1, 3);
        biblio.agregar_libro(&libro2, 0);
        biblio.agregar_libro(&libro3, 5);
        biblio.agregar_libro(&libro4, 7);

        let cantidad=*biblio.obtener_cantidad_copias(&libro3).unwrap();

        assert_eq!(cantidad,5);
    }

    #[test]
    fn obtener_cantidad_copias_libro_inexistente_test(){
        let libro1=Libro::new("9788497592208".to_string(),"Don Quijote de la Mancha".to_string(),"Miguel de Cervantes".to_string(),1376,Genero::Novela);
        let libro2=Libro::new("9788437604947".to_string(),"El principito".to_string(),"Antoine de Saint-Exupéry".to_string(),96,Genero::Infantil);
        let libro3=Libro::new("9781718500440".to_string(),"The Rust Programming Language".to_string(),"Steve Klabnik y Carol Nichols".to_string(),560,Genero::Tecnico);
        let libro4=Libro::new("9780307474278".to_string(),"Sapiens".to_string(),"Yuval Noah Harari".to_string(),498,Genero::Otros);
    
        let mut biblio=Biblioteca::new("Biblioteca Central La Plata".to_string(),"Calle 50 N° 742, La Plata".to_string());
    
        biblio.agregar_libro(&libro1, 3);
        biblio.agregar_libro(&libro2, 0);
        biblio.agregar_libro(&libro4, 7);

        assert!(biblio.obtener_cantidad_copias(&libro3).is_none());
    }

    #[test]
    fn obtener_cantidad_copias_sin_libros_test(){
        let biblio=Biblioteca::new("Biblioteca Central La Plata".to_string(),"Calle 50 N° 742, La Plata".to_string());
        let libro=Libro::new("9788437604947".to_string(),"El principito".to_string(),"Antoine de Saint-Exupéry".to_string(),96,Genero::Infantil);
        assert!(biblio.obtener_cantidad_copias(&libro).is_none());
    }

    #[test]
    fn decrementar_disposicion_test(){
        let libro1=Libro::new("9788497592208".to_string(),"Don Quijote de la Mancha".to_string(),"Miguel de Cervantes".to_string(),1376,Genero::Novela);
        let libro2=Libro::new("9788437604947".to_string(),"El principito".to_string(),"Antoine de Saint-Exupéry".to_string(),96,Genero::Infantil);
        let libro3=Libro::new("9781718500440".to_string(),"The Rust Programming Language".to_string(),"Steve Klabnik y Carol Nichols".to_string(),560,Genero::Tecnico);
        let libro4=Libro::new("9780307474278".to_string(),"Sapiens".to_string(),"Yuval Noah Harari".to_string(),498,Genero::Otros);
    
        let mut biblio=Biblioteca::new("Biblioteca Central La Plata".to_string(),"Calle 50 N° 742, La Plata".to_string());
    
        biblio.agregar_libro(&libro1, 3);
        biblio.agregar_libro(&libro2, 0);
        biblio.agregar_libro(&libro3, 5);
        biblio.agregar_libro(&libro4, 7);

        assert_eq!(*biblio.clone().obtener_cantidad_copias(&libro3).unwrap(),5);
        assert!(biblio.decrementar_disposicion(&libro3));
        assert_eq!(*biblio.obtener_cantidad_copias(&libro3).unwrap(),4);

    }

    #[test]
    fn decrementar_disposicion_libro_inexistente_test(){
        let libro1=Libro::new("9788497592208".to_string(),"Don Quijote de la Mancha".to_string(),"Miguel de Cervantes".to_string(),1376,Genero::Novela);
        let libro2=Libro::new("9788437604947".to_string(),"El principito".to_string(),"Antoine de Saint-Exupéry".to_string(),96,Genero::Infantil);
        let libro3=Libro::new("9781718500440".to_string(),"The Rust Programming Language".to_string(),"Steve Klabnik y Carol Nichols".to_string(),560,Genero::Tecnico);
        let libro4=Libro::new("9780307474278".to_string(),"Sapiens".to_string(),"Yuval Noah Harari".to_string(),498,Genero::Otros);
    
        let mut biblio=Biblioteca::new("Biblioteca Central La Plata".to_string(),"Calle 50 N° 742, La Plata".to_string());
    
        biblio.agregar_libro(&libro1, 3);
        biblio.agregar_libro(&libro2, 0);
        biblio.agregar_libro(&libro4, 7);

        assert!(!biblio.decrementar_disposicion(&libro3));
    }
    #[test]
    fn decrementar_disposicion_libro_sin_disposicion_test(){
        let libro1=Libro::new("9788497592208".to_string(),"Don Quijote de la Mancha".to_string(),"Miguel de Cervantes".to_string(),1376,Genero::Novela);
        let libro2=Libro::new("9788437604947".to_string(),"El principito".to_string(),"Antoine de Saint-Exupéry".to_string(),96,Genero::Infantil);
        let libro3=Libro::new("9781718500440".to_string(),"The Rust Programming Language".to_string(),"Steve Klabnik y Carol Nichols".to_string(),560,Genero::Tecnico);
    
        let mut biblio=Biblioteca::new("Biblioteca Central La Plata".to_string(),"Calle 50 N° 742, La Plata".to_string());
    
        biblio.agregar_libro(&libro1, 3);
        biblio.agregar_libro(&libro2, 0);
        biblio.agregar_libro(&libro3, 7);
        assert!(!biblio.decrementar_disposicion(&libro2));
    }

    #[test]
    fn decrementar_disposicion_sin_libros_test(){
        let mut biblio=Biblioteca::new("Biblioteca Central La Plata".to_string(),"Calle 50 N° 742, La Plata".to_string());
        let libro=Libro::new("9788437604947".to_string(),"El principito".to_string(),"Antoine de Saint-Exupéry".to_string(),96,Genero::Infantil);
        assert!(!biblio.decrementar_disposicion(&libro));

    }
    #[test]
    fn incrementar_disposicion_test(){
        let libro1=Libro::new("9788497592208".to_string(),"Don Quijote de la Mancha".to_string(),"Miguel de Cervantes".to_string(),1376,Genero::Novela);
        let libro2=Libro::new("9788437604947".to_string(),"El principito".to_string(),"Antoine de Saint-Exupéry".to_string(),96,Genero::Infantil);
        let libro3=Libro::new("9781718500440".to_string(),"The Rust Programming Language".to_string(),"Steve Klabnik y Carol Nichols".to_string(),560,Genero::Tecnico);
        let libro4=Libro::new("9780307474278".to_string(),"Sapiens".to_string(),"Yuval Noah Harari".to_string(),498,Genero::Otros);
    
        let mut biblio=Biblioteca::new("Biblioteca Central La Plata".to_string(),"Calle 50 N° 742, La Plata".to_string());
    
        biblio.agregar_libro(&libro1, 3);
        biblio.agregar_libro(&libro2, 0);
        biblio.agregar_libro(&libro3, 5);
        biblio.agregar_libro(&libro4, 7);

        assert_eq!(*biblio.clone().obtener_cantidad_copias(&libro3).unwrap(),5);
        assert!(biblio.incrementar_disposicion(&libro3));
        assert_eq!(*biblio.obtener_cantidad_copias(&libro3).unwrap(),6);

    }

    #[test]
    fn incrementar_disposicion_libro_inexistente_test(){
        let libro1=Libro::new("9788497592208".to_string(),"Don Quijote de la Mancha".to_string(),"Miguel de Cervantes".to_string(),1376,Genero::Novela);
        let libro2=Libro::new("9788437604947".to_string(),"El principito".to_string(),"Antoine de Saint-Exupéry".to_string(),96,Genero::Infantil);
        let libro3=Libro::new("9781718500440".to_string(),"The Rust Programming Language".to_string(),"Steve Klabnik y Carol Nichols".to_string(),560,Genero::Tecnico);
        let libro4=Libro::new("9780307474278".to_string(),"Sapiens".to_string(),"Yuval Noah Harari".to_string(),498,Genero::Otros);
    
        let mut biblio=Biblioteca::new("Biblioteca Central La Plata".to_string(),"Calle 50 N° 742, La Plata".to_string());
    
        biblio.agregar_libro(&libro1, 3);
        biblio.agregar_libro(&libro2, 0);
        biblio.agregar_libro(&libro4, 7);

        assert!(!biblio.incrementar_disposicion(&libro3));
    }

    #[test]
    fn incrementar_disposicion_sin_libros_test(){
        let mut biblio=Biblioteca::new("Biblioteca Central La Plata".to_string(),"Calle 50 N° 742, La Plata".to_string());
        let libro=Libro::new("9788437604947".to_string(),"El principito".to_string(),"Antoine de Saint-Exupéry".to_string(),96,Genero::Infantil);
        assert!(!biblio.incrementar_disposicion(&libro));
    }

    #[test]
    fn contar_prestamos_cliente_test(){
        let cliente1:Cliente=Cliente::new("Ana".to_string(),22222222,"hola@gmail.com".to_string());
        let cliente2:Cliente=Cliente::new("Martin Lopez".to_string(), 33333333, "hello@outlook.com".to_string());

        let libro1=Libro::new("9788497592208".to_string(),"Don Quijote de la Mancha".to_string(),"Miguel de Cervantes".to_string(),1376,Genero::Novela);
        let libro2=Libro::new("9788437604947".to_string(),"El principito".to_string(),"Antoine de Saint-Exupéry".to_string(),96,Genero::Infantil);
        let libro3=Libro::new("9781718500440".to_string(),"The Rust Programming Language".to_string(),"Steve Klabnik y Carol Nichols".to_string(),560,Genero::Tecnico);
       
       let mut biblio=Biblioteca::new("Biblioteca Central La Plata".to_string(),"Calle 50 N° 742, La Plata".to_string());
        biblio.agregar_libro(&libro1, 3);
        biblio.agregar_libro(&libro2, 5);
        biblio.agregar_libro(&libro3, 7);

        let fecha=Fecha::new(30, 05,2026);
        biblio.realizar_prestamo(&cliente1, &libro1, fecha.clone(), 10);
        biblio.realizar_prestamo(&cliente1, &libro2, fecha.clone(), 10);
        biblio.realizar_prestamo(&cliente2, &libro2, fecha, 10);

        assert_eq!(biblio.contar_prestamos_cliente(cliente1),2);

    }

    #[test]
    fn contar_prestamos_cliente_sin_prestamos_test(){
        let cliente1:Cliente=Cliente::new("Ana".to_string(),22222222,"hola@gmail.com".to_string());
        let cliente2:Cliente=Cliente::new("Martin Lopez".to_string(), 33333333, "hello@outlook.com".to_string());

        let libro1=Libro::new("9788497592208".to_string(),"Don Quijote de la Mancha".to_string(),"Miguel de Cervantes".to_string(),1376,Genero::Novela);
        let libro2=Libro::new("9788437604947".to_string(),"El principito".to_string(),"Antoine de Saint-Exupéry".to_string(),96,Genero::Infantil);
        let libro3=Libro::new("9781718500440".to_string(),"The Rust Programming Language".to_string(),"Steve Klabnik y Carol Nichols".to_string(),560,Genero::Tecnico);
       
       let mut biblio=Biblioteca::new("Biblioteca Central La Plata".to_string(),"Calle 50 N° 742, La Plata".to_string());
       biblio.agregar_libro(&libro1, 3);
        biblio.agregar_libro(&libro2, 5);
        biblio.agregar_libro(&libro3, 7);

        let fecha=Fecha::new(30, 05,2026);
        
        biblio.realizar_prestamo(&cliente2,& libro2, fecha, 10);

        assert_eq!(biblio.contar_prestamos_cliente(cliente1),0);

    }

    #[test]
    fn realizar_prestamo_test(){
        let cliente:Cliente=Cliente::new("Ana".to_string(),22222222,"hola@gmail.com".to_string());
        let libro=Libro::new("9788437604947".to_string(),"El principito".to_string(),"Antoine de Saint-Exupéry".to_string(),96,Genero::Infantil);
        let mut biblio=Biblioteca::new("Biblioteca Central La Plata".to_string(),"Calle 50 N° 742, La Plata".to_string());
        biblio.agregar_libro(&libro, 3);

        assert!(biblio.realizar_prestamo(&cliente, &libro,Fecha::new(30,05,2026), 10));
    }

    #[test]
    fn realizar_prestamo_cliente_con_mayor_a_cinco_prestamos_test(){
        let cliente:Cliente=Cliente::new("Ana".to_string(),22222222,"hola@gmail.com".to_string());
        let libro1=Libro::new("9788497592208".to_string(),"Don Quijote de la Mancha".to_string(),"Miguel de Cervantes".to_string(),1376,Genero::Novela);
        let libro2=Libro::new("9788437604947".to_string(),"El principito".to_string(),"Antoine de Saint-Exupéry".to_string(),96,Genero::Infantil);
        let libro3=Libro::new("9781718500440".to_string(),"The Rust Programming Language".to_string(),"Steve Klabnik y Carol Nichols".to_string(),560,Genero::Tecnico);
        let libro4=Libro::new("9780307474278".to_string(),"Sapiens".to_string(),"Yuval Noah Harari".to_string(),498,Genero::Otros);
        let libro5=Libro::new("9788498381498".to_string(),"Harry Potter y la piedra filosofal".to_string(),"J. K. Rowling".to_string(),320,Genero::Novela);
        let libro6=Libro::new("9789500727631".to_string(),"El Aleph".to_string(),"Jorge Luis Borges".to_string(),160,Genero::Novela);
    
        let mut biblio=Biblioteca::new("Biblioteca Central La Plata".to_string(),"Calle 50 N° 742, La Plata".to_string());

        biblio.agregar_libro(&libro1, 3);
        biblio.agregar_libro(&libro2, 5);
        biblio.agregar_libro(&libro3, 6);
        biblio.agregar_libro(&libro4, 8);
        biblio.agregar_libro(&libro5, 2);
        biblio.agregar_libro(&libro6, 5);

        biblio.realizar_prestamo(&cliente, &libro1, Fecha::new(25,05,2026), 10);
        biblio.realizar_prestamo(&cliente, &libro2, Fecha::new(26,05,2026), 10);
        biblio.realizar_prestamo(&cliente, &libro3, Fecha::new(26,05,2026), 10);
        biblio.realizar_prestamo(&cliente, &libro4, Fecha::new(27,05,2026), 10);
        biblio.realizar_prestamo(&cliente, &libro5, Fecha::new(28,05,2026), 10);

        assert!(!biblio.realizar_prestamo(&cliente, &libro6, Fecha::new(30,05,2025), 12));
    }

    #[test]
    fn realizar_prestamo_sin_copias_disponibles_test(){
        let cliente:Cliente=Cliente::new("Ana".to_string(),22222222,"hola@gmail.com".to_string());
        let libro=Libro::new("9788497592208".to_string(),"Don Quijote de la Mancha".to_string(),"Miguel de Cervantes".to_string(),1376,Genero::Novela);
        let mut biblio=Biblioteca::new("Biblioteca Central La Plata".to_string(),"Calle 50 N° 742, La Plata".to_string());
        biblio.agregar_libro(&libro, 0);
        assert!(!biblio.realizar_prestamo(&cliente, &libro,Fecha::new(30,05,2025) , 10));
    }

    #[test]
    fn realizar_prestamo_libro_inexistente_test(){
        let cliente:Cliente=Cliente::new("Ana".to_string(),22222222,"hola@gmail.com".to_string());
        let libro=Libro::new("9788497592208".to_string(),"Don Quijote de la Mancha".to_string(),"Miguel de Cervantes".to_string(),1376,Genero::Novela);
        let mut biblio=Biblioteca::new("Biblioteca Central La Plata".to_string(),"Calle 50 N° 742, La Plata".to_string());
        let fecha=Fecha::new(30, 05,2026);
        assert!(!biblio.realizar_prestamo(&cliente, &libro, fecha, 5));
    }

    #[test]
    fn proximos_a_vencer_test(){
        let cliente:Cliente=Cliente::new("Ana".to_string(),22222222,"hola@gmail.com".to_string());
        let cliente2:Cliente=Cliente::new("Jose".to_string(),333333333,"holis@outlook.com".to_string());
        let libro1=Libro::new("9788497592208".to_string(),"Don Quijote de la Mancha".to_string(),"Miguel de Cervantes".to_string(),1376,Genero::Novela);
        let libro2=Libro::new("9788437604947".to_string(),"El principito".to_string(),"Antoine de Saint-Exupéry".to_string(),96,Genero::Infantil);
        let libro3=Libro::new("9781718500440".to_string(),"The Rust Programming Language".to_string(),"Steve Klabnik y Carol Nichols".to_string(),560,Genero::Tecnico);
        
        let mut biblio=Biblioteca::new("Biblioteca Central La Plata".to_string(),"Calle 50 N° 742, La Plata".to_string());
        biblio.agregar_libro(&libro1, 3);
        biblio.agregar_libro(&libro2, 5);
        biblio.agregar_libro(&libro3, 6);

        biblio.realizar_prestamo(&cliente, &libro1,Fecha::new(31,05,2026), 4);
        biblio.realizar_prestamo(&cliente, &libro2,Fecha::new(31,05,2026), 20);
        biblio.realizar_prestamo(&cliente2, &libro3,Fecha::new(31,05,2026), 5);
        
        assert_eq!(biblio.proximos_a_vencer(7, Fecha::new(31,05, 2026)).len(),2);
    }

    #[test]
    fn proximos_a_vencer_todos_sin_vencer_test(){
        let cliente:Cliente=Cliente::new("Ana".to_string(),22222222,"hola@gmail.com".to_string());
        let cliente2:Cliente=Cliente::new("Jose".to_string(),333333333,"holis@outlook.com".to_string());
        let libro1=Libro::new("9788497592208".to_string(),"Don Quijote de la Mancha".to_string(),"Miguel de Cervantes".to_string(),1376,Genero::Novela);
        let libro2=Libro::new("9788437604947".to_string(),"El principito".to_string(),"Antoine de Saint-Exupéry".to_string(),96,Genero::Infantil);
        let libro3=Libro::new("9781718500440".to_string(),"The Rust Programming Language".to_string(),"Steve Klabnik y Carol Nichols".to_string(),560,Genero::Tecnico);
        let libro4=Libro::new("9780307474278".to_string(),"Sapiens".to_string(),"Yuval Noah Harari".to_string(),498,Genero::Otros);
        
        let mut biblio=Biblioteca::new("Biblioteca Central La Plata".to_string(),"Calle 50 N° 742, La Plata".to_string());
        biblio.agregar_libro(&libro1, 3);
        biblio.agregar_libro(&libro2, 5);
        biblio.agregar_libro(&libro3, 6);
        biblio.agregar_libro(&libro4, 8);

        biblio.realizar_prestamo(&cliente, &libro1,Fecha::new(31,05,2026), 15);
        biblio.realizar_prestamo(&cliente, &libro2,Fecha::new(31,05,2026), 25);
        biblio.realizar_prestamo(&cliente2, &libro3,Fecha::new(31,05,2026), 30);
        biblio.realizar_prestamo(&cliente2, &libro4,Fecha::new(31,05,2026), 20);
   
        assert_eq!(biblio.proximos_a_vencer(5,Fecha::new(31, 05,2026)).len(),0);
    }

    #[test]
    fn proximos_a_vencer_todos_por_vencer_test(){
        let cliente:Cliente=Cliente::new("Ana".to_string(),22222222,"hola@gmail.com".to_string());
        let cliente2:Cliente=Cliente::new("Jose".to_string(),333333333,"holis@outlook.com".to_string());
        let libro1=Libro::new("9788497592208".to_string(),"Don Quijote de la Mancha".to_string(),"Miguel de Cervantes".to_string(),1376,Genero::Novela);
        let libro2=Libro::new("9788437604947".to_string(),"El principito".to_string(),"Antoine de Saint-Exupéry".to_string(),96,Genero::Infantil);
        let libro3=Libro::new("9781718500440".to_string(),"The Rust Programming Language".to_string(),"Steve Klabnik y Carol Nichols".to_string(),560,Genero::Tecnico);
        let libro4=Libro::new("9780307474278".to_string(),"Sapiens".to_string(),"Yuval Noah Harari".to_string(),498,Genero::Otros);
        
        let mut biblio=Biblioteca::new("Biblioteca Central La Plata".to_string(),"Calle 50 N° 742, La Plata".to_string());
        biblio.agregar_libro(&libro1, 3);
        biblio.agregar_libro(&libro2, 5);
        biblio.agregar_libro(&libro3, 6);
        biblio.agregar_libro(&libro4, 8);

        biblio.realizar_prestamo(&cliente, &libro1,Fecha::new(31,05,2026), 2);
        biblio.realizar_prestamo(&cliente, &libro2,Fecha::new(31,05,2026), 5);
        biblio.realizar_prestamo(&cliente2,& libro3,Fecha::new(31,05,2026), 3);
        biblio.realizar_prestamo(&cliente2, &libro4,Fecha::new(31,05,2026), 2);
   
        assert_eq!(biblio.proximos_a_vencer(5,Fecha::new(31, 05,2026)).len(),4);
    }

    #[test]
    fn proximos_a_vencer_sin_prestamos_test(){
        let biblio=Biblioteca::new("Biblioteca Central La Plata".to_string(),"Calle 50 N° 742, La Plata".to_string());
        assert_eq!(biblio.proximos_a_vencer(5,Fecha::new(31, 05,2026)).len(),0);
    }

    #[test]
    fn proximos_a_vencer_todos_vencidos_test(){
        let cliente:Cliente=Cliente::new("Ana".to_string(),22222222,"hola@gmail.com".to_string());
        let cliente2:Cliente=Cliente::new("Jose".to_string(),333333333,"holis@outlook.com".to_string());
        let libro1=Libro::new("9788497592208".to_string(),"Don Quijote de la Mancha".to_string(),"Miguel de Cervantes".to_string(),1376,Genero::Novela);
        let libro2=Libro::new("9788437604947".to_string(),"El principito".to_string(),"Antoine de Saint-Exupéry".to_string(),96,Genero::Infantil);
        let libro3=Libro::new("9781718500440".to_string(),"The Rust Programming Language".to_string(),"Steve Klabnik y Carol Nichols".to_string(),560,Genero::Tecnico);
        let libro4=Libro::new("9780307474278".to_string(),"Sapiens".to_string(),"Yuval Noah Harari".to_string(),498,Genero::Otros);
        
        let mut biblio=Biblioteca::new("Biblioteca Central La Plata".to_string(),"Calle 50 N° 742, La Plata".to_string());
        biblio.agregar_libro(&libro1, 3);
        biblio.agregar_libro(&libro2, 5);
        biblio.agregar_libro(&libro3, 6);
        biblio.agregar_libro(&libro4, 8);

        biblio.realizar_prestamo(&cliente, &libro1,Fecha::new(12,05,2026), 2);
        biblio.realizar_prestamo(&cliente, &libro2,Fecha::new(03,05,2026), 5);
        biblio.realizar_prestamo(&cliente2,& libro3,Fecha::new(10,05,2026), 3);
        biblio.realizar_prestamo(&cliente2,& libro4,Fecha::new(3,05,2026), 2);
   
        assert_eq!(biblio.proximos_a_vencer(5,Fecha::new(31, 05,2026)).len(),0);
    }

    #[test]
    fn prestamos_vencidos_test(){
        let cliente:Cliente=Cliente::new("Ana".to_string(),22222222,"hola@gmail.com".to_string());
        let cliente2:Cliente=Cliente::new("Jose".to_string(),333333333,"holis@outlook.com".to_string());
        let libro1=Libro::new("9788497592208".to_string(),"Don Quijote de la Mancha".to_string(),"Miguel de Cervantes".to_string(),1376,Genero::Novela);
        let libro2=Libro::new("9788437604947".to_string(),"El principito".to_string(),"Antoine de Saint-Exupéry".to_string(),96,Genero::Infantil);
        let libro3=Libro::new("9781718500440".to_string(),"The Rust Programming Language".to_string(),"Steve Klabnik y Carol Nichols".to_string(),560,Genero::Tecnico);
        let libro4=Libro::new("9780307474278".to_string(),"Sapiens".to_string(),"Yuval Noah Harari".to_string(),498,Genero::Otros);
        
        let mut biblio=Biblioteca::new("Biblioteca Central La Plata".to_string(),"Calle 50 N° 742, La Plata".to_string());
        biblio.agregar_libro(&libro1, 3);
        biblio.agregar_libro(&libro2, 5);
        biblio.agregar_libro(&libro3, 6);
        biblio.agregar_libro(&libro4, 8);

        biblio.realizar_prestamo(&cliente, &libro1,Fecha::new(12,05,2026), 2); //vencido y no devuelto
        biblio.realizar_prestamo(&cliente, &libro2,Fecha::new(15,05,2026), 5); //devuelto
        biblio.realizar_prestamo(&cliente2, &libro3,Fecha::new(10,05,2026), 3); //vencido y no devuelto
        biblio.realizar_prestamo(&cliente2, &libro4,Fecha::new(29,05,2026), 20);//no vencido

        let fecha=Fecha::new(18,05,2026);
        biblio.devolver_libro(&libro2, &cliente, fecha);

        assert_eq!(biblio.pretamos_vencidos(Fecha::new(31,05,2026)).len(),2);
    }

    #[test]
    fn prestamos_vencidos_sin_prestamos_test(){
        let biblio=Biblioteca::new("Biblioteca Central La Plata".to_string(),"Calle 50 N° 742, La Plata".to_string());
        assert_eq!(biblio.pretamos_vencidos(Fecha::new(31, 05,2026)).len(),0);
    }

     #[test]
    fn prestamos_vencidos_todos_vencidos_en_prestamo_test(){
        let cliente:Cliente=Cliente::new("Ana".to_string(),22222222,"hola@gmail.com".to_string());
        let cliente2:Cliente=Cliente::new("Jose".to_string(),333333333,"holis@outlook.com".to_string());
        let libro1=Libro::new("9788497592208".to_string(),"Don Quijote de la Mancha".to_string(),"Miguel de Cervantes".to_string(),1376,Genero::Novela);
        let libro2=Libro::new("9788437604947".to_string(),"El principito".to_string(),"Antoine de Saint-Exupéry".to_string(),96,Genero::Infantil);
        let libro3=Libro::new("9781718500440".to_string(),"The Rust Programming Language".to_string(),"Steve Klabnik y Carol Nichols".to_string(),560,Genero::Tecnico);
        let libro4=Libro::new("9780307474278".to_string(),"Sapiens".to_string(),"Yuval Noah Harari".to_string(),498,Genero::Otros);
        
        let mut biblio=Biblioteca::new("Biblioteca Central La Plata".to_string(),"Calle 50 N° 742, La Plata".to_string());
        biblio.agregar_libro(&libro1, 3);
        biblio.agregar_libro(&libro2, 5);
        biblio.agregar_libro(&libro3, 6);
        biblio.agregar_libro(&libro4, 8);

        biblio.realizar_prestamo(&cliente, &libro1,Fecha::new(12,05,2026), 2);
        biblio.realizar_prestamo(&cliente, &libro2,Fecha::new(03,05,2026), 5);
        biblio.realizar_prestamo(&cliente2, &libro3,Fecha::new(10,05,2026), 3);
        biblio.realizar_prestamo(&cliente2, &libro4,Fecha::new(3,05,2026), 2);
   
        assert_eq!(biblio.pretamos_vencidos(Fecha::new(31, 05,2026)).len(),4);
    }

    #[test]
    fn prestamos_vencidos_todos_devueltos_test(){
        let cliente:Cliente=Cliente::new("Ana".to_string(),22222222,"hola@gmail.com".to_string());
        let libro1=Libro::new("9788497592208".to_string(),"Don Quijote de la Mancha".to_string(),"Miguel de Cervantes".to_string(),1376,Genero::Novela);
        let libro2=Libro::new("9788437604947".to_string(),"El principito".to_string(),"Antoine de Saint-Exupéry".to_string(),96,Genero::Infantil);
         let mut biblio=Biblioteca::new("Biblioteca Central La Plata".to_string(),"Calle 50 N° 742, La Plata".to_string());
        biblio.agregar_libro(&libro1, 3);
        biblio.agregar_libro(&libro2, 5);

        biblio.realizar_prestamo(&cliente, &libro1,Fecha::new(12,05,2026), 2);
        biblio.realizar_prestamo(&cliente, &libro2,Fecha::new(03,05,2026), 5);

        let fecha=Fecha::new(18,05,2026);
        biblio.devolver_libro(&libro1, &cliente, fecha.clone());
        biblio.devolver_libro(&libro2, &cliente, fecha);

        assert_eq!(biblio.pretamos_vencidos(Fecha::new(31,05,2026)).len(),0);

    }

    #[test]
    fn buscar_prestamo_test(){
        let cliente:Cliente=Cliente::new("Ana".to_string(),22222222,"hola@gmail.com".to_string());
        let cliente2:Cliente=Cliente::new("Jose".to_string(),333333333,"holis@outlook.com".to_string());
        let libro1=Libro::new("9788497592208".to_string(),"Don Quijote de la Mancha".to_string(),"Miguel de Cervantes".to_string(),1376,Genero::Novela);
        let libro2=Libro::new("9788437604947".to_string(),"El principito".to_string(),"Antoine de Saint-Exupéry".to_string(),96,Genero::Infantil);
        let libro3=Libro::new("9781718500440".to_string(),"The Rust Programming Language".to_string(),"Steve Klabnik y Carol Nichols".to_string(),560,Genero::Tecnico);
        let libro4=Libro::new("9780307474278".to_string(),"Sapiens".to_string(),"Yuval Noah Harari".to_string(),498,Genero::Otros);
       
       let mut biblio=Biblioteca::new("Biblioteca Central La Plata".to_string(),"Calle 50 N° 742, La Plata".to_string());
        biblio.agregar_libro(&libro1, 3);
        biblio.agregar_libro(&libro2, 5);
        biblio.agregar_libro(&libro3, 6);
        biblio.agregar_libro(&libro4, 8);
    
        biblio.realizar_prestamo(&cliente, &libro1,Fecha::new(12,05,2026), 2);
        biblio.realizar_prestamo(&cliente, &libro2,Fecha::new(03,05,2026), 5);
        biblio.realizar_prestamo(&cliente2, &libro3,Fecha::new(10,05,2026), 3);
        biblio.realizar_prestamo(&cliente2, &libro4,Fecha::new(3,05,2026), 2);

        let prestamo=Prestamo::new(libro2.clone(),cliente.clone(),Fecha::new(08,05,2026),"en préstamo".to_string());
        
        assert!(biblio.buscar_prestamo(&libro2, &cliente).unwrap().clone().mismo_prestamo(&prestamo));
    }

    #[test]
    fn buscar_prestamo_inxistente_test(){
        let cliente:Cliente=Cliente::new("Ana".to_string(),22222222,"hola@gmail.com".to_string());
        let cliente2:Cliente=Cliente::new("Jose".to_string(),333333333,"holis@outlook.com".to_string());
        let libro1=Libro::new("9788497592208".to_string(),"Don Quijote de la Mancha".to_string(),"Miguel de Cervantes".to_string(),1376,Genero::Novela);
        let libro2=Libro::new("9788437604947".to_string(),"El principito".to_string(),"Antoine de Saint-Exupéry".to_string(),96,Genero::Infantil);
        let libro3=Libro::new("9781718500440".to_string(),"The Rust Programming Language".to_string(),"Steve Klabnik y Carol Nichols".to_string(),560,Genero::Tecnico);
        
        let mut biblio=Biblioteca::new("Biblioteca Central La Plata".to_string(),"Calle 50 N° 742, La Plata".to_string());

        biblio.agregar_libro(&libro1, 3);
        biblio.agregar_libro(&libro2, 5);

        biblio.realizar_prestamo(&cliente, &libro1,Fecha::new(12,05,2026), 2);
        biblio.realizar_prestamo(&cliente2, &libro2,Fecha::new(03,05,2026), 5);

        assert!(biblio.buscar_prestamo(&libro3, &cliente).is_none());
    }

    #[test]
    fn buscar_prestamo_sin_prestamos_test(){
        let biblio=Biblioteca::new("Biblioteca Central La Plata".to_string(),"Calle 50 N° 742, La Plata".to_string());
        let libro=Libro::new("9788497592208".to_string(),"Don Quijote de la Mancha".to_string(),"Miguel de Cervantes".to_string(),1376,Genero::Novela);
        let cliente:Cliente=Cliente::new("Ana".to_string(),22222222,"hola@gmail.com".to_string());

        assert!(biblio.buscar_prestamo(&libro, &cliente).is_none());
    }

    #[test]
    fn devolver_libro_test(){
        let mut biblio=Biblioteca::new("Biblioteca Central La Plata".to_string(),"Calle 50 N° 742, La Plata".to_string());
        let libro1=Libro::new("9788497592208".to_string(),"Don Quijote de la Mancha".to_string(),"Miguel de Cervantes".to_string(),1376,Genero::Novela);
        let cliente:Cliente=Cliente::new("Ana".to_string(),22222222,"hola@gmail.com".to_string());

        biblio.agregar_libro(&libro1, 3);
        biblio.realizar_prestamo(&cliente, &libro1,Fecha::new(12,05,2026), 2);
        let fecha_actual:Fecha=Fecha::new(31,05,2026);

        assert!(biblio.devolver_libro(&libro1, &cliente, fecha_actual));
    }

    #[test]
    fn devolver_libro_inexistente_test(){
        let mut biblio=Biblioteca::new("Biblioteca Central La Plata".to_string(),"Calle 50 N° 742, La Plata".to_string());
        let libro1=Libro::new("9788497592208".to_string(),"Don Quijote de la Mancha".to_string(),"Miguel de Cervantes".to_string(),1376,Genero::Novela);
        let libro2=Libro::new("9788437604947".to_string(),"El principito".to_string(),"Antoine de Saint-Exupéry".to_string(),96,Genero::Infantil);
        let cliente:Cliente=Cliente::new("Ana".to_string(),22222222,"hola@gmail.com".to_string());

        biblio.agregar_libro(&libro1, 3);
        biblio.realizar_prestamo(&cliente, &libro1,Fecha::new(12,05,2026), 2);
        let fecha_actual:Fecha=Fecha::new(31,05,2026);

        assert!(!biblio.devolver_libro(&libro2, &cliente, fecha_actual));
    }

    #[test]
    fn devolver_libro_sin_prestamos_test(){
        let mut biblio=Biblioteca::new("Biblioteca Central La Plata".to_string(),"Calle 50 N° 742, La Plata".to_string());
        let libro=Libro::new("9788497592208".to_string(),"Don Quijote de la Mancha".to_string(),"Miguel de Cervantes".to_string(),1376,Genero::Novela);
        let cliente:Cliente=Cliente::new("Ana".to_string(),22222222,"hola@gmail.com".to_string());
        let fecha_actual:Fecha=Fecha::new(31,05,2026);

        assert!(!biblio.devolver_libro(&libro, &cliente, fecha_actual));
    }

}
