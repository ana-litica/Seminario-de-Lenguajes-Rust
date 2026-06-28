use std::collections::HashMap;
use crate::tp3::ej3::Fecha;

#[allow(unused)]
#[derive(Debug,PartialEq,Clone,Copy)]
struct Producto <'a>{
    nombre:&'a str,
    categoria:&'a str,
    precio_base:f32,
}

#[allow(unused)]
#[derive(Debug,PartialEq,Clone,Copy)]
struct Cliente<'a>{
    nombre:&'a str, 
    apellido:&'a str,
    direccion:&'a str,
    dni:u32,
    suscripcion_newsletter:bool,
}

#[allow(unused)]
#[derive(Debug,PartialEq,Clone,Copy)]
struct Vendedor{
    legajo:u32,
    antiguedad:u8,
    salario:f64,
}

#[allow(unused)]
#[derive(Debug,PartialEq,Clone)]
enum MedioDePago{
    Efectivo,
    TarjetaDeDebito,
    TarjetaDeCredito,
    TransferenciaBancaria,
}

#[allow(unused)]
#[derive(Debug,PartialEq,Clone)]
struct Venta<'a>{
    fecha:Fecha,
    cliente:Cliente<'a>,
    vendedor:Vendedor,
    medio_de_pago:MedioDePago,
    listado_de_productos:Vec<Detalle<'a>>,
}

#[allow(unused)]
#[derive(Debug,PartialEq,Clone,Copy)]
struct Detalle<'a>{
    producto:Producto<'a>,
    cantidad:u32,
}

#[allow(unused)]
#[derive(Debug,PartialEq,Clone)]
struct Sistema<'a>{
    ventas:Vec<Venta<'a>>,
    descuentos:HashMap<String,f32>,
}

#[allow(unused)]
#[derive(Debug,PartialEq,Clone)]
struct DatoDeCategoria<'a>{
    categoria:&'a str,
    monto_total:f64,
}

#[allow(unused)]
#[derive(Debug,PartialEq,Clone)]
struct DatoDeVendedor{
    vendedor:Vendedor,
    monto_total:f64,
}

pub struct ReporteDeCategoria<'a>{
    item:Vec<DatoDeCategoria<'a>>,
}

pub struct ReporteDeVendedor{
    item:Vec<DatoDeVendedor>,
}

#[allow(unused)]
impl<'a> Detalle<'a> {
    fn new(producto:Producto,cantidad:u32)->Detalle{
        Detalle { producto, cantidad }
    }
}

#[allow(unused)]
impl<'a> Cliente<'a>{
    
    fn new(nombre:&'a str, apellido:&'a str, direccion:&'a str,dni:u32,suscripcion_newsletter:bool)->Cliente<'a>{
        Cliente{
            nombre,
            apellido,
            direccion,
            dni,
            suscripcion_newsletter,
        }
    }

    fn get_descuento_por_newsletter(&self)->f32{
        if self.suscripcion_newsletter{0.10}else{0.0}     
    }
    
}

#[allow(unused)]
impl<'a> DatoDeCategoria<'a>{
    fn new(categoria:&'a str,monto:f64)->DatoDeCategoria<'a>{
        DatoDeCategoria { categoria, monto_total:monto }
    }
}

#[allow(unused)]
impl DatoDeVendedor{
    fn new(vendedor:Vendedor,monto:f64)->DatoDeVendedor{
        DatoDeVendedor { vendedor, monto_total:monto }
    }
}

#[allow(unused)]
impl<'a> ReporteDeCategoria<'a>{
    fn new()->ReporteDeCategoria <'a>{
        ReporteDeCategoria { item: Vec::new() }
    }
}

#[allow(unused)]
impl ReporteDeVendedor{
    fn new()->ReporteDeVendedor{
        ReporteDeVendedor { item: Vec::new() }
    }
}


#[allow(unused)]
impl Vendedor{
    
    fn new(legajo:u32,antiguedad:u8,salario:f64)->Vendedor{
        Vendedor { legajo, antiguedad, salario }
    }
}

#[allow(unused)]
impl<'a> Venta<'a>{
    
    fn new(fecha:Fecha, cliente:Cliente<'a>, vendedor:Vendedor,pago:MedioDePago,lista:Vec<Detalle<'a>>)->Venta<'a>{
        Venta { fecha, cliente, vendedor, medio_de_pago:pago, listado_de_productos:lista }
    }

    fn total_final_de_venta(&self,descuentos:&HashMap<String,f32>)->f32{
        let precio_productos=self.listado_de_productos.iter()
                                .fold(0.0,|acc,detalle|acc+detalle.producto.total_descuento(detalle.cantidad,descuentos));
        precio_productos-precio_productos*self.cliente.get_descuento_por_newsletter()
    }

