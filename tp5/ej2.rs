//El coverage del ejercicio original dio 96.67%

use std::fmt::Debug;
use std::{fs::File, io::Write};
use serde::{Serialize,Deserialize};


#[derive(Debug,Clone,Serialize,Deserialize)]
#[allow(unused)]
struct Cancion{
    titulo:String,
    artista:String,
    genero:Genero,
}

#[derive(Debug,Clone,Serialize,Deserialize)]
#[allow(unused)]
enum Genero{
    Rock,
    Pop,
    Rap,
    Jazz,
    Otros,
}

#[derive(Debug,Clone,Serialize,Deserialize)]
#[allow(unused)]
struct Playlist{
    nombre:String,
    canciones:Vec<Cancion>,
    path_archivo:String,
}

#[derive(Debug)]
enum Errores{
    ErrorDeCreacionDeArchivo,
    ErrorDeEscrituraDeDatos,
    ErrorDeSerializacionDeDatos,
}

#[allow(unused)]
impl Cancion{
    pub fn new(titulo:String,artista:String,genero:Genero)->Cancion{
        Cancion { titulo, artista, genero }
    }

    fn mismo_genero(self,genero:&Genero)->bool{
        match (self.genero,genero){
            (Genero::Jazz,Genero::Jazz)|(Genero::Otros,Genero::Otros)|
            (Genero::Rap,Genero::Rap) |(Genero::Pop,Genero::Pop)|(Genero::Rock,Genero::Rock)=>true,
            _=>false,
        }
    }

    fn misma_cancion(self,cancion:&Cancion)->bool{
        self.titulo==cancion.titulo && self.artista==cancion.artista && self.mismo_genero(&cancion.genero)
    }
}

#[allow(unused)]
impl Playlist{

    pub fn new(nombre:String)->Playlist{
        Self::inner_new(nombre, "src/tp5/archivo_playlist".to_string())
    }

    fn inner_new(nombre:String,path_archivo:String)->Playlist{
        Playlist {nombre, canciones: Vec::new(), path_archivo }
    }

    fn escribir_en_archivo(&self)->Result<(),Errores>{
        let mut archivo=File::create(&self.path_archivo).map_err(|_|Errores::ErrorDeCreacionDeArchivo)?;
        let canciones_serializadas=serde_json::to_string(&self.canciones).map_err(|_|Errores::ErrorDeSerializacionDeDatos)?;
        archivo.write_all(&canciones_serializadas.as_bytes()).map_err(|_|Errores::ErrorDeEscrituraDeDatos)?;
        Ok(())
    }

    pub fn agregar_cancion(&mut self,cancion:&Cancion){
        self.canciones.push(cancion.clone());  
        if let Err(err)=self.escribir_en_archivo(){
            self.canciones.pop();
        }
    }

    pub fn eliminar_cancion(&mut self,cancion:&Cancion)->Option<Cancion>{
        let resultado_busqueda=self.clone().buscar_cancion(cancion.clone().titulo);
        if resultado_busqueda.is_none(){
            return None;
        }
        let mut cancion_buscada=resultado_busqueda.unwrap();
        let mut eliminada=false;

        if cancion.clone().misma_cancion(&cancion_buscada){
            let mut indice=0;
            while !eliminada && indice<self.canciones.len(){
                if self.canciones[indice].clone().misma_cancion(&cancion){
                    cancion_buscada=self.canciones.remove(indice);
                    eliminada=true;
                    if let Err(err)=self.escribir_en_archivo(){
                        self.canciones.insert(indice,cancion_buscada);
                        return None;
                    }
                    return Some(cancion_buscada);
                }
                indice+=1;
            }
        }
        None
    }

    pub fn buscar_cancion(&self,nombre:String)->Option<Cancion>{
        let mut indice:usize=0;
        while indice<self.canciones.len(){
            if self.canciones[indice].titulo==nombre{
                return Some(self.canciones[indice].clone());
            }
            indice+=1;
        }
        None
    }

