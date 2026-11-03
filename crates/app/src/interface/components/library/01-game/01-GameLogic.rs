use crate::Juego;

pub fn recursos(juego: &mut Juego) {
    juego.banner = crate::library_body_logic::cargar_imagen("celeste-banner.png");
    juego.logo = crate::library_body_logic::cargar_imagen("celeste-logo.png");
    juego.personaje = crate::library_body_logic::cargar_imagen("celeste-personaje.png");
}