//El coverage del ejercicio original es de 91.13%

use std::collections::HashMap;
use crate::tp5::fecha::Fecha;
use std::fmt::Debug;
use std::{fs::File, io::Write};
use serde::Serialize;
use std::fmt::Display;

#[allow(unused)]
#[derive(Debug,PartialEq,Clone,Eq,Hash,Serialize)]
enum TipoSuscripcion{
    Basic ,
    Classic,
    Super,
}

#[allow(unused)]
#[derive(Debug,PartialEq,Clone,Serialize)]
enum EstadoSuscripcion{
    Activa,
    Inactiva,
}

#[allow(unused)]
#[derive(Debug,PartialEq,Clone,Eq,Hash,Serialize)]
enum MedioDePago{
    Efectivo,
    MercadoPago{cvu:u32, alias:String, correo:String},
    TarjetaDeCredito{numero:u32,titular:String,vencimiento:String,codigo_seguridad:u16},
    TransferenciaBancaria{cbu:u32,alias:String,banco:String,titular:String},
    Cripto{direccion_wallet:String,moneda:String},
}

#[allow(unused)]
#[derive(Debug,PartialEq,Clone,Serialize)]
struct Suscripcion{
    tipo:TipoSuscripcion,
    fecha_inicio:Fecha,
    pago:MedioDePago,
    estado:EstadoSuscripcion,
}

#[allow(unused)]
#[derive(Debug,PartialEq,Clone,Serialize)]
pub struct Usuario{
    nombre:String,
    suscripciones:Vec<Suscripcion>,
}

#[allow(unused)]
#[derive(Serialize)]
struct Plataforma{
    usuarios:Vec<Usuario>,
    path_archivo:String,
}

enum Errores{
    ErrorDeCreacionDeArchivo,
    ErrorDeEscrituraDeDatos,
    ErrorDeSerializacionDeDatos,
}

#[allow(unused)]
impl Display for Errores{
    fn fmt(&self,f: &mut std::fmt::Formatter<'_>)-> std::fmt::Result{
        match self{
            Errores::ErrorDeCreacionDeArchivo=>write!(f,"No se pudo acceder al archivo"),
            Errores::ErrorDeEscrituraDeDatos=>write!(f,"No se pudo escribir en el archivo"),
            Errores::ErrorDeSerializacionDeDatos=>write!(f,"No se pudieron serializar los datos"),
        }
    }
}

#[allow(unused)]
impl MedioDePago{

    fn tipo_de_pago(&self)->String{
        match self{
            MedioDePago::Cripto { .. }=>"Cripto".to_string(),
            MedioDePago::TransferenciaBancaria { ..}=>"Transferencia bancaria".to_string(),
            MedioDePago::TarjetaDeCredito { .. }=>"Tarjeta de credito".to_string(),
            MedioDePago::MercadoPago { .. }=>"Mercado Pago".to_string(),
            MedioDePago::Efectivo=>"Efectivo".to_string(),
        }
    }
}

#[allow(unused)]
impl TipoSuscripcion{

    fn get_costo(&self)->f32{
        match self{
            TipoSuscripcion::Basic => 500.0,
            TipoSuscripcion::Classic => 1000.0,
            TipoSuscripcion::Super=> 1500.0,
        }
    }

    fn get_duracion(&self)->u8{
        match self{
            TipoSuscripcion::Basic => 3,
            TipoSuscripcion::Classic => 6,
            TipoSuscripcion::Super=> 12,
        }
    }


}

#[allow(unused)]
impl Suscripcion{

    fn new(tipo:TipoSuscripcion,pago:MedioDePago,fecha_inicio:Fecha)->Suscripcion{
        Suscripcion{
            tipo,
            fecha_inicio,
            pago,
            estado:EstadoSuscripcion::Activa,
        }
    }

    fn cambiar_estado(&mut self){
            self.estado=EstadoSuscripcion::Inactiva;
    }

}

#[allow(unused)]
impl Usuario{

    //que la plataforma verifique si el usuario existe
    fn new (nombre:String)->Usuario{
        Usuario{
            nombre,
            suscripciones:Vec::new(),
        }
    }

    fn agregar_suscripcion(&mut self, suscripcion:Suscripcion){
        if self.suscripciones.len()>0{
            let ultimo=self.suscripciones.len()-1;
            self.suscripciones[ultimo].estado=EstadoSuscripcion::Inactiva;
        }
        self.suscripciones.push(suscripcion);
    }

    fn get_suscripcion_activa(&mut self)->Result<&mut Suscripcion,String>{
        self.suscripciones.iter_mut()
                        .find(|s|s.estado==EstadoSuscripcion::Activa)
                        .ok_or("El usuario no tiene suscripciones activas".to_string())
    }

    fn get_suscripcion(&mut self,suscripcion:&Suscripcion)->&mut Suscripcion{
        self.suscripciones.iter_mut()
                        .find(|s|*s==suscripcion).unwrap()
    
    }

    fn upgrade_suscripcion(&mut self){
        if let Ok(susc)=self.get_suscripcion_activa(){
            match susc.tipo{
                TipoSuscripcion::Basic=>susc.tipo=TipoSuscripcion::Classic,
                TipoSuscripcion::Classic=>susc.tipo=TipoSuscripcion::Super,
                TipoSuscripcion::Super=>(),
            }     
        }
    }

    fn downgrade_suscripcion(&mut self){
        if let Ok(susc)=self.get_suscripcion_activa(){
            match susc.tipo{
                TipoSuscripcion::Basic=>susc.cambiar_estado(),
                TipoSuscripcion::Classic=>susc.tipo=TipoSuscripcion::Basic,
                TipoSuscripcion::Super=>susc.tipo=TipoSuscripcion::Classic,
            }     
        }
    }