    pub fn mover_cancion(&mut self,cancion:&Cancion, posicion:usize)->bool{
        let resultado_busqueda=self.clone().buscar_cancion(cancion.clone().titulo);
        if posicion>=self.canciones.len() || resultado_busqueda.is_none(){
            false
        }else{
            let cancion_buscada=resultado_busqueda.unwrap();
            if !cancion_buscada.misma_cancion(&cancion){
                return false;
            }
            
            let indice=self.canciones.iter().position(|c|c.clone().misma_cancion(cancion)).unwrap();
            self.canciones.remove(indice);
            self.canciones.insert(posicion,cancion.clone());

            if let Err(err)=self.escribir_en_archivo(){
                self.canciones.remove(posicion.clone());
                self.canciones.insert(indice,cancion.clone());
                return false;
            }
            true
            
        }
    }

    pub fn obtener_canciones_genero(&self,genero:Genero)->Vec<Cancion>{
        let mut canciones_genero:Vec<Cancion>=vec![];
        for song in self.clone().canciones{
            if song.clone().mismo_genero(&genero){
                canciones_genero.push(song);
            }
        }
        canciones_genero
    }

    pub fn obtener_canciones_artista(&self,artista:String)->Vec<Cancion>{
        let mut canciones_artista:Vec<Cancion>=vec![];
        for song in self.clone().canciones{
            if song.artista==artista{
                canciones_artista.push(song);
            }
        }
        canciones_artista
    }

    pub fn modificar_titulo(&mut self,nuevo_titulo:String){
        self.nombre=nuevo_titulo;
    }

    pub fn vaciar_playlist(&mut self){
        let mut back_up_canciones=self.canciones.clone();
        self.canciones.clear();
        if let Err(err) = self.escribir_en_archivo() {
            self.canciones.append(&mut back_up_canciones);
        }
    }

}

#[cfg(test)]
mod test{
    use crate::tp5::ej2::{Cancion,Genero,Playlist};
    use std::fs;

    #[test]
    fn agregar_cancion_test(){
        let mut playlist=Playlist::new("Para llorar".to_string());
        let cancion=Cancion::new("Bohemian Rhapsody".to_string(), "Queen".to_string(), Genero::Rock);
        assert!(playlist.canciones.is_empty());
        playlist.agregar_cancion(&cancion);
        assert_eq!(playlist.canciones.len(),1);
    }

    #[test]
    fn eliminar_cancion_test(){
        let mut playlist=Playlist::new("Para llorar".to_string());
        let cancion=Cancion::new("Bohemian Rhapsody".to_string(), "Queen".to_string(), Genero::Rock);
        let cancion2=Cancion::new("Smells Like Teen Spirit".to_string(),"Nirvana".to_string(),Genero::Rock);
        let cancion3=Cancion::new("Billie Jean".to_string(),"Michael Jackson".to_string(),Genero::Pop);
        playlist.agregar_cancion(&cancion.clone());
        playlist.agregar_cancion(&cancion2.clone());
        playlist.agregar_cancion(&cancion3.clone());

        assert_eq!(playlist.canciones.len(),3);
        playlist.eliminar_cancion(&cancion2);
        assert_eq!(playlist.canciones.len(),2);
        assert!(playlist.canciones[0].clone().misma_cancion(&cancion));
        assert!(playlist.canciones[1].clone().misma_cancion(&cancion3));      
    }

    #[test]
    fn eliminar_cancion_inexistente_test(){
        let mut playlist=Playlist::new("Para llorar".to_string());
        let cancion=Cancion::new("Bohemian Rhapsody".to_string(), "Queen".to_string(), Genero::Rock);
        let cancion2=Cancion::new("Smells Like Teen Spirit".to_string(),"Nirvana".to_string(),Genero::Rock);
        let cancion3=Cancion::new("Billie Jean".to_string(),"Michael Jackson".to_string(),Genero::Pop);
        playlist.agregar_cancion(&cancion);
        playlist.agregar_cancion(&cancion3);

        assert!(playlist.eliminar_cancion(&cancion2).is_none());   
    }

