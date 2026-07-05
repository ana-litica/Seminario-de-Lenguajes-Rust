//El coverage del ejercicio original me dio 95.19%

use crate::tp5::fecha::Fecha;
use std::collections::HashMap;
use std::fmt::Debug;
use std::{fs::File, io::Write};
use serde::Serialize;
use std::fmt::Display;

#[allow(unused)]
#[derive(Debug,PartialEq,Clone,Serialize)]
struct Usuario{
    nombre:String,
    apellido:String,
    email:String,
    dni:u64,
    identidad_validada:bool,
    balance_fiat:f64,
    balance_cripto:HashMap<String,f64>, //la clave es el prefijo 
}

#[allow(unused)]
#[derive(Debug,PartialEq,Clone,Serialize)]
struct Criptomoneda{
    nombre:String,
    prefijo:String,
    blockchains:Vec<Blockchain>,
    cotizacion:f64,
}

#[allow(unused)]
#[derive(Debug,PartialEq,Clone,Serialize)]
struct Blockchain{
    nombre:String,
    prefijo:String,
}

#[allow(unused)]
#[derive(Debug,PartialEq,Clone,Serialize)]
enum MedioDeRetiro{
    MercadoPago{cvu:u64,alias:String},
    TransferenciaBancaria {cbu:u64,alias:String},
}

#[allow(unused)]
#[derive(Debug,PartialEq,Clone,Serialize)]
struct Transaccion{
    fecha:Fecha,
    tipo:TipoTransaccion,
    monto:f64,
    usuario:u64, //o sea, dni del usuario
}

#[allow(unused)]
#[derive(Debug,PartialEq,Clone,Serialize)]
enum TipoTransaccion{
    IngresoDeFiat,
    CompraDeCripto{criptomoneda:String,cotizacion:f64},
    VentaDeCripto{criptomoneda:String,cotizacion:f64},
    RetiroDeCripto{criptomoneda:String,cotizacion:f64,blockchain:String,hash:String}, //nombre de blockchain
    RecepcionDeCripto{cotizacion:f64,blockchain:String},
    RetiroFiat{medio:MedioDeRetiro},
}

#[allow(unused)]
#[derive(Debug,PartialEq,Clone,Serialize)]
struct Plataforma{
    usuarios:HashMap<u64,Usuario>,
    transacciones:Vec<Transaccion>,
    criptomonedas:HashMap<String,Criptomoneda>, //clave=prefijo  
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
impl Plataforma{

    fn inner_new(path:String)->Plataforma{
        let criptomonedas=Self::cargar_lista_criptos();

        Plataforma { usuarios: HashMap::new(),transacciones: Vec::new(),criptomonedas:criptomonedas, path_archivo:path}
    }

    pub fn new()->Plataforma{
        Self::inner_new("src/tp5/archivo_plataform_cripto".to_string())      
    }

    fn cargar_lista_criptos()->HashMap<String,Criptomoneda>{
        let mut criptomonedas = HashMap::new();

        let btc = Criptomoneda::new("Bitcoin".to_string(),
        "BTC".to_string(),
        vec![Blockchain::new("BitcoinNetwork".to_string(), "BTC".to_string())],
        110000.0);

        let eth = Criptomoneda::new("Ethereum".to_string(),
            "ETH".to_string(),
            vec![Blockchain::new("EthereumNetwork".to_string(), "ETH".to_string()), Blockchain::new("Polygon".to_string(), "MATIC".to_string())],
            3500.0);
        
        let sol=Criptomoneda::new("Solana".to_string(),
            "SOL".to_string(), 
            vec![Blockchain::new("Solana".to_string(), "SOL".to_string())],
        160.0);

        let ada=Criptomoneda::new("Cardano".to_string(),
        "ADA".to_string(),
        vec![Blockchain::new("Cardano".to_string(), "ADA".to_string())],
        0.75);

        criptomonedas.insert(btc.prefijo.clone(), btc);
        criptomonedas.insert(eth.prefijo.clone(), eth);
        criptomonedas.insert(sol.prefijo.clone(), sol);
        criptomonedas.insert(ada.prefijo.clone(), ada);

        criptomonedas
    }
    
    fn escribir_en_archivo_json(&self)->Result<(),Errores>{
        let mut archivo=File::create(&self.path_archivo).map_err(|_|Errores::ErrorDeCreacionDeArchivo)?;
        let datos_serializados=serde_json::to_string(&self).map_err(|_|Errores::ErrorDeSerializacionDeDatos)?;
        archivo.write_all(datos_serializados.as_bytes()).map_err(|_|Errores::ErrorDeEscrituraDeDatos)?;
        Ok(())
    }

    fn registrar_usuario(&mut self, usuario:Usuario){
        self.usuarios.insert(usuario.dni,usuario);
    }

    fn ingresar_dinero(&mut self,dni_usuario:u64, monto:f64)->Result<(),String>{
        if monto<=0.0{
            return Err("El monto a ingresar debe ser mayor a 0.0".to_string());
        }

        if self.usuarios.get_mut(&dni_usuario).is_none(){
            return Err("Usuario no encontrado".to_string());
        }

        let back_up=self.clone();
        let user=self.usuarios.get_mut(&dni_usuario).unwrap();
        user.sumar_balance_fiat(monto);
        let dni=user.dni;
        self.agregar_transaccion(monto, TipoTransaccion::IngresoDeFiat,dni);
        if let Err(err)=self.escribir_en_archivo_json(){
            *self=back_up;
            return Err(err.to_string());
        }

        Ok(())
        
    }

    fn agregar_transaccion(&mut self, monto:f64,tipo:TipoTransaccion,dni:u64){
        let transaccion=Transaccion::new(Fecha::fecha_actual(), tipo, monto, dni);
        self.transacciones.push(transaccion);
    }

    fn buscar_usuario(&self,dni_usuario:u64)->Result<&Usuario,String>{
        if let Some (user)=self.usuarios.get(&dni_usuario){
            return Ok(user);
        }
        Err("Usuario no encontrado".to_string())
    }

    fn comprar_cripto(&mut self,dni_usuario:u64,monto_fiat:f64,prefijo_cripto:&str)->Result<(),String>{
        let back_up=self.clone();
        let usuario=match self.usuarios.get_mut(&dni_usuario){
            Some(usuario)=>usuario,
            None=>return Err("Usuario no encontrado".to_string()),
        };

        let cotizacion=match self.criptomonedas.get(prefijo_cripto){
            Some(cripto)=>cripto.cotizacion,
            None=> return Err("No hay cotizacion para la moneda ingresada".to_string()),
        };

        usuario.verificar_estado_con_fiat(monto_fiat)?;

        let monto_cripto=monto_fiat/cotizacion;

        usuario.balance_fiat-=monto_fiat;
        *usuario.balance_cripto.entry(prefijo_cripto.to_string()).or_insert(0.0)+=monto_cripto;
        
        self.agregar_transaccion(monto_fiat,TipoTransaccion::CompraDeCripto { criptomoneda: prefijo_cripto.to_string(), cotizacion}, dni_usuario);
        if let Err(err)=self.escribir_en_archivo_json(){
            *self=back_up;
            return Err(err.to_string());
        }

        Ok(())
    }