    fn contabilizar_pagos(&self,contador_pagos:&mut HashMap<String, u32>,contar_activas:bool){
        if contar_activas{
            if let Some(susc)=self.suscripciones.iter().find(|susc|susc.estado==EstadoSuscripcion::Activa){
                *contador_pagos.entry(susc.pago.tipo_de_pago()).or_insert(0)+=1;
            }
        }else{
            self.suscripciones.iter().for_each(|s|*contador_pagos.entry(s.pago.tipo_de_pago()).or_insert(0)+=1);
        }

    }

    fn contabilizar_tipos(&self,contador_tipos:&mut HashMap<TipoSuscripcion, u32>,contar_activas:bool){
        if contar_activas{
            if let Some(susc)=self.suscripciones.iter().find(|susc|susc.estado==EstadoSuscripcion::Activa){
                *contador_tipos.entry(susc.tipo.clone()).or_insert(0)+=1;
            }
        }else{
            self.suscripciones.iter().for_each(|s|*contador_tipos.entry(s.tipo.clone()).or_insert(0)+=1);
        }

    }

}

#[allow(unused)]
impl Plataforma{

    pub fn new()->Plataforma{
        Self::inner_new("src/tp5/archivos_suscripciones".to_string())
    }

    fn inner_new(path:String)->Plataforma{
        Plataforma { usuarios:Vec::new(), path_archivo:path}
    }

    fn usuario_existente(&mut self,nombre:String)->Option<&mut Usuario> {
        self.usuarios.iter_mut().find(|user|nombre==user.nombre)
    }

    fn agregar_otra_suscripcion(&mut self, nombre:String,susc:Suscripcion)->Result<(),String>{
        if self.usuarios.len()==0{
            return Err("La plataforma no cuenta con usuarios".to_string());
        }

        let mut usuario=self.usuario_existente(nombre);
        if usuario.is_some(){
            usuario.unwrap().agregar_suscripcion(susc.clone());
            Ok(())
        }else{
            Err("El usuario no existe".to_string())
        }
    }

    fn escribir_en_archivo_json(&self)->Result<(),Errores>{
        let mut archivo=File::create(&self.path_archivo).map_err(|_|Errores::ErrorDeCreacionDeArchivo)?;
        let datos_serializados=serde_json::to_string(&self.usuarios).map_err(|_|Errores::ErrorDeSerializacionDeDatos)?;
        archivo.write_all(datos_serializados.as_bytes()).map_err(|_|Errores::ErrorDeEscrituraDeDatos)?;
        Ok(())
    }

    pub fn crear_usuario(&mut self,nombre:String, tipo_suscripcion:TipoSuscripcion,pago:MedioDePago,fecha:Fecha)->Result<Usuario,String>{
        if self.usuario_existente(nombre.clone()).is_none(){
            let suscripcion=Suscripcion::new(tipo_suscripcion,pago,fecha);
            let mut usuario=Usuario::new(nombre);

            usuario.agregar_suscripcion(suscripcion.clone());
            self.usuarios.push(usuario.clone());
            if let Err(err)=self.escribir_en_archivo_json(){
                self.usuarios.pop();
                return Err(err.to_string());
            }
            return Ok(usuario);
        }
        Err("Ya existe otro usuario con el nombre ingresado".to_string())
    }

    pub fn get_usuario(&self,nombre:String)->Option<Usuario>{
        self.usuarios.iter().find(|user|user.nombre==nombre).cloned()
    }

    pub fn upgrade(&mut self,nombre:String)->Result<(),String>{
        if self.usuario_existente(nombre.clone()).is_none(){
            return Err("El usuario no existe".to_string());
        }
        let back_up=self.usuarios.clone();
        let usuario=self.usuario_existente(nombre).unwrap();
        usuario.upgrade_suscripcion();
        if let Err(err)=self.escribir_en_archivo_json(){
            self.usuarios=back_up;
            return Err(err.to_string());
        }
       Ok(())    
    }

    pub fn downgrade(&mut self, nombre:String)->Result<(),String>{
        if self.usuario_existente(nombre.clone()).is_none(){
            return Err("El usuario no existe".to_string());
        }
        let back_up=self.usuarios.clone();
        let usuario=self.usuario_existente(nombre).unwrap();
        usuario.downgrade_suscripcion();
        if let Err(err)=self.escribir_en_archivo_json(){
            self.usuarios=back_up;
            return Err(err.to_string());
        }
       Ok(())    
    }

    pub fn cancelar_suscripcion(&mut self,nombre_usuario:String)->Result<(),String>{
        if self.usuario_existente(nombre_usuario.clone()).is_none(){
            return Err("El usuario no existe".to_string());
        }
        let back_up=self.usuarios.clone();
        let user=self.usuario_existente(nombre_usuario).unwrap();
        user.get_suscripcion_activa()?.cambiar_estado();
        if let Err(err)=self.escribir_en_archivo_json(){
            self.usuarios=back_up;
            return Err(err.to_string());
        }
        Ok(())
    }

    fn pago_mas_popular_suscripciones_activas(&self)-> Result<String,String>{
       let mut contador_pagos: HashMap<String, u32>=HashMap::new();

        //el boolean indica si se contabilizan las activas
        self.usuarios.iter().for_each(|user|user.contabilizar_pagos(&mut contador_pagos,true));
       
        let maximo=contador_pagos.iter()
                            .max_by_key(|&(_,cantidad)|cantidad);


        if let Some((clave,valor))=maximo{
            Ok(clave.clone())
        }else{
            Err("No hay suscripciones activas".to_string())
        }
    }

