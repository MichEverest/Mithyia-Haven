use crate::AppWindow;
use slint::{ComponentHandle, Timer, TimerMode};
use std::cell::Cell;
use std::time::{Duration, Instant};

const DURACION: Duration = Duration::from_millis(650);

thread_local! {
    static TEMPORIZADOR: Timer = Timer::default();
    static INICIO: Cell<Option<Instant>> = const { Cell::new(None) };
}

pub fn conectar(app: &AppWindow) {
    app.set_progreso_transicion(1.0);
}

pub fn transicion(app: &AppWindow, direccion: i32) {
    app.set_direccion_transicion(direccion);
    app.set_progreso_transicion(0.0);
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
            app.set_progreso_transicion(suave);
            if lineal >= 1.0 {
                TEMPORIZADOR.with(|temporizador| temporizador.stop());
            }
        });
    });
}