    fn vender_cripto(&mut self, dni_usuario:u64,monto_cripto:f64,prefijo_cripto:&str)->Result<(),String>{
        let back_up=self.clone();
        let usuario=match self.usuarios.get_mut(&dni_usuario){
            Some(usuario)=>usuario,
            None=>return Err("Usuario no encontrado".to_string()),
        };

        let cotizacion=match self.criptomonedas.get(prefijo_cripto){
            Some(cripto)=>cripto.cotizacion,
            None=> return Err("No hay cotizacion para la moneda ingresada".to_string()),
        };

        usuario.verificar_estado_con_cripto(monto_cripto,prefijo_cripto)?;

        let monto_fiat=monto_cripto*cotizacion;

        usuario.balance_fiat+=monto_fiat;
        let cripto=usuario.balance_cripto.get_mut(prefijo_cripto).unwrap();
        *cripto-=monto_cripto;

        self.agregar_transaccion(monto_cripto,TipoTransaccion::VentaDeCripto { criptomoneda: prefijo_cripto.to_string(), cotizacion }, dni_usuario);
        if let Err(err)=self.escribir_en_archivo_json(){
            *self=back_up;
            return Err(err.to_string());
        }
        
        Ok(())
    }

    fn retirar_cripto(&mut self, dni_usuario:u64,monto_cripto:f64,prefijo_cripto:&str,nombre_blockchain:&str)->Result<(),String>{
        let back_up=self.clone();
        let usuario=match self.usuarios.get_mut(&dni_usuario){
            Some(usuario)=>usuario,
            None=>return Err("Usuario no encontrado".to_string()),
        };

        let criptomoneda=match self.criptomonedas.get(prefijo_cripto){
            Some(criptomoneda)=>criptomoneda,
            None=>return Err("Criptomoneda no disponible ".to_string()),
        };

        
        usuario.verificar_estado_con_cripto(monto_cripto,prefijo_cripto)?;
        let blockchain=criptomoneda.obtener_blockchain(nombre_blockchain)?;
        let cotizacion=criptomoneda.cotizacion;

        let hash = format!("{}{}", blockchain, rand::random_range(10000..99999));

        let cripto=usuario.balance_cripto.get_mut(prefijo_cripto).unwrap();
        *cripto-=monto_cripto;
        

        self.agregar_transaccion(monto_cripto
            , TipoTransaccion::RetiroDeCripto { criptomoneda: prefijo_cripto.to_string(), cotizacion, blockchain, hash }
            , dni_usuario);
        
        if let Err(err)=self.escribir_en_archivo_json(){
            *self=back_up;
            return Err(err.to_string());
        }
        Ok(())
    }

    fn recibir_cripto(&mut self,dni_usuario:u64,monto_cripto:f64,prefijo_cripto:&str,nombre_blockchain:&str)->Result<(),String>{
        let back_up=self.clone();
        let usuario=match self.usuarios.get_mut(&dni_usuario){
            Some(usuario)=>usuario,
            None=>return Err("Usuario no encontrado".to_string()),
        };

        let criptomoneda=match self.criptomonedas.get(prefijo_cripto){
            Some(criptomoneda)=>criptomoneda,
            None=>return Err("Criptomoneda no disponible ".to_string()),
        };

        let blockchain=criptomoneda.obtener_blockchain(nombre_blockchain)?;
        let cotizacion=criptomoneda.cotizacion;

        *usuario.balance_cripto.entry(prefijo_cripto.to_string()).or_insert(0.0)+=monto_cripto;
        self.agregar_transaccion(monto_cripto,TipoTransaccion::RecepcionDeCripto { cotizacion , blockchain }, dni_usuario);

        if let Err(err)=self.escribir_en_archivo_json(){
            *self=back_up;
            return Err(err.to_string());
        }

        Ok(())
    }

    fn retirar_fiat(&mut self, dni_usuario:u64,monto_fiat:f64,medio:MedioDeRetiro)->Result<(),String>{
        let back_up=self.clone();
        let usuario=match self.usuarios.get_mut(&dni_usuario){
            Some(usuario)=>usuario,
            None=>return Err("Usuario no encontrado".to_string()),
        };

        usuario.verificar_estado_con_fiat(monto_fiat)?;

        usuario.balance_fiat-=monto_fiat;
        self.agregar_transaccion(monto_fiat,TipoTransaccion::RetiroFiat { medio }, dni_usuario);

        if let Err(err)=self.escribir_en_archivo_json(){
            *self=back_up;
            return Err(err.to_string());
        }

        Ok(())
    }

    fn cripto_mas_vendida(&self)->Result<Criptomoneda,String>{
        if self.transacciones.len()==0{
            return Err("No se realizaron transacciones".to_string());
        }

        let mut contador=HashMap::new();

        self.transacciones.iter().for_each(|t| { 
            if let TipoTransaccion::VentaDeCripto { criptomoneda ,..}=&t.tipo{
                *contador.entry(criptomoneda.clone()).or_insert(0)+=1;
            }
        });

        if contador.is_empty(){
            return Err("No se realizaron ventas".to_string());
        }

        let max_nombre=contador.iter().max_by_key(|(_,cant)|*cant)
                .map(|(prefijo,_)|prefijo.clone()).unwrap();
        let resultado=self.criptomonedas.get(&max_nombre).unwrap();

        Ok(resultado.clone())
    }

    fn cripto_mas_comprada(&self)->Result<Criptomoneda,String>{
        if self.transacciones.len()==0{
            return Err("No se realizaron transacciones".to_string());
        }

        let mut contador=HashMap::new();

        self.transacciones.iter().for_each(|t| { 
            if let TipoTransaccion::CompraDeCripto { criptomoneda ,..}=&t.tipo{
                *contador.entry(criptomoneda.clone()).or_insert(0)+=1;
            }
        });

        if contador.is_empty(){
            return Err("No se realizaron compras".to_string());
        }

        let max_nombre=contador.iter().max_by_key(|(_,cant)|*cant)
                .map(|(prefijo,_)|prefijo.clone()).unwrap();
        let resultado=self.criptomonedas.get(&max_nombre).unwrap();

        Ok(resultado.clone())
    }


    fn cripto_mayor_volumen_ventas(&self)->Result<Criptomoneda,String>{
        if self.transacciones.len()==0{
            return Err("No se realizaron transacciones".to_string());
        }

        let mut contador=HashMap::new();
        self.transacciones.iter().for_each(|t|
            if let TipoTransaccion::VentaDeCripto { criptomoneda, .. }=&t.tipo{
                *contador.entry(criptomoneda.clone()).or_insert(0.0)+=t.monto;
            });

        if contador.is_empty(){
            return Err("No se realizaron ventas".to_string());
        }

        let maximo=contador.iter()
                .max_by(|cant1,cant2|cant1.1.partial_cmp(&cant2.1).unwrap())
                .map(|(prefijo,_)|prefijo.clone()).unwrap();

        let resultado=self.criptomonedas.get(&maximo).unwrap();
        Ok(resultado.clone())
    }