    fn tipo_suscripcion_mas_popular_activas(&self)->Result<TipoSuscripcion,&str>{
        let mut contador_tipos: HashMap<TipoSuscripcion, u32>=HashMap::new();
        self.usuarios.iter().for_each(|user|user.contabilizar_tipos(&mut contador_tipos,true));

        let maximo=contador_tipos.iter().max_by_key(|&(_,cantidad)|cantidad);
        if let Some((clave,valor))=maximo{
            Ok(clave.clone())
        }else{
            Err("No hay suscripciones activas")
        }
    }

    fn medio_de_pago_mas_utilizado(&self)-> Result<String,String>{
        let mut contador_pagos: HashMap<String, u32>=HashMap::new();

        //el boolean indica si se contabilizan las activas
        self.usuarios.iter().for_each(|user|user.contabilizar_pagos(&mut contador_pagos,false));
       
        let maximo=contador_pagos.iter()
                            .max_by_key(|&(_,cantidad)|cantidad);


        if let Some((clave,valor))=maximo{
            Ok(clave.clone())
        }else{
            Err("No hay suscripciones registradas".to_string())
        }
    }
    fn tipo_de_suscripcion_mas_contratada(&self)->Result<TipoSuscripcion,&str>{
        let mut contador_tipos: HashMap<TipoSuscripcion, u32>=HashMap::new();
        self.usuarios.iter().for_each(|user|user.contabilizar_tipos(&mut contador_tipos,false));

        let maximo=contador_tipos.iter().max_by_key(|&(_,cantidad)|cantidad);
        if let Some((clave,valor))=maximo{
            Ok(clave.clone())
        }else{
            Err("No hay suscripciones registradas")
        }
    }
    
}

#[cfg(test)]
mod test{
    use crate::tp5::ej5::{EstadoSuscripcion, MedioDePago, Plataforma, Suscripcion, TipoSuscripcion, Usuario};
    use crate::tp5::fecha::Fecha;
    use std::fs;

    //test crear el usuario
    #[test]
    fn crear_usuario_test(){
        let mut streaming=Plataforma::new();

        let usuario_nuevo=streaming.crear_usuario("Anaaaa".to_string(),
                        TipoSuscripcion::Classic, 
                        MedioDePago::TransferenciaBancaria { cbu: 2222222, alias: "ana.p.alfaro".to_string(), banco: "Banco Provincia de Santa Cruz".to_string(), titular: "Alfaro Ana".to_string() }
                        ,Fecha::new(18,06,2026));

        let mut usuario=Usuario::new("Anaaaa".to_string());
        let suscripcion= Suscripcion::new(TipoSuscripcion::Classic , 
                        MedioDePago::TransferenciaBancaria { cbu: 2222222, alias: "ana.p.alfaro".to_string(), banco: "Banco Provincia de Santa Cruz".to_string(), titular: "Alfaro Ana".to_string() }
                        ,Fecha::new(18,06,2026));

        usuario.agregar_suscripcion(suscripcion);

        assert_eq!(usuario_nuevo,Ok(usuario));
    }

    #[test]
    fn crear_usuario_usuario_existente_test(){
        let mut streaming=Plataforma::new();

        let _=streaming.crear_usuario("Anaaaa".to_string(),
                        TipoSuscripcion::Classic, 
                        MedioDePago::TransferenciaBancaria { cbu: 2222222, alias: "ana.p.alfaro".to_string(), banco: "Banco Provincia de Santa Cruz".to_string(), titular: "Alfaro Ana".to_string() }
                        ,Fecha::new(18,06,2026));

        let resultado=streaming.crear_usuario("Anaaaa".to_string(),
                        TipoSuscripcion::Classic , 
                        MedioDePago::Cripto { direccion_wallet: "0x123456".to_string(), moneda: "Ethereum".to_string() }
                        ,Fecha::new(18,06,2026));
        assert_eq!(resultado,Err("Ya existe otro usuario con el nombre ingresado".to_string()));
    }

    //test agregar otra suscripcion. Es una funcion que agregué para utilizarla.
    #[test]
    fn agregar_otra_suscripcion_test(){
        let mut streaming=Plataforma::new();

        let _=streaming.crear_usuario("Ana".to_string(),
                        TipoSuscripcion::Classic, 
                        MedioDePago::TransferenciaBancaria { cbu: 2222222, alias: "ana.p.alfaro".to_string(), banco: "Banco Provincia de Santa Cruz".to_string(), titular: "Alfaro Ana".to_string() }
                        ,Fecha::new(18,06,2026));

        let susc=Suscripcion::new(TipoSuscripcion::Classic, 
                                        MedioDePago::Efectivo
                                        ,Fecha::new(24,06,2026));

        let resultado=streaming.agregar_otra_suscripcion("Ana".to_string(), susc);
        assert_eq!(resultado,Ok(()));

        let usuario=streaming.get_usuario("Ana".to_string()).unwrap();

        assert_eq!(usuario.suscripciones.len(),2);
    }

    #[test]
    fn agregar_otra_suscripcion_usuario_inexistente_test(){
        let mut streaming=Plataforma::new();

        let _=streaming.crear_usuario("Ana".to_string(),
                        TipoSuscripcion::Classic, 
                        MedioDePago::TransferenciaBancaria { cbu: 2222222, alias: "ana.p.alfaro".to_string(), banco: "Banco Provincia de Santa Cruz".to_string(), titular: "Alfaro Ana".to_string() }
                        ,Fecha::new(18,06,2026));

        let susc=Suscripcion::new(TipoSuscripcion::Classic, 
                                        MedioDePago::Efectivo
                                        ,Fecha::new(24,06,2026));

        let resultado=streaming.agregar_otra_suscripcion("Juan".to_string(), susc);
        assert_eq!(resultado,Err("El usuario no existe".to_string()));
    }

