#[derive(Debug,Clone)]
#[allow(unused)]
struct Cancion{
    titulo:String,
    artista:String,
    genero:Genero,
}

#[derive(Debug,Clone)]
#[allow(unused)]
enum Genero{
    Rock,
    Pop,
    Rap,
    Jazz,
    Otros,
}

#[derive(Debug,Clone)]
#[allow(unused)]
struct Playlist{
    nombre:String,
    canciones:Vec<Cancion>,
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
        Playlist {nombre, canciones: Vec::new() }
    }

    pub fn agregar_cancion(&mut self,cancion:Cancion){
        self.canciones.push(cancion);
    }

    pub fn eliminar_cancion(&mut self,cancion:Cancion)->Cancion{
        let mut cancion_buscada=self.clone().buscar_cancion(cancion.clone().titulo);

        if cancion.clone().misma_cancion(&cancion_buscada){
            let mut indice=0;
            let mut eliminada=false;
            while !eliminada && indice<self.canciones.len(){
                if self.canciones[indice].clone().misma_cancion(&cancion){
                    cancion_buscada=self.canciones.remove(indice);
                    eliminada=true;
                }
                indice+=1;
            }
        }
        cancion_buscada
    }

    pub fn buscar_cancion(self,nombre:String)->Cancion{
        let mut indice:usize=0;
        while indice<self.canciones.len(){
            if self.canciones[indice].titulo==nombre{
                return self.canciones[indice].clone();
            }
            indice+=1;
        }
        panic!("La cancion no se encuentra en la lista");
    }

    pub fn mover_cancion(&mut self,cancion:Cancion, posicion:usize){
        let cancion_buscada=self.clone().buscar_cancion(cancion.clone().titulo);
        if cancion_buscada.misma_cancion(&cancion) && posicion<self.canciones.len(){
            let cancion_a_mover=self.eliminar_cancion(cancion);
            self.canciones.insert(posicion,cancion_a_mover);
        }else if posicion>=self.canciones.len() {
            panic!();
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
        self.canciones.clear();
    }

}

#[cfg(test)]
mod test{
    use crate::tp3::ej8::{Cancion,Genero,Playlist};

    #[test]
    fn agregar_cancion_test(){
        let mut playlist=Playlist::new("Para llorar".to_string());
        let cancion=Cancion::new("Bohemian Rhapsody".to_string(), "Queen".to_string(), Genero::Rock);
        assert!(playlist.canciones.is_empty());
        playlist.agregar_cancion(cancion);
        assert_eq!(playlist.canciones.len(),1);
    }

    #[test]
    fn eliminar_cancion_test(){
        let mut playlist=Playlist::new("Para llorar".to_string());
        let cancion=Cancion::new("Bohemian Rhapsody".to_string(), "Queen".to_string(), Genero::Rock);
        let cancion2=Cancion::new("Smells Like Teen Spirit".to_string(),"Nirvana".to_string(),Genero::Rock);
        let cancion3=Cancion::new("Billie Jean".to_string(),"Michael Jackson".to_string(),Genero::Pop);
        playlist.agregar_cancion(cancion.clone());
        playlist.agregar_cancion(cancion2.clone());
        playlist.agregar_cancion(cancion3.clone());

        assert_eq!(playlist.canciones.len(),3);
        playlist.eliminar_cancion(cancion2);
        assert_eq!(playlist.canciones.len(),2);
        assert!(playlist.canciones[0].clone().misma_cancion(&cancion));
        assert!(playlist.canciones[1].clone().misma_cancion(&cancion3));      
    }

    #[test]
    #[should_panic(expected="La cancion no se encuentra en la lista")]
    fn eliminar_cancion_inexistente_test(){
        let mut playlist=Playlist::new("Para llorar".to_string());
        let cancion=Cancion::new("Bohemian Rhapsody".to_string(), "Queen".to_string(), Genero::Rock);
        let cancion2=Cancion::new("Smells Like Teen Spirit".to_string(),"Nirvana".to_string(),Genero::Rock);
        let cancion3=Cancion::new("Billie Jean".to_string(),"Michael Jackson".to_string(),Genero::Pop);
        playlist.agregar_cancion(cancion);
        playlist.agregar_cancion(cancion3);

        playlist.eliminar_cancion(cancion2);   
    }

    #[test]
    #[should_panic(expected="La cancion no se encuentra en la lista")]
    fn eliminar_cancion_playlist_vacia_test(){
        let mut playlist=Playlist::new("Para llorar".to_string());
        let cancion=Cancion::new("Bohemian Rhapsody".to_string(), "Queen".to_string(), Genero::Rock);

        playlist.eliminar_cancion(cancion);   
    }

    #[test]
    fn mover_cancion_para_arriba_test(){
        let mut playlist=Playlist::new("Para llorar".to_string());
        let cancion=Cancion::new("Bohemian Rhapsody".to_string(), "Queen".to_string(), Genero::Rock);
        let cancion2=Cancion::new("Smells Like Teen Spirit".to_string(),"Nirvana".to_string(),Genero::Rock);
        let cancion3=Cancion::new("Billie Jean".to_string(),"Michael Jackson".to_string(),Genero::Pop);
        playlist.agregar_cancion(cancion.clone());
        playlist.agregar_cancion(cancion2.clone());
        playlist.agregar_cancion(cancion3.clone());

        assert!(playlist.canciones[0].clone().misma_cancion(&cancion));
        assert!(playlist.canciones[1].clone().misma_cancion(&cancion2));
        assert!(playlist.canciones[2].clone().misma_cancion(&cancion3));      
        
        playlist.mover_cancion(cancion2.clone(),0);

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
        playlist.agregar_cancion(cancion.clone());
        playlist.agregar_cancion(cancion2.clone());
        playlist.agregar_cancion(cancion3.clone());

        assert!(playlist.canciones[0].clone().misma_cancion(&cancion));
        assert!(playlist.canciones[1].clone().misma_cancion(&cancion2));
        assert!(playlist.canciones[2].clone().misma_cancion(&cancion3));      
        
        playlist.mover_cancion(cancion2.clone(),2);

        assert!(playlist.canciones[0].clone().misma_cancion(&cancion));
        assert!(playlist.canciones[1].clone().misma_cancion(&cancion3));      
        assert!(playlist.canciones[2].clone().misma_cancion(&cancion2));
    }