    //funcion para agregar un producto al listado de la venta (para la creacion de la venta)
    fn agregar_producto(&mut self,producto:Producto<'a>,cantidad:u32){
        let agregado=Detalle{producto,cantidad};
        self.listado_de_productos.push(agregado);
    }

    fn acumular_por_categoria(&self,reporte:&mut Vec<DatoDeCategoria<'a>>,descuentos:&HashMap<String,f32>){
        self.listado_de_productos.iter().for_each(|det|det.producto.subtotal(det.cantidad,reporte,descuentos));
    }
    
}

#[allow(unused)]
impl<'a> Producto<'a>{
    
    fn new(nombre:&'a str, categoria:&'a str, precio:f32)->Producto<'a>{
        Producto { nombre, categoria, precio_base: precio }
    }

    fn total_descuento(&self,cantidad:u32,descuentos:&HashMap<String,f32>)->f32{
        let total=self.precio_base*cantidad as f32;

        if let Some(desc)=descuentos.get(self.categoria){
            total-total*desc
        }else{
            total
        }
    }

    fn subtotal(&self,cantidad:u32,reporte:&mut Vec<DatoDeCategoria<'a>>,descuentos:&HashMap<String,f32>){
        let pos=reporte.iter().position(|rep|rep.categoria==self.categoria);
        if pos.is_some(){
            reporte[pos.unwrap()].monto_total+=self.total_descuento(cantidad, descuentos) as f64;
        }else{
            let rep=DatoDeCategoria::new(self.categoria, self.total_descuento(cantidad, descuentos)as f64) ;
            reporte.push(rep);
        }
    }

}

#[allow(unused)]
impl<'a> Sistema<'a>{

    fn new()->Sistema<'a>{
        let descuentos=Self::get_descuento_por_categoria();
        Sistema { ventas: Vec::new(),descuentos}
    }

    fn vender(&mut self, fecha:Fecha, cliente:Cliente<'a>, vendedor:Vendedor,pago:MedioDePago,lista:Vec<Detalle<'a>>){
        let venta=Venta::new(fecha, cliente, vendedor, pago, lista);
        self.ventas.push(venta);
    }

    fn venta_existente(&self,venta:&Venta)->Option<&Venta<'_>>{
        self.ventas.iter().find(|v|*v==venta)
    }


    fn get_descuento_por_categoria()->HashMap<String,f32>{
        let mut descuentos=HashMap::new();
        descuentos.insert("Electronica".to_string(), 0.10);
        descuentos.insert("Indumentaria".to_string(), 0.15);
        descuentos.insert("Hogar".to_string(), 0.12);
        descuentos.insert("Libreria".to_string(), 0.08);
        descuentos
    }


    fn precio_final(&self,venta:&Venta)->Result<f32,&str>{
        match self.venta_existente(venta){
            Some(venta)=>{
                Ok(venta.total_final_de_venta(&self.descuentos))},
            None=>Err("No existe la venta"),
        }
    }

    fn reporte_por_categoria(&self)->ReporteDeCategoria<'a>{
        let mut reporte=ReporteDeCategoria::new();
        self.ventas.iter().for_each(|v|v.acumular_por_categoria(&mut reporte.item,&self.descuentos));
        reporte
    }

    fn acumular_por_vendedor(&self,venta:&Venta,reporte:&mut Vec<DatoDeVendedor>){
        if let Some(posicion_vendedor)=reporte.iter().position(|vend|vend.vendedor==venta.vendedor){
            reporte[posicion_vendedor].monto_total+=self.precio_final(venta).unwrap() as f64;
        }else{
            let dato=DatoDeVendedor::new(venta.vendedor.clone(),self.precio_final(venta).unwrap()as f64);
            reporte.push(dato);
        }
    }

    fn reporte_por_vendedor(&self)->ReporteDeVendedor{
        let mut reporte=ReporteDeVendedor::new();
        self.ventas.iter().for_each(|v|self.acumular_por_vendedor(v,& mut reporte.item));
        reporte
    }

}

#[cfg(test)]
mod test{
    use crate::tp4::ej4::{Fecha,Cliente, Detalle, MedioDePago, Producto,DatoDeCategoria,DatoDeVendedor, Sistema, Vendedor, Venta};