    #[test]
    fn agregar_otra_suscripcion_plataforma_vacia_test(){
        let mut streaming=Plataforma::new();

        let susc=Suscripcion::new(TipoSuscripcion::Classic, 
                                        MedioDePago::Efectivo
                                        ,Fecha::new(24,06,2026));

        let resultado=streaming.agregar_otra_suscripcion("Juan".to_string(), susc);
        assert_eq!(resultado,Err("La plataforma no cuenta con usuarios".to_string()));
    }

    //test upgrade de suscripcion
    #[test]
    fn upgrade_multiples_suscripciones_existentes_test(){
        let mut streaming=Plataforma::new();
        streaming.crear_usuario("Ana".to_string(),
                        TipoSuscripcion::Basic, 
                        MedioDePago::Efectivo
                        ,Fecha::new(18,06,2026)).unwrap();

        let susc=Suscripcion::new(TipoSuscripcion::Classic, 
                                        MedioDePago::TransferenciaBancaria { cbu: 2222222, alias: "ana.p.alfaro".to_string(), banco: "Banco Provincia de Santa Cruz".to_string(), titular: "Alfaro Ana".to_string() }
                                        ,Fecha::new(24,06,2026));
        let _=streaming.agregar_otra_suscripcion("Ana".to_string(), susc.clone());

        let _=streaming.upgrade("Ana".to_string());
        let mut usuario=streaming.get_usuario("Ana".to_string()).unwrap();
        let resultado=usuario.get_suscripcion_activa().unwrap();

        assert!(resultado.tipo==TipoSuscripcion::Super);
        assert!(resultado.estado==EstadoSuscripcion::Activa);
        assert!(resultado.pago.tipo_de_pago()=="Transferencia bancaria".to_string());
        assert!(resultado.fecha_inicio==Fecha::new(24,06,2026));
    }

    #[test]
    fn upgrade_classic_a_super_test(){
        let mut streaming=Plataforma::new();
        streaming.crear_usuario("Ana".to_string(),
                        TipoSuscripcion::Classic, 
                        MedioDePago::Cripto { direccion_wallet: "0x123456".to_string(), moneda: "Ethereum".to_string() }
                        ,Fecha::new(18,06,2026)).unwrap();

        let _=streaming.upgrade("Ana".to_string());
        let mut usuario=streaming.get_usuario("Ana".to_string()).unwrap();

        let resultado=usuario.get_suscripcion_activa().unwrap();

        assert!(resultado.tipo==TipoSuscripcion::Super);
        assert!(resultado.estado==EstadoSuscripcion::Activa);
        assert!(resultado.pago.tipo_de_pago()=="Cripto".to_string());
        assert!(resultado.fecha_inicio==Fecha::new(18,06,2026));

    }

    #[test]
    fn upgrade_basic_a_classic_test(){
        let mut streaming=Plataforma::new();
        streaming.crear_usuario("Ana".to_string(),
                        TipoSuscripcion::Basic, 
                        MedioDePago::TarjetaDeCredito { numero: 2222222, titular:"Ana".to_string(), vencimiento:"02/32".to_string(), codigo_seguridad: 315 }
                        ,Fecha::new(18,06,2026)).unwrap();

        let _=streaming.upgrade("Ana".to_string());
        let mut usuario=streaming.get_usuario("Ana".to_string()).unwrap();

        let resultado=usuario.get_suscripcion_activa().unwrap();

        assert!(resultado.tipo==TipoSuscripcion::Classic);
        assert!(resultado.estado==EstadoSuscripcion::Activa);
        assert!(resultado.pago.tipo_de_pago()=="Tarjeta de credito".to_string());
        assert!(resultado.fecha_inicio==Fecha::new(18,06,2026));
    }

    #[test]
    fn upgrade_sin_cambios_test(){
        let mut streaming=Plataforma::new();
        streaming.crear_usuario("Ana".to_string(),
                        TipoSuscripcion::Super, 
                        MedioDePago::TransferenciaBancaria { cbu: 2222222, alias: "ana.p.alfaro".to_string(), banco: "Banco Provincia de Santa Cruz".to_string(), titular: "Alfaro Ana".to_string() }
                        ,Fecha::new(18,06,2026)).unwrap();

        let _=streaming.upgrade("Ana".to_string());
        let mut usuario=streaming.get_usuario("Ana".to_string()).unwrap();
        let resultado=usuario.get_suscripcion_activa().unwrap();

        assert!(resultado.tipo==TipoSuscripcion::Super);
        assert!(resultado.tipo==TipoSuscripcion::Super);
        assert!(resultado.estado==EstadoSuscripcion::Activa);
        assert!(resultado.pago.tipo_de_pago()=="Transferencia bancaria".to_string());
        assert!(resultado.fecha_inicio==Fecha::new(18,06,2026));
        
    }

    #[test]
    fn upgrade_usuario_inexistente_test(){
        let mut streaming=Plataforma::new();
        let resultado=streaming.upgrade("Ana".to_string());
        assert_eq!(resultado,Err("El usuario no existe".to_string()));
    }