    #[test]
    #[should_panic]
    fn mover_cancion_posicion_fuera_de_rango_test(){
        let mut playlist=Playlist::new("Para llorar".to_string());
        let cancion=Cancion::new("Bohemian Rhapsody".to_string(), "Queen".to_string(), Genero::Rock);
        let cancion2=Cancion::new("Smells Like Teen Spirit".to_string(),"Nirvana".to_string(),Genero::Rock);
        let cancion3=Cancion::new("Billie Jean".to_string(),"Michael Jackson".to_string(),Genero::Pop);
        playlist.agregar_cancion(cancion.clone());
        playlist.agregar_cancion(cancion2.clone());
        playlist.agregar_cancion(cancion3.clone());    
        
        playlist.mover_cancion(cancion2.clone(),7);
    }

    #[test]
    #[should_panic(expected="La cancion no se encuentra en la lista")]
    fn mover_cancion_inexistente_test(){
        let mut playlist=Playlist::new("Para llorar".to_string());
        let cancion=Cancion::new("Bohemian Rhapsody".to_string(), "Queen".to_string(), Genero::Rock);
        let cancion2=Cancion::new("Smells Like Teen Spirit".to_string(),"Nirvana".to_string(),Genero::Rock);
        let cancion3=Cancion::new("Billie Jean".to_string(),"Michael Jackson".to_string(),Genero::Pop);
        playlist.agregar_cancion(cancion.clone());
        playlist.agregar_cancion(cancion3.clone());    
        
        playlist.mover_cancion(cancion2,0);
    }

    #[test]
    #[should_panic(expected="La cancion no se encuentra en la lista")]
    fn mover_cancion_playlist_vacia_test(){
        let mut playlist=Playlist::new("Para llorar".to_string());
        let cancion=Cancion::new("Bohemian Rhapsody".to_string(), "Queen".to_string(), Genero::Rock);
        
        playlist.mover_cancion(cancion,3);
    }

    #[test]
    fn buscar_cancion_test(){
        let mut playlist=Playlist::new("Para llorar".to_string());
        let cancion=Cancion::new("Bohemian Rhapsody".to_string(), "Queen".to_string(), Genero::Rock);
        let cancion2=Cancion::new("Smells Like Teen Spirit".to_string(),"Nirvana".to_string(),Genero::Rock);
        let cancion3=Cancion::new("Billie Jean".to_string(),"Michael Jackson".to_string(),Genero::Pop);
        playlist.agregar_cancion(cancion);
        playlist.agregar_cancion(cancion2);
        playlist.agregar_cancion(cancion3.clone());

        assert!(playlist.buscar_cancion("Billie Jean".to_string()).misma_cancion(&cancion3));
    }

    #[test]
    #[should_panic(expected="La cancion no se encuentra en la lista")]
    fn buscar_cancion_inexistente_test(){
        let mut playlist=Playlist::new("Para llorar".to_string());
        let cancion=Cancion::new("Bohemian Rhapsody".to_string(), "Queen".to_string(), Genero::Rock);
        let cancion3=Cancion::new("Billie Jean".to_string(),"Michael Jackson".to_string(),Genero::Pop);
        playlist.agregar_cancion(cancion);
        playlist.agregar_cancion(cancion3);

        playlist.buscar_cancion("Smells Like Teen Spirit".to_string());
    }

    #[test]
    #[should_panic(expected="La cancion no se encuentra en la lista")]
    fn buscar_cancion_playlist_vacia_test(){
        let playlist=Playlist::new("Para llorar".to_string());

        playlist.buscar_cancion("Smells Like Teen Spirit".to_string());
    }

    #[test]
    fn obtener_canciones_genero_test(){
        let mut playlist=Playlist::new("Para llorar".to_string());
        let cancion=Cancion::new("Bohemian Rhapsody".to_string(), "Queen".to_string(), Genero::Rock);
        let cancion2=Cancion::new("Smells Like Teen Spirit".to_string(),"Nirvana".to_string(),Genero::Rock);
        let cancion3=Cancion::new("Billie Jean".to_string(),"Michael Jackson".to_string(),Genero::Pop);
        playlist.agregar_cancion(cancion.clone());
        playlist.agregar_cancion(cancion2.clone());
        playlist.agregar_cancion(cancion3);

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
        playlist.agregar_cancion(cancion);
        playlist.agregar_cancion(cancion2);
        playlist.agregar_cancion(cancion3);

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
        playlist.agregar_cancion(cancion.clone());
        playlist.agregar_cancion(cancion2);
        playlist.agregar_cancion(cancion3);
        playlist.agregar_cancion(cancion4.clone());

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
        playlist.agregar_cancion(cancion);
        playlist.agregar_cancion(cancion2);
        playlist.agregar_cancion(cancion3);

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
        playlist.agregar_cancion(cancion.clone());
        playlist.agregar_cancion(cancion2.clone());
        playlist.agregar_cancion(cancion3.clone());  

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

}