    fn cripto_mayor_volumen_compras(&self)->Result<Criptomoneda,String>{
        if self.transacciones.len()==0{
            return Err("No se realizaron transacciones".to_string());
        }

        let mut contador=HashMap::new();
        self.transacciones.iter().for_each(|t|
            if let TipoTransaccion::CompraDeCripto { criptomoneda, .. }=&t.tipo{
                *contador.entry(criptomoneda.clone()).or_insert(0.0)+=t.monto;
            });

        if contador.is_empty(){
            return Err("No se realizaron compras".to_string());
        }

        let maximo=contador.iter()
                .max_by(|cant1,cant2|cant1.1.partial_cmp(&cant2.1).unwrap())
                .map(|(prefijo,_)|prefijo.clone()).unwrap();

        let resultado=self.criptomonedas.get(&maximo).unwrap();
        Ok(resultado.clone())
    }

}

#[allow(unused)]
impl Usuario{
    fn new(nombre:String,apellido:String,email:String,dni:u64,validacion:bool)->Usuario{
        Usuario { nombre, apellido, email, dni, identidad_validada: validacion, balance_fiat: 0.0, balance_cripto: HashMap::new() }
    }

    fn sumar_balance_fiat(&mut self,monto:f64){
        self.balance_fiat+=monto;
    }
    
    fn verificar_estado_con_fiat(&self, monto:f64)->Result<(),String>{
        if !self.identidad_validada{
            return Err("El usuario no tiene la identidad validada".to_string());
        }

        if self.balance_fiat<monto{
            return Err("Saldo fiat insuficiente".to_string());
        }

        Ok(())
    }

    fn verificar_estado_con_cripto(&self, monto_cripto:f64,prefijo_cripto:&str)->Result<(),String>{
        if !self.identidad_validada{
            return Err("El usuario no tiene la identidad validada".to_string());
        }

        if let Some(balance)=self.balance_cripto.get(prefijo_cripto){
            if balance<&monto_cripto{
                return Err("Saldo cripto insuficiente".to_string());
            }
        }else{
            return Err("No se encontró la criptomoneda en el balance".to_string());
        }
        Ok(())
    }
    
}

#[allow(unused)]
impl Criptomoneda{
    fn new(nombre:String,prefijo:String,vec_blockchains:Vec<Blockchain>,cotizacion:f64)->Criptomoneda{
        Criptomoneda { nombre, prefijo, blockchains:vec_blockchains, cotizacion }
    }

    fn obtener_blockchain(&self,nombre_blockchain:&str)->Result<String,String>{
        match self.blockchains.iter().find(|b|b.nombre==nombre_blockchain){
            Some(blockchain)=>Ok(blockchain.nombre.clone()),
            None=>Err("Blockchain no encontrado".to_string()),
        }
    }
}

#[allow(unused)]
impl Blockchain{
    fn new(nombre:String,prefijo:String)->Blockchain{
        Blockchain{ nombre, prefijo }
    }
}

#[allow(unused)]
impl Transaccion{
    fn new(fecha:Fecha,tipo:TipoTransaccion,monto:f64,usuario:u64)->Transaccion{
        Transaccion { fecha, tipo, monto, usuario}
    }

}

#[cfg(test)]
mod test{
    use crate::{tp5::fecha::Fecha, tp5::ej6::{Blockchain, Criptomoneda, MedioDeRetiro, Plataforma, TipoTransaccion, Transaccion, Usuario}};
    use std::fs;

    #[test]
    fn ingresar_dinero_test(){
        let mut plataforma=Plataforma::new();

        let usuario=Usuario::new("Juan".to_string(), "Perez".to_string(), "jp@gmail.com".to_string(), 2222222, true);
        let dni_usuario=usuario.dni;
        plataforma.registrar_usuario(usuario);
        
        let _=plataforma.ingresar_dinero(dni_usuario,500.0);
        let user=plataforma.buscar_usuario(dni_usuario).unwrap();

        let transaccion=Transaccion::new(Fecha::fecha_actual(),
                         TipoTransaccion::IngresoDeFiat,
                          500.0, dni_usuario);

        assert_eq!(user.balance_fiat,500.0);
        assert_eq!(plataforma.transacciones[0],transaccion);
    }

    #[test]
    fn ingresar_dinero_sin_usuarios_test(){
        let mut plataforma=Plataforma::new();
        let dni_usuario=12345;
        
        let resultado=plataforma.ingresar_dinero(dni_usuario,500.0);

        assert_eq!(resultado,Err("Usuario no encontrado".to_string()));
    }

    #[test]
    fn comprar_cripto_test(){
        let mut plataforma=Plataforma::new();

        let usuario=Usuario::new("Juan".to_string(), "Perez".to_string(), "jp@gmail.com".to_string(), 2222222, true);
        let dni_usuario=usuario.dni;
        plataforma.registrar_usuario(usuario);
        let _=plataforma.ingresar_dinero(dni_usuario,1000.0);

        let _=plataforma.comprar_cripto(dni_usuario, 350.0, "ETH");

        let user=plataforma.buscar_usuario(dni_usuario).unwrap();
        let transaccion=Transaccion::new(Fecha::fecha_actual(),
                         TipoTransaccion::CompraDeCripto { criptomoneda: "ETH".to_string(), cotizacion:3500.0},
                          350.0, dni_usuario);

        assert_eq!(user.balance_fiat,650.0);
        assert_eq!(user.balance_cripto.get("ETH"),Some(&0.1));
        assert_eq!(plataforma.transacciones[1],transaccion);
    }

    #[test]
    fn comprar_cripto_usuario_inxistente_test(){
        let mut plataforma=Plataforma::new();

        let usuario=Usuario::new("Juan".to_string(), "Perez".to_string(), "jp@gmail.com".to_string(), 2222222, true);
        plataforma.registrar_usuario(usuario);

        assert_eq!(plataforma.comprar_cripto(12345678, 350.0, "ETH"),Err("Usuario no encontrado".to_string()));
    }

    #[test]
    fn comprar_cripto_sin_cotizacion_test(){
        let mut plataforma=Plataforma::new();

        let usuario=Usuario::new("Juan".to_string(), "Perez".to_string(), "jp@gmail.com".to_string(), 2222222, true);
        let dni_usuario=usuario.dni;
        plataforma.registrar_usuario(usuario);
        let _=plataforma.ingresar_dinero(dni_usuario,1000.0);

        assert_eq!(plataforma.comprar_cripto(dni_usuario, 100.0, "DOGE"),Err("No hay cotizacion para la moneda ingresada".to_string()));
    }