    //test downgrade de suscripcion
    #[test]
    fn downgrade_multiples_suscripciones_existentes_test(){
        let mut streaming=Plataforma::new();
        streaming.crear_usuario("Ana".to_string(),
                        TipoSuscripcion::Basic, 
                        MedioDePago::TransferenciaBancaria { cbu: 2222222, alias: "ana.p.alfaro".to_string(), banco: "Banco Provincia de Santa Cruz".to_string(), titular: "Alfaro Ana".to_string() }
                        ,Fecha::new(18,06,2026)).unwrap();

        let susc=Suscripcion::new(TipoSuscripcion::Classic, 
                                        MedioDePago::Efectivo
                                        ,Fecha::new(24,06,2026));
        let _=streaming.agregar_otra_suscripcion("Ana".to_string(), susc.clone());

        let _=streaming.downgrade("Ana".to_string());
        let mut usuario=streaming.get_usuario("Ana".to_string()).unwrap();

        assert!(usuario.get_suscripcion_activa().unwrap().tipo==TipoSuscripcion::Basic);
    }

    #[test]
    fn downgrade_super_a_classic_test(){
        let mut streaming=Plataforma::new();
        streaming.crear_usuario("Ana".to_string(),
                        TipoSuscripcion::Super, 
                        MedioDePago::TransferenciaBancaria { cbu: 2222222, alias: "ana.p.alfaro".to_string(), banco: "Banco Provincia de Santa Cruz".to_string(), titular: "Alfaro Ana".to_string() }
                        ,Fecha::new(18,06,2026)).unwrap();

        let _=streaming.downgrade("Ana".to_string());
        let mut usuario=streaming.get_usuario("Ana".to_string()).unwrap();

        let resultado=usuario.get_suscripcion_activa().unwrap();

        assert!(resultado.tipo==TipoSuscripcion::Classic);
        assert!(resultado.estado==EstadoSuscripcion::Activa);
        assert!(resultado.pago.tipo_de_pago()=="Transferencia bancaria".to_string());
        assert!(resultado.fecha_inicio==Fecha::new(18,06,2026));
    }

    #[test]
    fn downgrade_classic_a_basic_test(){
        let mut streaming=Plataforma::new();
        streaming.crear_usuario("Ana".to_string(),
                        TipoSuscripcion::Classic, 
                        MedioDePago::TransferenciaBancaria { cbu: 2222222, alias: "ana.p.alfaro".to_string(), banco: "Banco Provincia de Santa Cruz".to_string(), titular: "Alfaro Ana".to_string() }
                        ,Fecha::new(18,06,2026)).unwrap();

        let _=streaming.downgrade("Ana".to_string());
        let mut usuario=streaming.get_usuario("Ana".to_string()).unwrap();

        let resultado=usuario.get_suscripcion_activa().unwrap();
        assert!(resultado.tipo==TipoSuscripcion::Basic);
        assert!(resultado.estado==EstadoSuscripcion::Activa);
        assert!(resultado.pago.tipo_de_pago()=="Transferencia bancaria".to_string());
        assert!(resultado.fecha_inicio==Fecha::new(18,06,2026));
    }

    #[test]
    fn downgrade_basic_a_cancelacion_test(){
        let mut streaming=Plataforma::new();
        streaming.crear_usuario("Ana".to_string(),
                        TipoSuscripcion::Basic, 
                        MedioDePago::TransferenciaBancaria { cbu: 2222222, alias: "ana.p.alfaro".to_string(), banco: "Banco Provincia de Santa Cruz".to_string(), titular: "Alfaro Ana".to_string() }
                        ,Fecha::new(18,06,2026)).unwrap();

        let _=streaming.downgrade("Ana".to_string());
        let mut usuario=streaming.get_usuario("Ana".to_string()).unwrap();
        let mut susc=Suscripcion::new(TipoSuscripcion::Basic, 
                        MedioDePago::TransferenciaBancaria { cbu: 2222222, alias: "ana.p.alfaro".to_string(), banco: "Banco Provincia de Santa Cruz".to_string(), titular: "Alfaro Ana".to_string() }
                        ,Fecha::new(18,06,2026));
        susc.cambiar_estado();

        let resultado=usuario.get_suscripcion(&susc);
        assert!(resultado.estado==EstadoSuscripcion::Inactiva);
    }

    #[test]
    fn downgrade_usuario_inexistente_test(){
        let mut streaming=Plataforma::new();
        let resultado=streaming.downgrade("Ana".to_string());
        assert_eq!(resultado,Err("El usuario no existe".to_string()));
    }

    //test cancelacion de suscripcion
    #[test]
    fn cancelar_suscripcion_test(){
        let mut streaming=Plataforma::new();
        let usuario=streaming.crear_usuario("Ana".to_string(),
                        TipoSuscripcion::Basic, 
                        MedioDePago::TransferenciaBancaria { cbu: 2222222, alias: "ana.p.alfaro".to_string(), banco: "Banco Provincia de Santa Cruz".to_string(), titular: "Alfaro Ana".to_string() }
                        ,Fecha::new(18,06,2026)).unwrap();

        let _=streaming.cancelar_suscripcion(usuario.nombre).unwrap();

        let mut actualizacion_usuario=streaming.get_usuario("Ana".to_string()).unwrap();

        let mut susc=Suscripcion::new(TipoSuscripcion::Basic, 
                        MedioDePago::TransferenciaBancaria { cbu: 2222222, alias: "ana.p.alfaro".to_string(), banco: "Banco Provincia de Santa Cruz".to_string(), titular: "Alfaro Ana".to_string() }
                        ,Fecha::new(18,06,2026));
        susc.cambiar_estado();

        let resultado=actualizacion_usuario.get_suscripcion(&susc);

        assert!(resultado.estado==EstadoSuscripcion::Inactiva);
    }