    #[test]
    fn eliminar_cancion_playlist_vacia_test(){
        let mut playlist=Playlist::new("Para llorar".to_string());
        let cancion=Cancion::new("Bohemian Rhapsody".to_string(), "Queen".to_string(), Genero::Rock);

        assert!(playlist.eliminar_cancion(&cancion).is_none());   
    }

    #[test]
    fn mover_cancion_para_arriba_test(){
        let mut playlist=Playlist::new("Para llorar".to_string());
        let cancion=Cancion::new("Bohemian Rhapsody".to_string(), "Queen".to_string(), Genero::Rock);
        let cancion2=Cancion::new("Smells Like Teen Spirit".to_string(),"Nirvana".to_string(),Genero::Rock);
        let cancion3=Cancion::new("Billie Jean".to_string(),"Michael Jackson".to_string(),Genero::Pop);
        playlist.agregar_cancion(&cancion.clone());
        playlist.agregar_cancion(&cancion2.clone());
        playlist.agregar_cancion(&cancion3.clone());

        assert!(playlist.canciones[0].clone().misma_cancion(&cancion));
        assert!(playlist.canciones[1].clone().misma_cancion(&cancion2));
        assert!(playlist.canciones[2].clone().misma_cancion(&cancion3));      
        
        playlist.mover_cancion(&cancion2.clone(),0);

        assert!(playlist.canciones[0].clone().misma_cancion(&cancion2));
        assert!(playlist.canciones[1].clone().misma_cancion(&cancion));
        assert!(playlist.canciones[2].clone().misma_cancion(&cancion3));      
    }

    #[test]
    fn mover_cancion_para_abajo_test(){
        let mut playlist=Playlist::new("Para llorar".to_string());
        let cancion=Cancion::new("Bohemian Rhapsody".to_string(), "Queen".to_string(), Genero::Rock);
        let cancion2=Cancion::new("Smells Like Teen Spirit".to_string(),"Nirvana".to_string(),Genero::Rock);
        let cancion3=Cancion::new("Billie Jean".to_string(),"Michael Jackson".to_string(),Genero::Pop);
        playlist.agregar_cancion(&cancion);
        playlist.agregar_cancion(&cancion2);
        playlist.agregar_cancion(&cancion3);

        assert!(playlist.canciones[0].clone().misma_cancion(&cancion));
        assert!(playlist.canciones[1].clone().misma_cancion(&cancion2));
        assert!(playlist.canciones[2].clone().misma_cancion(&cancion3));      
        
        playlist.mover_cancion(&cancion2,2);

        assert!(playlist.canciones[0].clone().misma_cancion(&cancion));
        assert!(playlist.canciones[1].clone().misma_cancion(&cancion3));      
        assert!(playlist.canciones[2].clone().misma_cancion(&cancion2));
    }

    #[test]
    fn mover_cancion_posicion_fuera_de_rango_test(){
        let mut playlist=Playlist::new("Para llorar".to_string());
        let cancion=Cancion::new("Bohemian Rhapsody".to_string(), "Queen".to_string(), Genero::Rock);
        let cancion2=Cancion::new("Smells Like Teen Spirit".to_string(),"Nirvana".to_string(),Genero::Rock);
        let cancion3=Cancion::new("Billie Jean".to_string(),"Michael Jackson".to_string(),Genero::Pop);
        playlist.agregar_cancion(&cancion);
        playlist.agregar_cancion(&cancion2);
        playlist.agregar_cancion(&cancion3);  
        
        assert!(!playlist.mover_cancion(&cancion2,7));
    }

    #[test]
    fn mover_cancion_mismo_titulo_otra_cancion_test(){
        let mut playlist=Playlist::new("Para llorar".to_string());
        let cancion=Cancion::new("Bohemian Rhapsody".to_string(), "Queen".to_string(), Genero::Rock);
        let cancion2=Cancion::new("Smells Like Teen Spirit".to_string(),"Nirvana".to_string(),Genero::Rock);
        let cancion3=Cancion::new("Billie Jean".to_string(),"Michael Jackson".to_string(),Genero::Pop);
        let cancion4=Cancion::new("Bohemian Rhapsody".to_string(), "Lali".to_string(), Genero::Rock);

        playlist.agregar_cancion(&cancion);
        playlist.agregar_cancion(&cancion2);
        playlist.agregar_cancion(&cancion3);   
        
        assert!(!playlist.mover_cancion(&cancion4,2));
    }