    #[test]
    fn comprar_cripto_identidad_no_validada_test(){
        let mut plataforma=Plataforma::new();

        let usuario=Usuario::new("Juan".to_string(), "Perez".to_string(), "jp@gmail.com".to_string(), 2222222, false);
        let dni_usuario=usuario.dni;
        plataforma.registrar_usuario(usuario);
        let _=plataforma.ingresar_dinero(dni_usuario,1000.0);

        assert_eq!(plataforma.comprar_cripto(dni_usuario, 100.0, "ETH"),Err("El usuario no tiene la identidad validada".to_string()));
    }

    #[test]
    fn comprar_cripto_balance_fiat_insuficiente_test(){
        let mut plataforma=Plataforma::new();

        let usuario=Usuario::new("Juan".to_string(), "Perez".to_string(), "jp@gmail.com".to_string(), 2222222, true);
        let dni_usuario=usuario.dni;
        plataforma.registrar_usuario(usuario);
        let _=plataforma.ingresar_dinero(dni_usuario,100.0);

        assert_eq!(plataforma.comprar_cripto(dni_usuario, 200.0, "ETH"),Err("Saldo fiat insuficiente".to_string()));
    }

    #[test]
    fn comprar_cripto_plataforma_vacia_test(){
        let mut plataforma=Plataforma::new();

        assert_eq!(plataforma.comprar_cripto(2222222, 200.0, "ETH"),Err("Usuario no encontrado".to_string()));
    }

     #[test]
    fn vender_cripto_test(){
        let mut plataforma=Plataforma::new();

        let usuario=Usuario::new("Juan".to_string(), "Perez".to_string(), "jp@gmail.com".to_string(), 2222222, true);
        let dni_usuario=usuario.dni;
        plataforma.registrar_usuario(usuario);
        let _=plataforma.ingresar_dinero(dni_usuario,1000.0);
        let _=plataforma.comprar_cripto(dni_usuario, 350.0, "ETH");

        let _=plataforma.vender_cripto(dni_usuario, 0.05,"ETH");

        let user=plataforma.buscar_usuario(dni_usuario).unwrap();
        let transaccion=Transaccion::new(Fecha::fecha_actual(),
                         TipoTransaccion::VentaDeCripto  { criptomoneda: "ETH".to_string(), cotizacion:3500.0},
                          0.05, dni_usuario);

        assert_eq!(user.balance_fiat,825.0);
        assert_eq!(user.balance_cripto.get("ETH"),Some(&0.05));
        assert_eq!(plataforma.transacciones[2],transaccion);
    }

    #[test]
    fn vender_cripto_usuario_inxistente_test(){
        let mut plataforma=Plataforma::new();

        let usuario=Usuario::new("Juan".to_string(), "Perez".to_string(), "jp@gmail.com".to_string(), 2222222, true);
        plataforma.registrar_usuario(usuario);

        assert_eq!(plataforma.vender_cripto(12345678, 1.0, "ETH"),Err("Usuario no encontrado".to_string()));
    }

    #[test]
    fn vender_cripto_sin_cotizacion_test(){
        let mut plataforma=Plataforma::new();

        let usuario=Usuario::new("Juan".to_string(), "Perez".to_string(), "jp@gmail.com".to_string(), 2222222, true);
        let dni_usuario=usuario.dni;
        plataforma.registrar_usuario(usuario);
        let _=plataforma.ingresar_dinero(dni_usuario,1000.0);

        assert_eq!(plataforma.vender_cripto(dni_usuario, 10.0, "DOGE"),Err("No hay cotizacion para la moneda ingresada".to_string()));
    }

    #[test]
    fn vender_cripto_identidad_no_validada_test(){
        let mut plataforma=Plataforma::new();

        let usuario=Usuario::new("Juan".to_string(), "Perez".to_string(), "jp@gmail.com".to_string(), 2222222, false);
        let dni_usuario=usuario.dni;
        plataforma.registrar_usuario(usuario);
        let _=plataforma.ingresar_dinero(dni_usuario,1000.0);

        assert_eq!(plataforma.vender_cripto(dni_usuario, 1.0, "ETH"),Err("El usuario no tiene la identidad validada".to_string()));
    }

    #[test]
    fn vender_cripto_balance_cripto_insuficiente_test(){
        let mut plataforma=Plataforma::new();

        let usuario=Usuario::new("Juan".to_string(), "Perez".to_string(), "jp@gmail.com".to_string(), 2222222, true);
        let dni_usuario=usuario.dni;
        plataforma.registrar_usuario(usuario);
        let _=plataforma.ingresar_dinero(dni_usuario,1000.0);
        let _=plataforma.comprar_cripto(dni_usuario, 350.0, "ETH");

        assert_eq!(plataforma.vender_cripto(dni_usuario, 2.0, "ETH"),Err("Saldo cripto insuficiente".to_string()));
    }

    #[test]
    fn vender_cripto_balance_cripto_ausente_en_balance_test(){
        let mut plataforma=Plataforma::new();

        let usuario=Usuario::new("Juan".to_string(), "Perez".to_string(), "jp@gmail.com".to_string(), 2222222, true);
        let dni_usuario=usuario.dni;
        plataforma.registrar_usuario(usuario);
        let _=plataforma.ingresar_dinero(dni_usuario,1000.0);
        let _=plataforma.comprar_cripto(dni_usuario, 350.0, "ETH");

        assert_eq!(plataforma.vender_cripto(dni_usuario, 1.0, "BTC"),Err("No se encontró la criptomoneda en el balance".to_string()));
    }

    #[test]
    fn vender_cripto_plataforma_vacia_test(){
        let mut plataforma=Plataforma::new();

        assert_eq!(plataforma.vender_cripto(2222222, 200.0, "ETH"),Err("Usuario no encontrado".to_string()));
    }

    #[test]
    fn retirar_cripto_test(){
        let mut plataforma=Plataforma::new();

        let usuario=Usuario::new("Juan".to_string(), "Perez".to_string(), "jp@gmail.com".to_string(), 2222222, true);
        let dni_usuario=usuario.dni;
        plataforma.registrar_usuario(usuario);
        let _=plataforma.ingresar_dinero(dni_usuario,1000.0);
        let _=plataforma.comprar_cripto(dni_usuario, 350.0, "ETH");

        let resultado=plataforma.retirar_cripto(dni_usuario, 0.04, "ETH", "EthereumNetwork");
        let user=plataforma.buscar_usuario(dni_usuario).unwrap();

        assert_eq!(resultado,Ok(()));
        assert_eq!(plataforma.transacciones.len(),3);
        assert_eq!(user.balance_fiat,650.0);

        let balance_cripto_usuario=user.balance_cripto.get("ETH").unwrap();
        assert!((balance_cripto_usuario - 0.06).abs() < 0.0001);
    }