    #[test]
    fn cancelar_suscripcion_varias_suscripciones_test(){
        let mut streaming=Plataforma::new();
        let usuario=streaming.crear_usuario("Ana".to_string(),
                        TipoSuscripcion::Basic, 
                        MedioDePago::TransferenciaBancaria { cbu: 2222222, alias: "ana.p.alfaro".to_string(), banco: "Banco Provincia de Santa Cruz".to_string(), titular: "Alfaro Ana".to_string() }
                        ,Fecha::new(18,06,2026)).unwrap();

        let mut susc=Suscripcion::new(TipoSuscripcion::Super, 
                                        MedioDePago::Efectivo
                                        ,Fecha::new(24,06,2026));
        let _=streaming.agregar_otra_suscripcion("Ana".to_string(), susc.clone());

        let _=streaming.cancelar_suscripcion(usuario.nombre).unwrap();

        let mut actualizacion_usuario=streaming.get_usuario("Ana".to_string()).unwrap();
        susc.cambiar_estado();

        let resultado=actualizacion_usuario.get_suscripcion(&susc);

        assert!(resultado.estado==EstadoSuscripcion::Inactiva);
    }

    #[test]
    fn cancelar_suscripcion_usuario_inexistente_test(){
        let mut streaming=Plataforma::new();
        let usuario=Usuario::new("Ana".to_string());
        let resultado=streaming.cancelar_suscripcion(usuario.nombre);
        assert_eq!(resultado,Err("El usuario no existe".to_string()));
    }

    #[test]
    fn cancelar_suscripcion_usuario_sin_suscripciones_activas_test(){
        let mut streaming=Plataforma::new();
        let usuario=streaming.crear_usuario("Ana".to_string(),
                        TipoSuscripcion::Basic, 
                        MedioDePago::TransferenciaBancaria { cbu: 2222222, alias: "ana.p.alfaro".to_string(), banco: "Banco Provincia de Santa Cruz".to_string(), titular: "Alfaro Ana".to_string() }
                        ,Fecha::new(18,06,2026)).unwrap();

        let _=streaming.downgrade("Ana".to_string());
        let susc=Suscripcion::new(TipoSuscripcion::Basic, 
                                        MedioDePago::Efectivo
                                        ,Fecha::new(24,06,2026));
        let _=streaming.agregar_otra_suscripcion("Ana".to_string(), susc.clone());

        let _=streaming.downgrade("Ana".to_string());
        let resultado=streaming.cancelar_suscripcion(usuario.nombre);

        assert_eq!(resultado,Err("El usuario no tiene suscripciones activas".to_string()));
    }

    //test medio de pago mas popular en suscripciones activas
    #[test]
    fn pago_mas_popular_suscripciones_activas_test(){
        let mut streaming=Plataforma::new();
        let _=streaming.crear_usuario("Ana".to_string(),
                        TipoSuscripcion::Basic, 
                        MedioDePago::TransferenciaBancaria { cbu: 2222222, alias: "ana.p.alfaro".to_string(), banco: "Banco Provincia de Santa Cruz".to_string(), titular: "Alfaro Ana".to_string() }
                        ,Fecha::new(18,06,2026)).unwrap();
        let _=streaming.crear_usuario("Juan".to_string(),
                        TipoSuscripcion::Classic, 
                        MedioDePago::TransferenciaBancaria { cbu: 2332222, alias: " Juan".to_string(), banco: "Banco Provincia de Santa Cruz".to_string(), titular: "Juan".to_string() }
                        ,Fecha::new(22,06,2026)).unwrap();
        let _=streaming.crear_usuario("Jose".to_string(),
                        TipoSuscripcion::Basic, 
                        MedioDePago::Efectivo
                        ,Fecha::new(22,06,2026)).unwrap();

        let resultado=streaming.pago_mas_popular_suscripciones_activas();

        assert!(resultado.unwrap()=="Transferencia bancaria");
    }

    #[test]
    fn pago_mas_popular_suscripciones_activas_sin_suscripciones_activas_test(){
        let mut streaming=Plataforma::new();
        let usuario1=streaming.crear_usuario("Ana".to_string(),
                        TipoSuscripcion::Basic, 
                        MedioDePago::TransferenciaBancaria { cbu: 2222222, alias: "ana.p.alfaro".to_string(), banco: "Banco Provincia de Santa Cruz".to_string(), titular: "Alfaro Ana".to_string() }
                        ,Fecha::new(18,06,2026)).unwrap();
        let usuario2=streaming.crear_usuario("Juan".to_string(),
                        TipoSuscripcion::Classic, 
                        MedioDePago::Efectivo
                        ,Fecha::new(22,06,2026)).unwrap();
        
        let _=streaming.cancelar_suscripcion(usuario1.nombre);
        let _=streaming.cancelar_suscripcion(usuario2.nombre);

        let resultado=streaming.pago_mas_popular_suscripciones_activas();

        assert_eq!(resultado,Err("No hay suscripciones activas".to_string()));
    }

    #[test]
    fn pago_mas_popular_suscripciones_activas_sin_suscripciones_test(){
        let streaming=Plataforma::new();
        let resultado=streaming.pago_mas_popular_suscripciones_activas();
        assert_eq!(resultado,Err("No hay suscripciones activas".to_string()));
    }