    #[test]
    fn vender_test(){
        let mut sistema=Sistema::new();

        let prod1=Producto::new("Notebook","Electronica",1000.0);
        let prod2=Producto::new("Auriculares","Electronica",500.0);

        let cliente=Cliente::new("Selena", "Gomez", "calle 123", 40123456, false);
        let vendedor=Vendedor::new(1001,2, 1_200_000.0 );

        let det1=Detalle::new(prod1,1);
        let det2=Detalle::new(prod2,3);
        let lista=vec![det1,det2];

        assert!(sistema.ventas.len()==0);
        sistema.vender(Fecha::new(21,06,26), cliente, vendedor, MedioDePago::Efectivo, lista.clone());

        assert!(sistema.ventas.len()==1);
        assert_eq!(sistema.ventas[0],Venta::new(Fecha::new(21,06,26), cliente, vendedor, MedioDePago::Efectivo, lista));
    }

    #[test]
    fn precio_final_sin_newsletter_con_descuento_por_categoria_test(){
        let mut sistema=Sistema::new();

        let prod1=Producto::new("Notebook","Electronica",1000.0);
        let prod2=Producto::new("Auriculares","Electronica",500.0);

        let cliente=Cliente::new("Selena", "Gomez", "calle 123", 40123456, false);
        let cliente2=Cliente::new("Juan", "Perez", "calle 132", 40156456, true);
        let vendedor=Vendedor::new(1001,2, 1_200_000.0 );

        let det1=Detalle::new(prod1,1);
        let det2=Detalle::new(prod2,3);
        let det3=Detalle::new(prod2,2);
        let lista=vec![det1,det2];
        let lista2=vec![det3];

        sistema.vender(Fecha::new(21,06,26), cliente, vendedor, MedioDePago::Efectivo, lista.clone());
        sistema.vender(Fecha::new(25,06,26), cliente2, vendedor, MedioDePago::TarjetaDeDebito, lista2.clone());

        let venta=Venta::new(Fecha::new(21,06,26), cliente, vendedor, MedioDePago::Efectivo, lista);

        assert_eq!(sistema.precio_final(&venta),Ok(2250.0));
    }

    #[test]
    fn precio_final_con_newsletter_con_descuento_por_categoria_test(){
        let mut sistema=Sistema::new();

        let prod1=Producto::new("Notebook","Electronica",1000.0);
        let prod2=Producto::new("Auriculares","Electronica",500.0);

        let cliente=Cliente::new("Selena", "Gomez", "calle 123", 40123456, false);
        let cliente2=Cliente::new("Juan", "Perez", "calle 132", 40156456, true);
        let vendedor=Vendedor::new(1001,2, 1_200_000.0 );

        let det1=Detalle::new(prod1,1);
        let det2=Detalle::new(prod2,3);
        let det3=Detalle::new(prod2,2);
        let lista=vec![det1,det2];
        let lista2=vec![det3];

        sistema.vender(Fecha::new(21,06,26), cliente, vendedor, MedioDePago::Efectivo, lista.clone());
        sistema.vender(Fecha::new(25,06,26), cliente2, vendedor, MedioDePago::TarjetaDeDebito, lista2.clone());

        let venta2=Venta::new(Fecha::new(25,06,26), cliente2, vendedor, MedioDePago::TarjetaDeDebito, lista2);

        assert_eq!(sistema.precio_final(&venta2),Ok(810.0));
    }

    #[test]
    fn precio_final_con_newsletter_sin_descuento_por_categoria_test(){
        let mut sistema=Sistema::new();

        let prod1=Producto::new("Notebook","Electronica",1000.0);
        let prod2=Producto::new("Queso","Alimentos",300.0);

        let cliente=Cliente::new("Selena", "Gomez", "calle 123", 40123456, false);
        let cliente2=Cliente::new("Juan", "Perez", "calle 132", 40156456, true);
        let vendedor=Vendedor::new(1001,2, 1_200_000.0 );

        let det1=Detalle::new(prod1,1);
        let det2=Detalle::new(prod2,3);
        let det3=Detalle::new(prod2,2);
        let lista=vec![det1,det2];
        let lista2=vec![det3];

        sistema.vender(Fecha::new(21,06,26), cliente, vendedor, MedioDePago::Efectivo, lista.clone());
        sistema.vender(Fecha::new(25,06,26), cliente2, vendedor, MedioDePago::TarjetaDeDebito, lista2.clone());

        let venta2=Venta::new(Fecha::new(25,06,26), cliente2, vendedor, MedioDePago::TarjetaDeDebito, lista2);

        assert_eq!(sistema.precio_final(&venta2),Ok(540.0));
    }

