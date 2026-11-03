use crate::Juego;

pub fn recursos(juego: &mut Juego) {
    juego.banner = crate::library_body_logic::cargar_imagen("genshin-banner.png");
    juego.logo = crate::library_body_logic::cargar_imagen("genshin-logo.png");
    juego.personaje = crate::library_body_logic::cargar_imagen("genshin-personaje.png");

    juego.ultima_vez = "Hace 2 días".into();
    juego.horas = "312 h".into();
    juego.generos = "RPG · MUNDO ABIERTO · AVENTURA".into();
    juego.descripcion = "Explora Teyvat, un mundo de siete naciones guiadas por sus dioses, en busca de tu hermano perdido. Combina elementos, forma equipos y descubre los secretos de cada región.".into();
    juego.desarrollador = "HoYoverse".into();
    juego.lanzamiento = "28 sep 2020".into();
    juego.plataforma = "PC · iPhone".into();
    juego.tamano = "—".into();
    juego.tiempo_total = "312 h".into();
    juego.ultima_sesion = "1 h 20 min".into();
    juego.sesiones = "148".into();
    juego.en_vivo = "Sin actividad".into();
    juego.ejecutable = "GenshinImpact.exe".into();
    juego.argumentos = "—".into();
    juego.notas = "".into();

    let historial = [1.2, 0.6, 2.4, 1.8, 0.4, 3.1, 1.0, 2.2, 0.8, 1.6, 2.9, 1.3];
    juego.estrellas = crate::library_body_logic::constelacion("genshin impact", &historial);
}