    //test suscripción más contratada por los usuarios sobre las suscripciones activas.
    #[test]
    fn tipo_suscripcion_mas_popular_activas_test(){
        let mut streaming=Plataforma::new();
        let _=streaming.crear_usuario("Ana".to_string(),
                        TipoSuscripcion::Basic, 
                        MedioDePago::TransferenciaBancaria { cbu: 2222222, alias: "ana.p.alfaro".to_string(), banco: "Banco Provincia de Santa Cruz".to_string(), titular: "Alfaro Ana".to_string() }
                        ,Fecha::new(18,06,2026)).unwrap();
        let _=streaming.crear_usuario("Juan".to_string(),
                        TipoSuscripcion::Classic, 
                        MedioDePago::TransferenciaBancaria { cbu: 2332222, alias: " Juan".to_string(), banco: "Banco Provincia de Santa Cruz".to_string(), titular: "Juan".to_string() }
                        ,Fecha::new(22,06,2026)).unwrap();
        let _=streaming.crear_usuario("Jose".to_string(),
                        TipoSuscripcion::Basic, 
                        MedioDePago::Efectivo
                        ,Fecha::new(22,06,2026)).unwrap();

        let resultado=streaming.tipo_suscripcion_mas_popular_activas();

        assert!(resultado.unwrap()==TipoSuscripcion::Basic);
    }

    #[test]
    fn tipo_suscripcion_mas_popular_activas_sin_suscripciones_activas_test(){
        let mut streaming=Plataforma::new();
        let usuario1=streaming.crear_usuario("Ana".to_string(),
                        TipoSuscripcion::Basic, 
                        MedioDePago::TransferenciaBancaria { cbu: 2222222, alias: "ana.p.alfaro".to_string(), banco: "Banco Provincia de Santa Cruz".to_string(), titular: "Alfaro Ana".to_string() }
                        ,Fecha::new(18,06,2026)).unwrap();
        let usuario2=streaming.crear_usuario("Juan".to_string(),
                        TipoSuscripcion::Classic, 
                        MedioDePago::Efectivo
                        ,Fecha::new(22,06,2026)).unwrap();
        
        let _=streaming.cancelar_suscripcion(usuario1.nombre);
        let _=streaming.cancelar_suscripcion(usuario2.nombre);

        let resultado=streaming.tipo_suscripcion_mas_popular_activas();

        assert_eq!(resultado,Err("No hay suscripciones activas"));
    }

    #[test]
    fn tipo_suscripcion_mas_popular_activas_sin_suscripciones_test(){
        let streaming=Plataforma::new();
        let resultado=streaming.tipo_suscripcion_mas_popular_activas();
        assert_eq!(resultado,Err("No hay suscripciones activas"));
    }

    // test medio de pago más utilizado.
    #[test]
    fn medio_de_pago_mas_utilizado_test(){
        let mut streaming=Plataforma::new();
        let usuario1=streaming.crear_usuario("Ana".to_string(),
                        TipoSuscripcion::Basic, 
                        MedioDePago::TransferenciaBancaria { cbu: 2222222, alias: "ana.p.alfaro".to_string(), banco: "Banco Provincia de Santa Cruz".to_string(), titular: "Alfaro Ana".to_string() }
                        ,Fecha::new(18,06,2026)).unwrap();
        let usuario2=streaming.crear_usuario("Juan".to_string(),
                        TipoSuscripcion::Classic, 
                        MedioDePago::TransferenciaBancaria { cbu: 2332222, alias: " Juan".to_string(), banco: "Banco Provincia de Santa Cruz".to_string(), titular: "Juan".to_string() }
                        ,Fecha::new(22,06,2026)).unwrap();
        
        let _=streaming.cancelar_suscripcion(usuario1.nombre);
        let _=streaming.cancelar_suscripcion(usuario2.nombre);

        let _=streaming.crear_usuario("Jose".to_string(),
                        TipoSuscripcion::Basic, 
                        MedioDePago::Efectivo
                        ,Fecha::new(22,06,2026)).unwrap();
        
        let resultado=streaming.medio_de_pago_mas_utilizado();

        assert!(resultado.unwrap()=="Transferencia bancaria");
    }

    #[test]
    fn medio_de_pago_mas_utilizado_sin_suscripciones_test(){
        let streaming=Plataforma::new();
        let resultado=streaming.medio_de_pago_mas_utilizado();
        assert_eq!(resultado,Err("No hay suscripciones registradas".to_string()));
    }

    //test suscripción más contratada.
    #[test]
    fn tipo_de_suscripcion_mas_contratada_test(){
        let mut streaming=Plataforma::new();
        let usuario1=streaming.crear_usuario("Ana".to_string(),
                        TipoSuscripcion::Basic, 
                        MedioDePago::TransferenciaBancaria { cbu: 2222222, alias: "ana.p.alfaro".to_string(), banco: "Banco Provincia de Santa Cruz".to_string(), titular: "Alfaro Ana".to_string() }
                        ,Fecha::new(18,06,2026)).unwrap();
        let usuario2=streaming.crear_usuario("Juan".to_string(),
                        TipoSuscripcion::Classic, 
                        MedioDePago::TransferenciaBancaria { cbu: 2332222, alias: " Juan".to_string(), banco: "Banco Provincia de Santa Cruz".to_string(), titular: "Juan".to_string() }
                        ,Fecha::new(22,06,2026)).unwrap();
        
        let _=streaming.cancelar_suscripcion(usuario1.nombre);
        let _=streaming.cancelar_suscripcion(usuario2.nombre);

        let _=streaming.crear_usuario("Jose".to_string(),
                        TipoSuscripcion::Basic, 
                        MedioDePago::Efectivo
                        ,Fecha::new(22,06,2026)).unwrap();
        
        let resultado=streaming.tipo_de_suscripcion_mas_contratada();

        assert_eq!(resultado.unwrap(),TipoSuscripcion::Basic);
    }

    #[test]
    fn tipo_de_suscripcion_mas_contratada_sin_suscripciones_test(){
        let streaming=Plataforma::new();
        let resultado=streaming.tipo_de_suscripcion_mas_contratada();
        assert_eq!(resultado,Err("No hay suscripciones registradas"));
    }