    #[test]
    fn precio_final_sin_newsletter_sin_descuento_por_categoria_test(){
        let mut sistema=Sistema::new();

        let prod1=Producto::new("Queso","Alimentos",1000.0);
        let prod3=Producto::new("Cereal","Alimentos",1000.0);
        let prod2=Producto::new("Auriculares","Electronica",500.0);

        let cliente=Cliente::new("Selena", "Gomez", "calle 123", 40123456, false);
        let cliente2=Cliente::new("Juan", "Perez", "calle 132", 40156456, true);
        let vendedor=Vendedor::new(1001,2, 1_200_000.0 );

        let det1=Detalle::new(prod1,1);
        let det2=Detalle::new(prod3,3);
        let det3=Detalle::new(prod2,2);
        let lista=vec![det1,det2];
        let lista2=vec![det3];

        sistema.vender(Fecha::new(21,06,26), cliente, vendedor, MedioDePago::Efectivo, lista.clone());
        sistema.vender(Fecha::new(25,06,26), cliente2, vendedor, MedioDePago::TarjetaDeDebito, lista2.clone());

        let venta=Venta::new(Fecha::new(21,06,26), cliente, vendedor, MedioDePago::Efectivo, lista);

        assert_eq!(sistema.precio_final(&venta),Ok(4000.0));
    }

    #[test]
    fn precio_final_sin_newsletter_algunos_sin_descuento_por_categoria_test(){
        let mut sistema=Sistema::new();

        let prod1=Producto::new("Queso","Alimentos",1000.0);
        let prod3=Producto::new("Notebook","Electronica",1000.0);
        let prod2=Producto::new("Auriculares","Electronica",500.0);

        let cliente=Cliente::new("Selena", "Gomez", "calle 123", 40123456, false);
        let cliente2=Cliente::new("Juan", "Perez", "calle 132", 40156456, true);
        let vendedor=Vendedor::new(1001,2, 1_200_000.0 );

        let det1=Detalle::new(prod1,3);
        let det2=Detalle::new(prod3,1);
        let det3=Detalle::new(prod2,2);
        let lista=vec![det1,det2];
        let lista2=vec![det3];

        sistema.vender(Fecha::new(21,06,26), cliente, vendedor, MedioDePago::Efectivo, lista.clone());
        sistema.vender(Fecha::new(25,06,26), cliente2, vendedor, MedioDePago::TarjetaDeDebito, lista2.clone());

        let venta=Venta::new(Fecha::new(21,06,26), cliente, vendedor, MedioDePago::Efectivo, lista);

        assert_eq!(sistema.precio_final(&venta),Ok(3900.0));
    }

    #[test]
    fn precio_final_con_newsletter_algunos_sin_descuento_por_categoria_test(){
        let mut sistema=Sistema::new();

        let prod1=Producto::new("Queso","Alimentos",1000.0);
        let prod3=Producto::new("Notebook","Electronica",1000.0);
        let prod2=Producto::new("Auriculares","Electronica",500.0);

        let cliente=Cliente::new("Selena", "Gomez", "calle 123", 40123456, true);
        let cliente2=Cliente::new("Juan", "Perez", "calle 132", 40156456, true);
        let vendedor=Vendedor::new(1001,2, 1_200_000.0 );

        let det1=Detalle::new(prod1,3);
        let det2=Detalle::new(prod3,1);
        let det3=Detalle::new(prod2,2);
        let lista=vec![det1,det2];
        let lista2=vec![det3];

        sistema.vender(Fecha::new(21,06,26), cliente, vendedor, MedioDePago::Efectivo, lista.clone());
        sistema.vender(Fecha::new(25,06,26), cliente2, vendedor, MedioDePago::TarjetaDeDebito, lista2.clone());

        let venta=Venta::new(Fecha::new(21,06,26), cliente, vendedor, MedioDePago::Efectivo, lista);

        assert_eq!(sistema.precio_final(&venta),Ok(3510.0));
    }

    #[test]
    fn precio_final_venta_inexistente_test(){
        let mut sistema=Sistema::new();

        let prod1=Producto::new("Queso","Alimentos",1000.0);
        let prod3=Producto::new("Notebook","Electronica",1000.0);
        let prod2=Producto::new("Auriculares","Electronica",500.0);

        let cliente=Cliente::new("Selena", "Gomez", "calle 123", 40123456, true);
        let cliente2=Cliente::new("Juan", "Perez", "calle 132", 40156456, true);
        let vendedor=Vendedor::new(1001,2, 1_200_000.0 );

        let det1=Detalle::new(prod1,3);
        let det2=Detalle::new(prod3,1);
        let det3=Detalle::new(prod2,2);
        let lista=vec![det1,det2];
        let lista2=vec![det3];

        sistema.vender(Fecha::new(21,06,26), cliente, vendedor, MedioDePago::Efectivo, lista.clone());

        let venta2=Venta::new(Fecha::new(25,06,26), cliente2, vendedor, MedioDePago::TarjetaDeDebito, lista2);

        assert_eq!(sistema.precio_final(&venta2),Err("No existe la venta"));
    }

