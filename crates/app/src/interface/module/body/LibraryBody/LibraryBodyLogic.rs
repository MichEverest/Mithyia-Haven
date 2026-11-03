use crate::{AppWindow, Estrella, Juego};
use slint::{Color, ComponentHandle, Image, ModelRc, Timer, TimerMode, VecModel};
use std::cell::Cell;
use std::path::Path;
use std::time::{Duration, Instant};

const DURACION: Duration = Duration::from_millis(600);

thread_local! {
    static TEMPORIZADOR: Timer = Timer::default();
    static INICIO: Cell<Option<Instant>> = const { Cell::new(None) };
}

pub fn conectar(app: &AppWindow) {
    app.set_juegos(ModelRc::new(VecModel::from(juegos_de_ejemplo())));

    let debil = app.as_weak();
    app.on_abrir_juego(move |indice, x, y, ancho, alto| {
        if let Some(app) = debil.upgrade() {
            app.set_origen_x(x);
            app.set_origen_y(y);
            app.set_origen_ancho(ancho);
            app.set_origen_alto(alto);
            app.set_progreso_apertura(0.0);
            app.set_juego_abierto(indice);
            animar(&app, true);
        }
    });

    let debil = app.as_weak();
    app.on_cerrar_juego(move || {
        if let Some(app) = debil.upgrade() {
            animar(&app, false);
        }
    });

    app.on_iniciar_juego(|_indice| {});
}

pub fn cargar_imagen(archivo: &str) -> Image {
    let ruta = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src/assets")
        .join(archivo);
    Image::load_from_path(&ruta).unwrap_or_default()
}

pub fn constelacion(nombre: &str, horas: &[f32]) -> ModelRc<Estrella> {
    let mut semilla: u64 = 0xcbf29ce484222325;
    for byte in nombre.to_lowercase().bytes() {
        semilla ^= byte as u64;
        semilla = semilla.wrapping_mul(0x100000001b3);
    }

    let maximo = horas.iter().cloned().fold(0.0_f32, f32::max).max(0.1);
    let total = horas.len().max(1);
    let ultima = horas.len().saturating_sub(1);

    let estrellas: Vec<Estrella> = horas
        .iter()
        .enumerate()
        .map(|(indice, &duracion)| {
            semilla ^= semilla >> 33;
            semilla = semilla.wrapping_mul(0xff51afd7ed558ccd);
            semilla ^= semilla >> 33;
            let desvio = (semilla % 1000) as f32 / 1000.0;
            let avance = if total > 1 { indice as f32 / (total - 1) as f32 } else { 0.5 };

            Estrella {
                x: 6.0 + avance * 88.0,
                y: 14.0 + desvio * 72.0,
                tamano: 0.3 + 0.7 * duracion / maximo,
                ultima: indice == ultima,
            }
        })
        .collect();

    ModelRc::new(VecModel::from(estrellas))
}

fn animar(app: &AppWindow, abrir: bool) {
    INICIO.with(|inicio| inicio.set(Some(Instant::now())));

    let debil = app.as_weak();
    TEMPORIZADOR.with(|temporizador| {
        temporizador.start(TimerMode::Repeated, Duration::from_millis(16), move || {
            let Some(app) = debil.upgrade() else {
                return;
            };
            let inicio = INICIO.with(|inicio| inicio.get()).unwrap_or_else(Instant::now);
            let lineal = (inicio.elapsed().as_secs_f32() / DURACION.as_secs_f32()).min(1.0);
            let suave = 1.0 - (1.0 - lineal).powi(3);
            app.set_progreso_apertura(if abrir { suave } else { 1.0 - suave });
            if lineal >= 1.0 {
                if !abrir {
                    app.set_juego_abierto(-1);
                }
                TEMPORIZADOR.with(|temporizador| temporizador.stop());
            }
        });
    });
}

fn juegos_de_ejemplo() -> Vec<Juego> {
    let mut celeste = juego(
        "01-game",
        "CELESTE",
        "celeste.png",
        Color::from_rgb_u8(0x7F, 0xD3, 0xF7),
        1,
    );
    crate::game_01_logic::recursos(&mut celeste);

    let mut genshin = juego(
        "02-game",
        "GENSHIN IMPACT",
        "genshin.png",
        Color::from_rgb_u8(0xE8, 0xC4, 0x74),
        2,
    );
    crate::game_02_logic::recursos(&mut genshin);

    vec![celeste, genshin]
}

fn juego(id: &str, titulo: &str, archivo: &str, acento: Color, diseno: i32) -> Juego {
    Juego {
        id: id.into(),
        titulo: titulo.into(),
        portada: cargar_imagen(archivo),
        acento,
        ultima_vez: "".into(),
        horas: "".into(),
        diseno,
        banner: Image::default(),
        logo: Image::default(),
        personaje: Image::default(),
        generos: "".into(),
        descripcion: "".into(),
        desarrollador: "".into(),
        lanzamiento: "".into(),
        plataforma: "".into(),
        tamano: "".into(),
        tiempo_total: "".into(),
        ultima_sesion: "".into(),
        sesiones: "".into(),
        en_vivo: "".into(),
        ejecutable: "".into(),
        argumentos: "".into(),
        notas: "".into(),
        estrellas: ModelRc::default(),
    }
}