    #[test]
    fn crear_usuario_fallido_por_creacion_de_archivo_fallida_test(){
        let path="src/tp5/testeo_de_errores_crear_usuario";
        let _ = std::fs::remove_file(path);
        let _ = std::fs::remove_dir(path);
        //El archivo existe pero es un directorio, no se pueden escribir datos en él

        fs::create_dir(path).unwrap();

        let mut plataforma=Plataforma::inner_new(path.to_string());

        assert!(plataforma.usuarios.len()==0);
        let usuario_nuevo=plataforma.crear_usuario("Anaaaa".to_string(),
        TipoSuscripcion::Classic, 
        MedioDePago::TransferenciaBancaria { cbu: 2222222, alias: "ana.p.alfaro".to_string(), banco: "Banco Provincia de Santa Cruz".to_string(), titular: "Alfaro Ana".to_string() }
        ,Fecha::new(18,06,2026));
        
        assert_eq!(usuario_nuevo,Err("No se pudo acceder al archivo".to_string()));
        assert!(plataforma.usuarios.len()==0);

        let _=std::fs::remove_dir(path);
    }

    #[test]
    fn upgrade_fallido_por_creacion_de_archivo_fallida_test(){
        let path="src/tp5/testeo_de_errores_upgrade_suscripcion";
        let _ = std::fs::remove_file(path);
        let _ = std::fs::remove_dir(path);
        //El archivo existe pero es un directorio, no se pueden escribir datos en él

        fs::create_dir(path).unwrap();

        let mut plataforma=Plataforma::inner_new(path.to_string());
        let mut usuario=Usuario::new("Ana".to_string());
        //pusheo para que el devuelva el error en upgrade y no en crear_usuario
        
        let susc=Suscripcion::new(TipoSuscripcion::Basic, 
            MedioDePago::TransferenciaBancaria { cbu: 2222222, alias: "ana.p.alfaro".to_string(), banco: "Banco Provincia de Santa Cruz".to_string(), titular: "Alfaro Ana".to_string() }
            ,Fecha::new(18,06,2026));
            
        usuario.suscripciones.push(susc.clone());
        plataforma.usuarios.push(usuario);
            
        let mut user=plataforma.get_usuario("Ana".to_string()).unwrap();
        assert!(user.get_suscripcion_activa().unwrap().tipo==TipoSuscripcion::Basic);
        let resultado=plataforma.upgrade("Ana".to_string());
        assert_eq!(resultado,Err("No se pudo acceder al archivo".to_string()));
        let mut usuario=plataforma.get_usuario("Ana".to_string()).unwrap();
        assert!(usuario.get_suscripcion_activa().unwrap().tipo==TipoSuscripcion::Basic);
        
    
        let _=std::fs::remove_dir(path);
    }

    #[test]
    fn downgrade_fallido_por_creacion_de_archivo_fallida_test(){
        let path="src/tp5/testeo_de_errores_downgrade_suscripcion";
        let _ = std::fs::remove_file(path);
        let _ = std::fs::remove_dir(path);
        //El archivo existe pero es un directorio, no se pueden escribir datos en él

        fs::create_dir(path).unwrap();

        let mut plataforma=Plataforma::inner_new(path.to_string());
        let mut usuario=Usuario::new("Ana".to_string());
        //pusheo para que el devuelva el error en upgrade y no en crear_usuario
        
        let susc=Suscripcion::new(TipoSuscripcion::Basic, 
            MedioDePago::TransferenciaBancaria { cbu: 2222222, alias: "ana.p.alfaro".to_string(), banco: "Banco Provincia de Santa Cruz".to_string(), titular: "Alfaro Ana".to_string() }
            ,Fecha::new(18,06,2026));
            
        usuario.suscripciones.push(susc.clone());
        plataforma.usuarios.push(usuario);
            
        let mut user=plataforma.get_usuario("Ana".to_string()).unwrap();
        assert!(user.get_suscripcion_activa().unwrap().tipo==TipoSuscripcion::Basic);
        let resultado=plataforma.downgrade("Ana".to_string());
        assert_eq!(resultado,Err("No se pudo acceder al archivo".to_string()));

        let mut usuario=plataforma.get_usuario("Ana".to_string()).unwrap();
        assert!(usuario.get_suscripcion_activa().unwrap().tipo==TipoSuscripcion::Basic);    
    
        let _=std::fs::remove_dir(path);
    }

    #[test]
    fn cancelar_suscripcion_fallido_por_creacion_de_archivo_fallida_test(){
        let path="src/tp5/testeo_de_errores_cancelar_suscripcion";
        let _ = std::fs::remove_file(path);
        let _ = std::fs::remove_dir(path);
        //El archivo existe pero es un directorio, no se pueden escribir datos en él

        fs::create_dir(path).unwrap();

        let mut plataforma=Plataforma::inner_new(path.to_string());
        let mut usuario=Usuario::new("Ana".to_string());
        //pusheo para que el devuelva el error en upgrade y no en crear_usuario
        
        let susc=Suscripcion::new(TipoSuscripcion::Basic, 
            MedioDePago::TransferenciaBancaria { cbu: 2222222, alias: "ana.p.alfaro".to_string(), banco: "Banco Provincia de Santa Cruz".to_string(), titular: "Alfaro Ana".to_string() }
            ,Fecha::new(18,06,2026));
            
        usuario.suscripciones.push(susc.clone());
        plataforma.usuarios.push(usuario);
            
        let resultado=plataforma.cancelar_suscripcion("Ana".to_string());
        assert_eq!(resultado,Err("No se pudo acceder al archivo".to_string()));
    
        let _=std::fs::remove_dir(path);
    }

    
}