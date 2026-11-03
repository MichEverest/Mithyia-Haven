use crate::{AppWindow, OpcionMenu};
use slint::{ComponentHandle, ModelRc, VecModel};

pub fn conectar(app: &AppWindow) {
    let debil = app.as_weak();
    app.on_opcion_seleccionada(move |indice| {
        if let Some(app) = debil.upgrade() {
            let anterior = app.get_opcion_activa();
            if indice == anterior {
                return;
            }
            let direccion = if indice > anterior { 1 } else { -1 };
            app.set_opcion_activa(indice);
            crate::body_logic::transicion(&app, direccion);
        }
    });

    app.on_busqueda_editada(|_texto| {});

    let debil = app.as_weak();
    app.on_cerrar_sesion(move || {
        if let Some(app) = debil.upgrade() {
            app.set_sesion_iniciada(false);
            app.set_busqueda("".into());
            crate::login_logic::reiniciar(&app);
        }
    });
}

pub fn abrir_sesion(app: &AppWindow, nombre: &str, es_admin: bool) {
    let nombre = nombre.trim();
    let inicial: String = nombre
        .chars()
        .next()
        .map(|letra| letra.to_uppercase().collect())
        .unwrap_or_default();

    app.set_nombre_usuario(nombre.into());
    app.set_inicial_usuario(inicial.into());
    app.set_opciones(ModelRc::new(VecModel::from(opciones(es_admin))));
    app.set_opcion_activa(0);
    app.set_progreso_transicion(1.0);
    app.set_sesion_iniciada(true);
}

fn opciones(es_admin: bool) -> Vec<OpcionMenu> {
    let mut lista = vec![
        opcion("biblioteca", "BIBLIOTECA"),
        opcion("colecciones", "COLECCIONES"),
        opcion("anadir", "AÑADIR JUEGO"),
        opcion("dispositivos", "DISPOSITIVOS"),
        opcion("ajustes", "AJUSTES"),
    ];

    if es_admin {
        lista.push(opcion("administracion", "ADMINISTRACIÓN"));
    }

    lista
}

fn opcion(id: &str, titulo: &str) -> OpcionMenu {
    OpcionMenu {
        id: id.into(),
        titulo: titulo.into(),
    }
}