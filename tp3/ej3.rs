#[derive(Debug,Clone)]
pub struct Fecha{
    dia:u32,
    mes:u32,
    año:u32
}

#[allow(unused)]
impl Fecha{

    pub fn new(dia:u32,mes:u32,año:u32) ->Fecha{
            Fecha{
                dia,
                mes,
                año
            }
    }

    fn cantidad_dias_en_mes(&self)->u32{
        match self.mes{
            1|3|5|7|8|10|12=>31,
            4|6|9|11=> 30,
            2 if self.es_bisiesto()=>29,
            2 =>28,
            _=>0,
        }
    }

    pub fn es_fecha_valida(self) ->bool{
        self.cantidad_dias_en_mes() !=0 && self.dia <= self.cantidad_dias_en_mes() 
    }

    pub fn es_bisiesto(&self)->bool{
        self.año % 4 ==0 && (self.año % 100 != 0 || self.año % 400 ==0)
    }

    // pub fn es_mayor(&self,una_fecha:Fecha)->bool{
    //     (self.año<una_fecha.año)&&(self.mes<=una_fecha.mes)&&(self.dia<una_fecha.dia)||    

    // }
    pub fn es_mayor(&self,una_fecha:Fecha)->bool{
        (self.año>una_fecha.año)||    
        ((self.año==una_fecha.año)&&(self.mes>una_fecha.mes))||
        ((self.año==una_fecha.año)&&(self.mes==una_fecha.mes)&& (self.dia>una_fecha.dia))
    }

    pub fn sumar_dias(&mut self,mut dias:u32){
        while dias>0{
            let dias_restantes:u32=self.cantidad_dias_en_mes()-self.dia;
            if dias<=dias_restantes{
                self.dia+=dias;
                dias=0;
            }else{
                dias-=dias_restantes+1;
                self.dia=1;
                self.mes+=1;
                if self.año.checked_add(1).is_some() && self.mes==12 {
                    self.mes=1;
                    self.año+=1;
                }else if self.año.checked_add(1).is_none(){
                    panic!("No se pueden sumar días");
                }
            }
        }
    }

    pub fn restar_dias(&mut self, mut dias:u32){
        while dias>0{
            let dias_restantes:u32=self.dia-1;
            if dias<= dias_restantes{
                self.dia-=dias;
                dias=0;
            }else{
                self.mes-=1;
                dias-=dias_restantes+1;
                if self.año.checked_sub(1).is_some() && self.mes==1 {
                    self.mes=12;
                    self.año-=1;
                }else if self.año.checked_sub(1).is_none(){
                    panic!("No se pueden restar días");
                }
                self.dia=self.cantidad_dias_en_mes();
            }
        }
    }

    pub fn misma_fecha(&self,fecha:&Fecha)->bool {
        self.dia==fecha.dia && self.mes==fecha.mes && self.año==fecha.año
    }

}

#[cfg(test)]
mod test{
    use crate::tp3::ej3::Fecha;

    //Test es fecha válida
    #[test]
    fn es_fecha_valida_test(){
        let fecha=Fecha::new(01, 11, 2001);
        assert_eq!(fecha.es_fecha_valida(),true);
    }

    #[test]
    fn es_fecha_valida_y_año_bisiesto_test(){
        let fecha=Fecha::new(29, 02, 2000);
        assert_eq!(fecha.es_fecha_valida(),true);
    }
    
    #[test]
    fn es_fecha_valida_mes_invalido_test(){
        let fecha=Fecha::new(01, 15, 2001);
        assert_eq!(fecha.es_fecha_valida(),false);
    }
    
    #[test]
    fn es_fecha_no_valida_dia_invalido_test(){
        let fecha=Fecha::new(60, 11, 2001);
        assert_eq!(fecha.es_fecha_valida(),false);
    }
    
    #[test]
    fn es_fecha_no_valida_por_dia_y_mes_invalidos_test(){
        let fecha=Fecha::new(60, 15, 2001);
        assert_eq!(fecha.es_fecha_valida(),false);
    }
    
    
    //test es bisiesto
    #[test]
    fn es_bisiesto_año_no_bisiesto_test(){
        let fecha=Fecha::new(12, 03, 2001);
        assert_eq!(fecha.es_bisiesto(),false);
    }
    
    #[test]
    fn es_bisiesto_año_bisiesto_test(){
        let fecha=Fecha::new(12, 03, 2000);
        assert_eq!(fecha.es_bisiesto(),true);
    }

    //test es mayor
    #[test]
    fn es_mayor_test(){
        let fecha=Fecha::new(03,12,2002);
        let otra_fecha=Fecha::new(01,11,2001);
        assert_eq!(fecha.es_mayor(otra_fecha),true);
    }
    
    #[test]
    fn es_mayor_no_cumple_por_año_test(){
        let fecha=Fecha::new(01,11,1999);
        let otra_fecha=Fecha::new(01,11,2000);
        assert_eq!(fecha.es_mayor(otra_fecha),false);
    }
    
     #[test]
    fn es_mayor_no_cumple_por_mes_test(){
        let fecha=Fecha::new(01,09,2001);
        let otra_fecha=Fecha::new(01,10,2001);
        assert_eq!(fecha.es_mayor(otra_fecha),false);
    }
    
     #[test]
    fn es_mayor_no_cumple_por_dia_test(){
        let fecha=Fecha::new(01,11,2001);
        let otra_fecha=Fecha::new(21,11,2001);
        assert_eq!(fecha.es_mayor(otra_fecha),false);
    }

    #[test]
    fn sumar_dias_test(){
        let mut fecha=Fecha::new(14,4,2026);
        fecha.sumar_dias(50);
        assert_eq!(fecha.dia,3);
        assert_eq!(fecha.mes,6);
        assert_eq!(fecha.año,2026);
    }

    #[test]
    fn sumar_cero_dias_test(){
        let mut fecha=Fecha::new(14,4,2026);
        fecha.sumar_dias(0);
        assert_eq!(fecha.dia,14);
        assert_eq!(fecha.mes,4);
        assert_eq!(fecha.año,2026);
    }

    #[test]
    #[should_panic(expected="No se pueden sumar días")]
    fn sumar_dias_overflow_test(){
        let mut fecha=Fecha::new(25,12,u32::MAX);
        fecha.sumar_dias(50);
    }
    
    #[test]
    fn restar_dias_test(){
        let mut fecha=Fecha::new(14,4,2026);
        fecha.restar_dias(50);
        assert_eq!(fecha.dia,23);
        assert_eq!(fecha.mes,2);
        assert_eq!(fecha.año,2026);
    }

    #[test]
    fn restar_cero_dias_test(){
        let mut fecha=Fecha::new(14,4,2026);
        fecha.restar_dias(0);
        assert_eq!(fecha.dia,14);
        assert_eq!(fecha.mes,4);
        assert_eq!(fecha.año,2026);
    }
    
    #[test]
    #[should_panic(expected="No se pueden restar días")]
    fn restar_dias_underflow_test(){
        let mut fecha=Fecha::new(25,12,0);
        fecha.restar_dias(50);
    }

    #[test]
    fn misma_fecha_test(){
        let fecha=Fecha::new(12, 03, 2001);
        assert!(fecha.misma_fecha(&fecha));
    }

    #[test]
    fn misma_fecha_diferente_fecha_test(){
        let fecha=Fecha::new(12, 03, 2001);
        assert!(!fecha.misma_fecha(&Fecha::new(12, 03, 2002)));
    }
}