    #[test]
    fn retirar_cripto_usuario_inexistente_test(){
        let mut plataforma=Plataforma::new();

        let usuario=Usuario::new("Juan".to_string(), "Perez".to_string(), "jp@gmail.com".to_string(), 2222222, true);
        plataforma.registrar_usuario(usuario);

        assert_eq!(plataforma.retirar_cripto(12345678, 1.0, "ETH","EthereumNetwork"),Err("Usuario no encontrado".to_string()));
    }

    #[test]
    fn retirar_cripto_moneda_no_disponible_test(){
        let mut plataforma=Plataforma::new();

        let usuario=Usuario::new("Juan".to_string(), "Perez".to_string(), "jp@gmail.com".to_string(), 2222222, true);
        let dni_usuario=usuario.dni;
        plataforma.registrar_usuario(usuario);
        let _=plataforma.ingresar_dinero(dni_usuario,1000.0);

        assert_eq!(plataforma.retirar_cripto(dni_usuario, 10.0, "DOGE","DogecoinNetwork"),Err("Criptomoneda no disponible ".to_string()));
    }

    #[test]
    fn retirar_cripto_identidad_no_validada_test(){
        let mut plataforma=Plataforma::new();

        let usuario=Usuario::new("Juan".to_string(), "Perez".to_string(), "jp@gmail.com".to_string(), 2222222, false);
        let dni_usuario=usuario.dni;
        plataforma.registrar_usuario(usuario);
        let _=plataforma.ingresar_dinero(dni_usuario,1000.0);

        assert_eq!(plataforma.retirar_cripto(dni_usuario, 1.0, "ETH","EthereumNetwork"),Err("El usuario no tiene la identidad validada".to_string()));
    }

    #[test]
    fn retirar_cripto_balance_cripto_insuficiente_test(){
        let mut plataforma=Plataforma::new();

        let usuario=Usuario::new("Juan".to_string(), "Perez".to_string(), "jp@gmail.com".to_string(), 2222222, true);
        let dni_usuario=usuario.dni;
        plataforma.registrar_usuario(usuario);
        let _=plataforma.ingresar_dinero(dni_usuario,1000.0);
        let _=plataforma.comprar_cripto(dni_usuario, 350.0, "ETH");

        assert_eq!(plataforma.retirar_cripto(dni_usuario, 2.0, "ETH","EthereumNetwork"),Err("Saldo cripto insuficiente".to_string()));
    }

    #[test]
    fn retirar_cripto_balance_cripto_ausente_en_balance_test(){
        let mut plataforma=Plataforma::new();

        let usuario=Usuario::new("Juan".to_string(), "Perez".to_string(), "jp@gmail.com".to_string(), 2222222, true);
        let dni_usuario=usuario.dni;
        plataforma.registrar_usuario(usuario);
        let _=plataforma.ingresar_dinero(dni_usuario,1000.0);
        let _=plataforma.comprar_cripto(dni_usuario, 350.0, "ETH");

        assert_eq!(plataforma.retirar_cripto(dni_usuario, 1.0, "BTC","EthereumNetwork"),Err("No se encontró la criptomoneda en el balance".to_string()));
    }

    #[test]
    fn retirar_cripto_balance_blockchain_ausente_en_cripto_test(){
        let mut plataforma=Plataforma::new();

        let usuario=Usuario::new("Juan".to_string(), "Perez".to_string(), "jp@gmail.com".to_string(), 2222222, true);
        let dni_usuario=usuario.dni;
        plataforma.registrar_usuario(usuario);
        let _=plataforma.ingresar_dinero(dni_usuario,1000.0);
        let _=plataforma.comprar_cripto(dni_usuario, 350.0, "ETH");

        assert_eq!(plataforma.retirar_cripto(dni_usuario, 0.05, "ETH","BitcoinNetwork"),Err("Blockchain no encontrado".to_string()));
    }

    #[test]
    fn retirar_cripto_plataforma_vacia_test(){
        let mut plataforma=Plataforma::new();

        assert_eq!(plataforma.retirar_cripto(2222222, 200.0, "ETH","EthereumNetwork"),Err("Usuario no encontrado".to_string()));
    }

    #[test]
    fn recibir_cripto_moneda_existente_en_balance_test(){
        let mut plataforma=Plataforma::new();

        let usuario=Usuario::new("Juan".to_string(), "Perez".to_string(), "jp@gmail.com".to_string(), 2222222, true);
        let dni_usuario=usuario.dni;
        plataforma.registrar_usuario(usuario);
        let _=plataforma.ingresar_dinero(dni_usuario,1000.0);
        let _=plataforma.comprar_cripto(dni_usuario, 350.0, "ETH");

        let resultado=plataforma.recibir_cripto(dni_usuario, 0.15, "ETH", "EthereumNetwork");
        let user=plataforma.buscar_usuario(dni_usuario).unwrap();

        assert_eq!(resultado,Ok(()));

        let transaccion=Transaccion::new(Fecha::fecha_actual(),
             TipoTransaccion::RecepcionDeCripto { cotizacion: 3500.0, blockchain:"EthereumNetwork".to_string() }, 
             0.15, dni_usuario);
        assert_eq!(plataforma.transacciones[2],transaccion);
        assert_eq!(user.balance_cripto.get("ETH"),Some(&0.25));

    }

    #[test]
    fn recibir_cripto_moneda_inexistente_en_balance_test(){
        let mut plataforma=Plataforma::new();

        let usuario=Usuario::new("Juan".to_string(), "Perez".to_string(), "jp@gmail.com".to_string(), 2222222, true);
        let dni_usuario=usuario.dni;
        plataforma.registrar_usuario(usuario);
        let _=plataforma.ingresar_dinero(dni_usuario,1000.0);
        let _=plataforma.comprar_cripto(dni_usuario, 350.0, "ETH");

        let resultado=plataforma.recibir_cripto(dni_usuario, 0.005, "BTC", "BitcoinNetwork");
        let user=plataforma.buscar_usuario(dni_usuario).unwrap();

        assert_eq!(resultado,Ok(()));

        let transaccion=Transaccion::new(Fecha::fecha_actual(),
             TipoTransaccion::RecepcionDeCripto { cotizacion: 110000.0, blockchain:"BitcoinNetwork".to_string() }, 
             0.005, dni_usuario);
        assert_eq!(plataforma.transacciones[2],transaccion);
        assert_eq!(user.balance_cripto.get("BTC"),Some(&0.005));
    }

    #[test]
    fn recibir_cripto_usuario_inxistente_test(){
        let mut plataforma=Plataforma::new();

        let usuario=Usuario::new("Juan".to_string(), "Perez".to_string(), "jp@gmail.com".to_string(), 2222222, true);
        plataforma.registrar_usuario(usuario);

        assert_eq!(plataforma.recibir_cripto(12345678, 1.0, "ETH","EthereumNetwork"),Err("Usuario no encontrado".to_string()));
    }