    #[test]
    fn mover_cancion_inexistente_test(){
        let mut playlist=Playlist::new("Para llorar".to_string());
        let cancion=Cancion::new("Bohemian Rhapsody".to_string(), "Queen".to_string(), Genero::Rock);
        let cancion2=Cancion::new("Smells Like Teen Spirit".to_string(),"Nirvana".to_string(),Genero::Rock);
        let cancion3=Cancion::new("Billie Jean".to_string(),"Michael Jackson".to_string(),Genero::Pop);
        playlist.agregar_cancion(&cancion);
        playlist.agregar_cancion(&cancion3);    
        
        assert!(!playlist.mover_cancion(&cancion2,0));
    }

    #[test]
    fn mover_cancion_playlist_vacia_test(){
        let mut playlist=Playlist::new("Para llorar".to_string());
        let cancion=Cancion::new("Bohemian Rhapsody".to_string(), "Queen".to_string(), Genero::Rock);
        
        assert!(!playlist.mover_cancion(&cancion,3));
    }

    #[test]
    fn buscar_cancion_test(){
        let mut playlist=Playlist::new("Para llorar".to_string());
        let cancion=Cancion::new("Bohemian Rhapsody".to_string(), "Queen".to_string(), Genero::Rock);
        let cancion2=Cancion::new("Smells Like Teen Spirit".to_string(),"Nirvana".to_string(),Genero::Rock);
        let cancion3=Cancion::new("Billie Jean".to_string(),"Michael Jackson".to_string(),Genero::Pop);
        playlist.agregar_cancion(&cancion);
        playlist.agregar_cancion(&cancion2);
        playlist.agregar_cancion(&cancion3);

        assert!(playlist.buscar_cancion("Billie Jean".to_string()).unwrap().misma_cancion(&cancion3));
    }

    #[test]
    fn buscar_cancion_inexistente_test(){
        let mut playlist=Playlist::new("Para llorar".to_string());
        let cancion=Cancion::new("Bohemian Rhapsody".to_string(), "Queen".to_string(), Genero::Rock);
        let cancion3=Cancion::new("Billie Jean".to_string(),"Michael Jackson".to_string(),Genero::Pop);
        playlist.agregar_cancion(&cancion);
        playlist.agregar_cancion(&cancion3);

        assert!(playlist.buscar_cancion("Smells Like Teen Spirit".to_string()).is_none());
    }

    #[test]
    fn buscar_cancion_playlist_vacia_test(){
        let playlist=Playlist::new("Para llorar".to_string());

        assert!(playlist.buscar_cancion("Smells Like Teen Spirit".to_string()).is_none());
    }

    #[test]
    fn obtener_canciones_genero_test(){
        let mut playlist=Playlist::new("Para llorar".to_string());
        let cancion=Cancion::new("Bohemian Rhapsody".to_string(), "Queen".to_string(), Genero::Rock);
        let cancion2=Cancion::new("Smells Like Teen Spirit".to_string(),"Nirvana".to_string(),Genero::Rock);
        let cancion3=Cancion::new("Billie Jean".to_string(),"Michael Jackson".to_string(),Genero::Pop);
        playlist.agregar_cancion(&cancion);
        playlist.agregar_cancion(&cancion2);
        playlist.agregar_cancion(&cancion3);

        let canciones=playlist.obtener_canciones_genero(Genero::Rock);
        assert!(canciones[0].clone().misma_cancion(&cancion));
        assert!(canciones[1].clone().misma_cancion(&cancion2));
    }    
    
