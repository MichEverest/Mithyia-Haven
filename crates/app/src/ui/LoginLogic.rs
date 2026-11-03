use crate::{AppWindow, Trazo};
use slint::{ComponentHandle, Model, ModelRc, Timer, VecModel};
use std::f32::consts::{FRAC_PI_2, TAU};
use std::time::Duration;

pub fn conectar(app: &AppWindow) {
    let debil = app.as_weak();
    app.on_usuario_editado(move |texto| {
        if let Some(app) = debil.upgrade() {
            let trazos = generar_sigilo(&texto);
            let total = trazos.len();
            app.set_trazos(ModelRc::new(VecModel::from(trazos)));
            let escritos = app.get_contrasena().chars().count();
            app.set_encendidos(escritos.min(total) as i32);
        }
    });

    let debil = app.as_weak();
    app.on_contrasena_editada(move |texto| {
        if let Some(app) = debil.upgrade() {
            let total = app.get_trazos().row_count();
            app.set_encendidos(texto.chars().count().min(total) as i32);
        }
    });

    let debil = app.as_weak();
    app.on_iniciar_sesion(move |usuario, _contrasena| {
        if let Some(app) = debil.upgrade() {
            app.set_cargando(true);
            app.set_acceso_concedido(true);
            let total = app.get_trazos().row_count();
            app.set_encendidos(total as i32);

            let debil = app.as_weak();
            let nombre = usuario.to_string();
            Timer::single_shot(Duration::from_millis(900), move || {
                if let Some(app) = debil.upgrade() {
                    crate::layout_logic::abrir_sesion(&app, &nombre, false);
                }
            });
        }
    });
}

pub fn reiniciar(app: &AppWindow) {
    app.set_usuario("".into());
    app.set_contrasena("".into());
    app.set_mensaje_error("".into());
    app.set_cargando(false);
    app.set_trazos(ModelRc::new(VecModel::from(Vec::<Trazo>::new())));
    app.set_encendidos(0);
    app.set_acceso_concedido(false);
}

fn generar_sigilo(usuario: &str) -> Vec<Trazo> {
    let nombre = usuario.trim().to_lowercase();
    if nombre.is_empty() {
        return Vec::new();
    }

    let mut semilla: u64 = 0xcbf29ce484222325;
    for byte in nombre.bytes() {
        semilla ^= byte as u64;
        semilla = semilla.wrapping_mul(0x100000001b3);
    }

    let puntos: Vec<(f32, f32)> = (0..9)
        .map(|i| {
            if i == 8 {
                (120.0, 120.0)
            } else {
                let angulo = i as f32 * TAU / 8.0 - FRAC_PI_2;
                (120.0 + 72.0 * angulo.cos(), 120.0 + 72.0 * angulo.sin())
            }
        })
        .collect();

    let mut trazos = Vec::with_capacity(12);
    let mut actual = 8usize;

    for _ in 0..12 {
        semilla ^= semilla >> 33;
        semilla = semilla.wrapping_mul(0xff51afd7ed558ccd);
        semilla ^= semilla >> 33;

        let mut siguiente = (semilla % 9) as usize;
        if siguiente == actual {
            siguiente = (siguiente + 1) % 9;
        }

        let (x1, y1) = puntos[actual];
        let (x2, y2) = puntos[siguiente];
        trazos.push(Trazo { x1, y1, x2, y2 });
        actual = siguiente;
    }

    trazos
}