    #[test]
    fn recibir_criptomoneda_no_disponible_test(){
        let mut plataforma=Plataforma::new();

        let usuario=Usuario::new("Juan".to_string(), "Perez".to_string(), "jp@gmail.com".to_string(), 2222222, true);
        let dni_usuario=usuario.dni;
        plataforma.registrar_usuario(usuario);

        assert_eq!(plataforma.recibir_cripto(dni_usuario, 10.0, "DOGE","DogecoinNetwork"),Err("Criptomoneda no disponible ".to_string()));
    }

    #[test]
    fn recibir_cripto_balance_blockchain_ausente_en_cripto_test(){
        let mut plataforma=Plataforma::new();

        let usuario=Usuario::new("Juan".to_string(), "Perez".to_string(), "jp@gmail.com".to_string(), 2222222, true);
        let dni_usuario=usuario.dni;
        plataforma.registrar_usuario(usuario);

        assert_eq!(plataforma.recibir_cripto(dni_usuario, 0.05, "ETH","BitcoinNetwork"),Err("Blockchain no encontrado".to_string()));
    }

    #[test]
    fn recibir_cripto_plataforma_vacia_test(){
        let mut plataforma=Plataforma::new();

        assert_eq!(plataforma.recibir_cripto(2222222, 200.0, "ETH","EthereumNetwork"),Err("Usuario no encontrado".to_string()));
    }

    #[test]
    fn retirar_fiat_tranferencia_bancaria_test(){
        let mut plataforma=Plataforma::new();

        let usuario=Usuario::new("Juan".to_string(), "Perez".to_string(), "jp@gmail.com".to_string(), 2222222, true);
        let dni_usuario=usuario.dni;
        plataforma.registrar_usuario(usuario);
        let _=plataforma.ingresar_dinero(dni_usuario,650.0);

        let resultado=plataforma.retirar_fiat(dni_usuario, 200.0,
            MedioDeRetiro::TransferenciaBancaria { cbu: 123456, alias: "juan.mp".to_string()});

        assert_eq!(resultado,Ok(()));

        let user=plataforma.buscar_usuario(dni_usuario).unwrap();
        let transaccion=Transaccion::new(Fecha::fecha_actual(),
            TipoTransaccion::RetiroFiat { medio:  MedioDeRetiro::TransferenciaBancaria { cbu: 123456, alias: "juan.mp".to_string()}},
             200.0, dni_usuario);

        assert_eq!(user.balance_fiat,450.0);
        assert_eq!(plataforma.transacciones[1],transaccion);
    
    }
    #[test]
    fn retirar_fiat_mercado_pago_test(){
        let mut plataforma=Plataforma::new();

        let usuario=Usuario::new("Juan".to_string(), "Perez".to_string(), "jp@gmail.com".to_string(), 2222222, true);
        let dni_usuario=usuario.dni;
        plataforma.registrar_usuario(usuario);
        let _=plataforma.ingresar_dinero(dni_usuario,650.0);

        let resultado=plataforma.retirar_fiat(dni_usuario, 200.0,
            MedioDeRetiro::MercadoPago { cvu: 123456, alias: "juan.mp".to_string()});

        assert_eq!(resultado,Ok(()));

        let user=plataforma.buscar_usuario(dni_usuario).unwrap();
        let transaccion=Transaccion::new(Fecha::fecha_actual(),
            TipoTransaccion::RetiroFiat { medio:  MedioDeRetiro::MercadoPago { cvu: 123456, alias: "juan.mp".to_string()}},
             200.0, dni_usuario);

        assert_eq!(user.balance_fiat,450.0);
        assert_eq!(plataforma.transacciones[1],transaccion);
    }

    #[test]
    fn retirar_fiat_usuario_inxistente_test(){
        let mut plataforma=Plataforma::new();

        let usuario=Usuario::new("Juan".to_string(), "Perez".to_string(), "jp@gmail.com".to_string(), 2222222, true);
        plataforma.registrar_usuario(usuario);

        assert_eq!(plataforma.retirar_fiat(12345678, 200.0,MedioDeRetiro::MercadoPago { cvu: 123456, alias: "juan.mp".to_string()}),Err("Usuario no encontrado".to_string()));
    }

    #[test]
    fn retirar_fiat_identidad_no_validada_test(){
        let mut plataforma=Plataforma::new();

        let usuario=Usuario::new("Juan".to_string(), "Perez".to_string(), "jp@gmail.com".to_string(), 2222222, false);
        let dni_usuario=usuario.dni;
        plataforma.registrar_usuario(usuario);
        let _=plataforma.ingresar_dinero(dni_usuario,1000.0);

        assert_eq!(plataforma.retirar_fiat(dni_usuario, 200.0, MedioDeRetiro::MercadoPago { cvu: 123456, alias: "juan.mp".to_string()}),Err("El usuario no tiene la identidad validada".to_string()));
    }

    #[test]
    fn retirar_fiat_balance_fiat_insuficiente_test(){
        let mut plataforma=Plataforma::new();

        let usuario=Usuario::new("Juan".to_string(), "Perez".to_string(), "jp@gmail.com".to_string(), 2222222, true);
        let dni_usuario=usuario.dni;
        plataforma.registrar_usuario(usuario);
        let _=plataforma.ingresar_dinero(dni_usuario,100.0);

        assert_eq!(plataforma.retirar_fiat(dni_usuario, 200.0,MedioDeRetiro::MercadoPago { cvu: 123456, alias: "juan.mp".to_string()}),Err("Saldo fiat insuficiente".to_string()));
    }
   
    #[test]
    fn retirar_fiat_plataforma_vacia_test(){
        let mut plataforma=Plataforma::new();

        assert_eq!(plataforma.retirar_fiat(1234567, 200.0,MedioDeRetiro::MercadoPago { cvu: 123456, alias: "juan.mp".to_string()}),Err("Usuario no encontrado".to_string()));
    }

    #[test]
    fn cripto_mas_vendida_test(){
        let mut plataforma=Plataforma::new();

        let usuario=Usuario::new("Juan".to_string(), "Perez".to_string(), "jp@gmail.com".to_string(), 2222222, true);
        let dni_usuario=usuario.dni;
        plataforma.registrar_usuario(usuario);
        let _=plataforma.ingresar_dinero(dni_usuario,200000.0);
        let _=plataforma.comprar_cripto(dni_usuario, 3500.0, "ETH");
        let _= plataforma.comprar_cripto(dni_usuario, 110000.0, "BTC");
        
        let _=plataforma.vender_cripto(dni_usuario, 0.04, "ETH");

        let _ = plataforma.vender_cripto(dni_usuario, 0.1, "ETH");
        let _ = plataforma.vender_cripto(dni_usuario, 0.1, "ETH");
        let _ = plataforma.vender_cripto(dni_usuario, 0.1, "BTC");

        let resultado = plataforma.cripto_mas_vendida();

        let cripto=Criptomoneda::new("Ethereum".to_string(),
            "ETH".to_string(),
            vec![Blockchain::new("EthereumNetwork".to_string(), "ETH".to_string()), Blockchain::new("Polygon".to_string(), "MATIC".to_string())],
            3500.0);
    
        assert_eq!(resultado,Ok(cripto));
    }