    #[test]
    fn obtener_canciones_genero_ausente_en_playlist_test(){
        let mut playlist=Playlist::new("Para llorar".to_string());
        let cancion=Cancion::new("Bohemian Rhapsody".to_string(), "Queen".to_string(), Genero::Rock);
        let cancion2=Cancion::new("Smells Like Teen Spirit".to_string(),"Nirvana".to_string(),Genero::Rock);
        let cancion3=Cancion::new("Billie Jean".to_string(),"Michael Jackson".to_string(),Genero::Pop);
        playlist.agregar_cancion(&cancion);
        playlist.agregar_cancion(&cancion2);
        playlist.agregar_cancion(&cancion3);

        let canciones=playlist.obtener_canciones_genero(Genero::Otros);
        assert!(canciones.is_empty());
    }    

    #[test]
    fn obtener_canciones_genero_playlist_vacia_test(){
        let playlist=Playlist::new("Para llorar".to_string());

        let canciones=playlist.obtener_canciones_genero(Genero::Otros);
        assert!(canciones.is_empty());
    }    

    #[test]
    fn obtener_canciones_artista_test(){
        let mut playlist=Playlist::new("Para llorar".to_string());
        let cancion=Cancion::new("Bohemian Rhapsody".to_string(), "Queen".to_string(), Genero::Rock);
        let cancion2=Cancion::new("Smells Like Teen Spirit".to_string(),"Nirvana".to_string(),Genero::Rock);
        let cancion3=Cancion::new("Billie Jean".to_string(),"Michael Jackson".to_string(),Genero::Pop);
        let cancion4=Cancion::new("Good Old Fashioned Lover Boy".to_string(), "Queen".to_string(), Genero::Rock);
        playlist.agregar_cancion(&cancion);
        playlist.agregar_cancion(&cancion2);
        playlist.agregar_cancion(&cancion3);
        playlist.agregar_cancion(&cancion4);

        let canciones=playlist.obtener_canciones_artista("Queen".to_string());
        assert!(canciones[0].clone().misma_cancion(&cancion));
        assert!(canciones[1].clone().misma_cancion(&cancion4));
    }    

    #[test]
    fn obtener_canciones_artista_ausente_en_playlist_test(){
        let mut playlist=Playlist::new("Para llorar".to_string());
        let cancion=Cancion::new("Bad Romance".to_string(), "Lady Gaga".to_string(), Genero::Rock);
        let cancion2=Cancion::new("Smells Like Teen Spirit".to_string(),"Nirvana".to_string(),Genero::Rock);
        let cancion3=Cancion::new("Billie Jean".to_string(),"Michael Jackson".to_string(),Genero::Pop);
        playlist.agregar_cancion(&cancion);
        playlist.agregar_cancion(&cancion2);
        playlist.agregar_cancion(&cancion3);

        let canciones=playlist.obtener_canciones_artista("Queen".to_string());
        assert!(canciones.is_empty());
    }    

    #[test]
    fn obtener_canciones_artista_playlist_vacia_test(){
        let playlist=Playlist::new("Para llorar".to_string());

        let canciones=playlist.obtener_canciones_artista("Queen".to_string());
        assert!(canciones.is_empty());
    }    

    #[test]
    fn modificar_titulo_test(){
        let mut playlist=Playlist::new("Para llorar".to_string());
        assert_eq!(playlist.nombre,"Para llorar".to_string());
        playlist.modificar_titulo("Para reir".to_string());
        assert_eq!(playlist.nombre,"Para reir".to_string());
    }

    #[test]
    fn vaciar_playlist_test(){
        let mut playlist=Playlist::new("Para llorar".to_string());
        let cancion=Cancion::new("Bohemian Rhapsody".to_string(), "Queen".to_string(), Genero::Rock);
        let cancion2=Cancion::new("Smells Like Teen Spirit".to_string(),"Nirvana".to_string(),Genero::Rock);
        let cancion3=Cancion::new("Billie Jean".to_string(),"Michael Jackson".to_string(),Genero::Pop);
        playlist.agregar_cancion(&cancion);
        playlist.agregar_cancion(&cancion2);
        playlist.agregar_cancion(&cancion3);  

        assert!(!playlist.canciones.is_empty());
        playlist.vaciar_playlist();
        assert!(playlist.canciones.is_empty());
    }

    #[test]
    fn vaciar_playlist_vacia_test(){
        let mut playlist=Playlist::new("Para llorar".to_string());
        assert!(playlist.canciones.is_empty());
        playlist.vaciar_playlist();
        assert!(playlist.canciones.is_empty());
    }