    #[test]
    fn precio_final_sistema_sin_ventas_test(){
        let sistema=Sistema::new();

        let prod1=Producto::new("Queso","Alimentos",1000.0);
        let prod3=Producto::new("Notebook","Electronica",1000.0);
        let det1=Detalle::new(prod1,3);
        let det2=Detalle::new(prod3,1);
        let lista=vec![det1,det2];
        let cliente=Cliente::new("Selena", "Gomez", "calle 123", 40123456, true);
        let vendedor=Vendedor::new(1001,2, 1_200_000.0 );
        let venta=Venta::new(Fecha::new(21,06,26), cliente, vendedor, MedioDePago::Efectivo, lista);

        assert_eq!(sistema.precio_final(&venta),Err("No existe la venta"));
    }

    #[test]
    fn reporte_por_categoria_test(){
        let mut sistema = Sistema::new();

        let cliente = Cliente::new("Selena", "Gomez", "calle 123", 40123456, false);
        let vendedor = Vendedor::new(1001, 2, 1_200_000.0);

        let prod = Producto::new("Notebook", "Electronica", 1000.0);
        let prod2=Producto::new("Auriculares","Electronica",500.0);

        let lista1 = vec![Detalle::new(prod, 1),Detalle::new(prod2, 3)];
        let lista2 = vec![Detalle::new(prod2, 2)];

        
        sistema.vender(Fecha::new(21,06,26),cliente,vendedor,MedioDePago::Efectivo,lista1);
        sistema.vender(Fecha::new(22,06,26),cliente, vendedor,MedioDePago::TarjetaDeDebito,lista2);

        let reporte = sistema.reporte_por_categoria();

        assert_eq!(reporte.item.len(), 1);
        assert_eq!(reporte.item[0],DatoDeCategoria::new("Electronica", 3150.0));
    }

    //////////////////////////////

    #[test]
    fn reporte_por_vendedor_un_vendedor_test() {
        let mut sistema = Sistema::new();

        let prod = Producto::new("Notebook", "Electronica", 1000.0);

        let cliente = Cliente::new("Selena", "Gomez", "calle 123", 40123456, false);
        let vendedor = Vendedor::new(1001, 2, 1_200_000.0);

        let lista1 = vec![Detalle::new(prod, 1)];
        let lista2 = vec![Detalle::new(prod, 2)];

        sistema.vender(Fecha::new(21,06,26),cliente,vendedor,MedioDePago::Efectivo,lista1);
        sistema.vender(Fecha::new(22,06,26),cliente, vendedor,MedioDePago::TarjetaDeDebito,lista2);

        let reporte = sistema.reporte_por_vendedor();

        assert_eq!(reporte.item.len(), 1);
        assert_eq!(reporte.item[0],DatoDeVendedor::new(vendedor, 2700.0));
    }

    #[test]
    fn reporte_por_vendedor_varios_vendedores_test() {
        let mut sistema = Sistema::new();

        let prod = Producto::new("Notebook", "Electronica", 1000.0);

        let cliente = Cliente::new("Selena", "Gomez", "calle 123", 40123456, false);

        let vendedor1 = Vendedor::new(1001, 2, 1_200_000.0);
        let vendedor2 = Vendedor::new(1002, 5, 1_500_000.0);

        sistema.vender(Fecha::new(21,06,26),cliente,vendedor1,MedioDePago::Efectivo,vec![Detalle::new(prod, 1)]); 

        sistema.vender(Fecha::new(22,06,26), cliente,vendedor2,MedioDePago::Efectivo,vec![Detalle::new(prod, 2)]);

        let reporte = sistema.reporte_por_vendedor();

        assert_eq!(reporte.item.len(), 2);

        let rep1=DatoDeVendedor::new(vendedor1, 900.0);
        let rep2=DatoDeVendedor::new(vendedor2, 1800.0);

        assert_eq!(reporte.item[0],rep1);
        assert_eq!(reporte.item[1],rep2);

    }

    #[test]
    fn reporte_por_vendedor_sistema_sin_ventas_test() {
        let sistema = Sistema::new();

        let reporte = sistema.reporte_por_vendedor();

        assert!(reporte.item.len()==0);
    }
    

}