    #[test]
    fn cripto_mas_vendida_sin_ventas_test(){
        let mut plataforma=Plataforma::new();

        let usuario=Usuario::new("Juan".to_string(), "Perez".to_string(), "jp@gmail.com".to_string(), 2222222, true);
        let dni_usuario=usuario.dni;
        plataforma.registrar_usuario(usuario);
        let _=plataforma.ingresar_dinero(dni_usuario,200000.0);
        let _=plataforma.comprar_cripto(dni_usuario, 3500.0, "ETH");
        let _= plataforma.comprar_cripto(dni_usuario, 110000.0, "BTC");

        let resultado=plataforma.cripto_mas_vendida();

        assert_eq!(resultado,Err("No se realizaron ventas".to_string()));
    }

    #[test]
    fn cripto_mas_vendida_sin_transacciones_test(){
        let plataforma=Plataforma::new();

        let resultado=plataforma.cripto_mas_vendida();
        assert_eq!(resultado,Err("No se realizaron transacciones".to_string()));
    }

    #[test]
    fn cripto_mas_comprada_test() {
        let mut plataforma = Plataforma::new();

        let usuario = Usuario::new("Juan".to_string(), "Perez".to_string(), "jp@gmail.com".to_string(), 2222222, true);
        let dni_usuario = usuario.dni;
        plataforma.registrar_usuario(usuario);
        let _ = plataforma.ingresar_dinero(dni_usuario, 100000.0);

        // Compro ETH tres veces, BTC una sola vez -> ETH debería ganar en cantidad
        let _ = plataforma.comprar_cripto(dni_usuario, 350.0, "ETH");
        let _ = plataforma.comprar_cripto(dni_usuario, 350.0, "ETH");
        let _ = plataforma.comprar_cripto(dni_usuario, 350.0, "ETH");
        let _ = plataforma.comprar_cripto(dni_usuario, 1100.0, "BTC");

        let resultado = plataforma.cripto_mas_comprada();
        let cripto=Criptomoneda::new("Ethereum".to_string(),
            "ETH".to_string(),
            vec![Blockchain::new("EthereumNetwork".to_string(), "ETH".to_string()), Blockchain::new("Polygon".to_string(), "MATIC".to_string())],
            3500.0);
        assert_eq!(resultado,Ok(cripto));
    }

    #[test]
    fn cripto_mas_comprada_sin_compras_test(){
        let mut plataforma = Plataforma::new();

        let usuario = Usuario::new("Juan".to_string(), "Perez".to_string(), "jp@gmail.com".to_string(), 2222222, true);
        let dni_usuario = usuario.dni;
        plataforma.registrar_usuario(usuario);
        let _ = plataforma.ingresar_dinero(dni_usuario, 1000.0);
        let _ = plataforma.ingresar_dinero(dni_usuario, 100.0);
        let _=plataforma.retirar_fiat(dni_usuario, 500.0, MedioDeRetiro::MercadoPago { cvu: 123456, alias: "juan.mp".to_string()});

        let resultado = plataforma.cripto_mas_comprada();
        assert_eq!(resultado,Err("No se realizaron compras".to_string()));
    }

    #[test]
    fn cripto_mas_comprada_sin_transacciones_test(){
        let plataforma=Plataforma::new();

        let resultado=plataforma.cripto_mas_comprada();
        assert_eq!(resultado,Err("No se realizaron transacciones".to_string()));
    }

    #[test]
    fn cripto_mayor_volumen_ventas_test(){
        let mut plataforma=Plataforma::new();

        let usuario=Usuario::new("Juan".to_string(), "Perez".to_string(), "jp@gmail.com".to_string(), 2222222, true);
        let dni_usuario=usuario.dni;
        plataforma.registrar_usuario(usuario);
        let _=plataforma.ingresar_dinero(dni_usuario,200000.0);
        let _=plataforma.comprar_cripto(dni_usuario, 3500.0, "ETH");
        let _= plataforma.comprar_cripto(dni_usuario, 110000.0, "BTC");
        
        let _=plataforma.vender_cripto(dni_usuario, 0.04, "ETH");
        let _ = plataforma.vender_cripto(dni_usuario, 0.1, "ETH");
        let _ = plataforma.vender_cripto(dni_usuario, 0.1, "ETH");
        let _ = plataforma.vender_cripto(dni_usuario, 0.1, "BTC");

        let resultado = plataforma.cripto_mayor_volumen_ventas();

        let cripto=Criptomoneda::new("Ethereum".to_string(),
            "ETH".to_string(),
            vec![Blockchain::new("EthereumNetwork".to_string(), "ETH".to_string()), Blockchain::new("Polygon".to_string(), "MATIC".to_string())],
            3500.0);

        assert_eq!(resultado,Ok(cripto));       
    }

     #[test]
    fn cripto_mayor_volumen_ventas_sin_ventas_test(){
        let mut plataforma=Plataforma::new();

        let usuario=Usuario::new("Juan".to_string(), "Perez".to_string(), "jp@gmail.com".to_string(), 2222222, true);
        let dni_usuario=usuario.dni;
        plataforma.registrar_usuario(usuario);
        let _=plataforma.ingresar_dinero(dni_usuario,200000.0);
        let _=plataforma.comprar_cripto(dni_usuario, 3500.0, "ETH");
        let _= plataforma.comprar_cripto(dni_usuario, 110000.0, "BTC");

        let resultado=plataforma.cripto_mayor_volumen_ventas();

        assert_eq!(resultado,Err("No se realizaron ventas".to_string()));
    }

    #[test]
    fn cripto_mayor_volumen_ventas_sin_transacciones_test(){
        let plataforma=Plataforma::new();

        let resultado=plataforma.cripto_mayor_volumen_ventas();
        assert_eq!(resultado,Err("No se realizaron transacciones".to_string()));
    }

    #[test]
    fn cripto_mayor_volumen_compras_test(){
        let mut plataforma = Plataforma::new();

        let usuario = Usuario::new("Juan".to_string(), "Perez".to_string(), "jp@gmail.com".to_string(), 2222222, true);
        let dni_usuario = usuario.dni;
        plataforma.registrar_usuario(usuario);
        let _ = plataforma.ingresar_dinero(dni_usuario, 100000.0);

        // Compro ETH tres veces, BTC una sola vez -> ETH debería ganar en cantidad
        let _ = plataforma.comprar_cripto(dni_usuario, 350.0, "ETH");
        let _ = plataforma.comprar_cripto(dni_usuario, 350.0, "ETH");
        let _ = plataforma.comprar_cripto(dni_usuario, 350.0, "ETH");
        let _ = plataforma.comprar_cripto(dni_usuario, 1100.0, "BTC");

        let resultado = plataforma.cripto_mayor_volumen_compras();
        let cripto=Criptomoneda::new("Bitcoin".to_string(),
            "BTC".to_string(),
            vec![Blockchain::new("BitcoinNetwork".to_string(), "BTC".to_string())],
            110000.0);
        assert_eq!(resultado,Ok(cripto));
    }