    #[test]
    fn agregar_cancion_fallida_por_falla_de_creacion_de_archivo_test(){
        let path="src/tp5/testeo_de_errores_agregar_canciones";
        let _ = std::fs::remove_file(path);
        let _ = std::fs::remove_dir(path);

        //El archivo existe pero es un directorio, no se pueden escribir canciones en él
        fs::create_dir(path).unwrap();

        let mut playlist=Playlist::inner_new("Para llorar".to_string(),path.to_string());
        let cancion=Cancion::new("Bohemian Rhapsody".to_string(), "Queen".to_string(), Genero::Rock);
        playlist.agregar_cancion(&cancion);

        assert!(playlist.canciones.len()==0);

        let _=std::fs::remove_dir(path);

    }

    #[test]
    fn eliminar_cancion_fallida_por_falla_de_creacion_de_archivo_test(){
        let path="src/tp5/testeo_de_errores_eliminar_canciones";
        let _ = std::fs::remove_file(path);
        let _ = std::fs::remove_dir(path);

        //El archivo existe pero es un directorio, no se pueden escribir canciones en él
        fs::create_dir(path).unwrap();

        let mut playlist=Playlist::inner_new("Para llorar".to_string(),path.to_string());
        let cancion=Cancion::new("Bohemian Rhapsody".to_string(), "Queen".to_string(), Genero::Rock);

        //agregué con push porque sino el agregar falla
        playlist.canciones.push(cancion.clone());
        assert!(playlist.eliminar_cancion(&cancion).is_none());
        assert!(playlist.canciones.len()==1);

        let _=std::fs::remove_dir(path);
    }

    #[test]
    fn mover_cancion_fallida_por_falla_de_creacion_de_archivo_test(){
        let path="src/tp5/testeo_de_errores_mover_cancion";
        let _ = std::fs::remove_file(path);
        let _ = std::fs::remove_dir(path);

        //El archivo existe pero es un directorio, no se pueden escribir canciones en él
        fs::create_dir(path).unwrap();

        let mut playlist=Playlist::inner_new("Para llorar".to_string(),path.to_string());
        let cancion=Cancion::new("Bohemian Rhapsody".to_string(), "Queen".to_string(), Genero::Rock);
        let cancion2=Cancion::new("Smells Like Teen Spirit".to_string(),"Nirvana".to_string(),Genero::Rock);
        let cancion3=Cancion::new("Billie Jean".to_string(),"Michael Jackson".to_string(),Genero::Pop); 

        //agregué con push porque sino el agregar falla
        playlist.canciones.push(cancion);
        playlist.canciones.push(cancion2);
        playlist.canciones.push(cancion3.clone());

        assert!(!playlist.mover_cancion(&cancion3, 1));
        let _=std::fs::remove_dir(path);
    }

    #[test]
    fn vaciar_playlist_fallida_por_falla_de_creacion_de_archivo_test(){
        let path="src/tp5/testeo_de_errores_vaciar_playlist";
        let _ = std::fs::remove_file(path);
        let _ = std::fs::remove_dir(path);

        //El archivo existe pero es un directorio, no se pueden escribir canciones en él
        fs::create_dir(path).unwrap();

        let mut playlist=Playlist::inner_new("Para llorar".to_string(),path.to_string());
        let cancion=Cancion::new("Bohemian Rhapsody".to_string(), "Queen".to_string(), Genero::Rock);
        let cancion2=Cancion::new("Smells Like Teen Spirit".to_string(),"Nirvana".to_string(),Genero::Rock);
        let cancion3=Cancion::new("Billie Jean".to_string(),"Michael Jackson".to_string(),Genero::Pop); 

        //agregué con push porque sino el agregar falla
        playlist.canciones.push(cancion);
        playlist.canciones.push(cancion2);
        playlist.canciones.push(cancion3.clone());

        let _= playlist.vaciar_playlist();
        assert!(playlist.canciones.len()==3);
        let _=std::fs::remove_dir(path);
    }

}