    #[test]
    fn cripto_mayor_volumen_compras_sin_compras_test(){
        let mut plataforma = Plataforma::new();

        let usuario = Usuario::new("Juan".to_string(), "Perez".to_string(), "jp@gmail.com".to_string(), 2222222, true);
        let dni_usuario = usuario.dni;
        plataforma.registrar_usuario(usuario);
        let _ = plataforma.ingresar_dinero(dni_usuario, 1000.0);
        let _ = plataforma.ingresar_dinero(dni_usuario, 100.0);
        let _=plataforma.retirar_fiat(dni_usuario, 500.0, MedioDeRetiro::MercadoPago { cvu: 123456, alias: "juan.mp".to_string()});

        let resultado = plataforma.cripto_mas_comprada();
        assert_eq!(resultado,Err("No se realizaron compras".to_string()));
    }

    #[test]
    fn cripto_mayor_volumen_compras_sin_transacciones_test(){
        let plataforma=Plataforma::new();

        let resultado=plataforma.cripto_mas_comprada();
        assert_eq!(resultado,Err("No se realizaron transacciones".to_string()));
    }

    #[test]
    fn ingresar_dinero_fallido_por_falla_en_creacion_de_archivo_test(){
        let path="src/tp5/testeo_de_errores_ingresar_dinero";
        let _ = std::fs::remove_file(path);
        let _ = std::fs::remove_dir(path);
        //El archivo existe pero es un directorio, no se pueden escribir datos en él

        fs::create_dir(path).unwrap();
        let mut plataforma=Plataforma::inner_new(path.to_string());
        let usuario=Usuario::new("Juan".to_string(), "Perez".to_string(), "jp@gmail.com".to_string(), 2222222, true);
        let dni_usuario=usuario.dni;
        plataforma.registrar_usuario(usuario);
        
        assert!(plataforma.transacciones.len()==0);
        let resultado=plataforma.ingresar_dinero(dni_usuario,500.0);
        assert_eq!(resultado,Err("No se pudo acceder al archivo".to_string()));
        assert!(plataforma.transacciones.len()==0);


        let _=std::fs::remove_dir(path);
    }

    #[test]
    fn comprar_cripto_fallido_por_falla_en_creacion_de_archivo_test(){
        let path="src/tp5/testeo_de_errores_comprar_cripto";
        let _ = std::fs::remove_file(path);
        let _ = std::fs::remove_dir(path);
        //El archivo existe pero es un directorio, no se pueden escribir datos en él

        fs::create_dir(path).unwrap();
        let mut plataforma=Plataforma::inner_new(path.to_string());
        let mut usuario=Usuario::new("Juan".to_string(), "Perez".to_string(), "jp@gmail.com".to_string(), 2222222, true);
        let dni_usuario=usuario.dni;
        //sumo para que no falle en operaciones previas
        usuario.balance_fiat+=5000.0;
        plataforma.registrar_usuario(usuario.clone());
        let resultado=plataforma.comprar_cripto(dni_usuario, 350.0, "ETH");
        
        assert!(plataforma.transacciones.len()==0);
        assert_eq!(resultado,Err("No se pudo acceder al archivo".to_string()));
        assert!(plataforma.transacciones.len()==0);

        let _=std::fs::remove_dir(path);
    }

    #[test]
    fn vender_cripto_fallido_por_falla_en_creacion_de_archivo_test(){
        let path="src/tp5/testeo_de_errores_vender_cripto";
        let _ = std::fs::remove_file(path);
        let _ = std::fs::remove_dir(path);
        //El archivo existe pero es un directorio, no se pueden escribir datos en él

        fs::create_dir(path).unwrap();
        let mut plataforma=Plataforma::inner_new(path.to_string());
        let mut usuario=Usuario::new("Juan".to_string(), "Perez".to_string(), "jp@gmail.com".to_string(), 2222222, true);
        let dni_usuario=usuario.dni;
        //inserto para que no falle en operaciones previas
        usuario.balance_cripto.insert("ETH".to_string(), 2.0);
        plataforma.registrar_usuario(usuario.clone());
        let resultado=plataforma.vender_cripto(dni_usuario, 0.05,"ETH");
        
        assert!(plataforma.transacciones.len()==0);
        assert_eq!(resultado,Err("No se pudo acceder al archivo".to_string()));
        assert!(plataforma.transacciones.len()==0);

        let _=std::fs::remove_dir(path);
    }

    #[test]
    fn retirar_cripto_fallido_por_falla_en_creacion_de_archivo_test(){
        let path="src/tp5/testeo_de_errores_retirar_cripto";
        let _ = std::fs::remove_file(path);
        let _ = std::fs::remove_dir(path);
        //El archivo existe pero es un directorio, no se pueden escribir datos en él

        fs::create_dir(path).unwrap();

        let mut plataforma=Plataforma::inner_new(path.to_string());

        let mut usuario=Usuario::new("Juan".to_string(), "Perez".to_string(), "jp@gmail.com".to_string(), 2222222, true);
        let dni_usuario=usuario.dni;

        //inserto para que no falle en operaciones previas
        usuario.balance_cripto.insert("ETH".to_string(), 2.0);
        plataforma.registrar_usuario(usuario);
        
        let resultado=plataforma.retirar_cripto(dni_usuario, 0.04, "ETH", "EthereumNetwork");

        assert!(plataforma.transacciones.len()==0);
        assert_eq!(resultado,Err("No se pudo acceder al archivo".to_string()));
        assert!(plataforma.transacciones.len()==0);

        let _=std::fs::remove_dir(path);
    }

    #[test]
    fn retirar_fiat_fallido_por_falla_en_creacion_de_archivo_test(){
        let path="src/tp5/testeo_de_errores_retirar_fiat";
        let _ = std::fs::remove_file(path);
        let _ = std::fs::remove_dir(path);
        //El archivo existe pero es un directorio, no se pueden escribir datos en él

        fs::create_dir(path).unwrap();

        let mut plataforma=Plataforma::inner_new(path.to_string());

        let mut usuario=Usuario::new("Juan".to_string(), "Perez".to_string(), "jp@gmail.com".to_string(), 2222222, true);
        let dni_usuario=usuario.dni;
        usuario.balance_fiat+=1000.0;
        plataforma.registrar_usuario(usuario);
        
        let resultado=plataforma.retirar_fiat(dni_usuario, 200.0,
            MedioDeRetiro::TransferenciaBancaria { cbu: 123456, alias: "juan.mp".to_string()});

        assert!(plataforma.transacciones.len()==0);
        assert_eq!(resultado,Err("No se pudo acceder al archivo".to_string()));
        assert!(plataforma.transacciones.len()==0);

        let _=std::fs::remove_dir(path);
    }

}