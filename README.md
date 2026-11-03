# Mithyia-Haven

> Tu santuario personal de juegos. *Donde el viento es libre.*

**Mithyia-Haven** es una biblioteca de juegos de escritorio, escrita en **Rust** y **Slint**, pensada para iniciar cualquier juego desde un solo lugar sin depender de los clientes de cada tienda. Está diseñada para ser fluida incluso en equipos modestos (objetivo: funcionar bien en un Intel i3 con 4 GB de RAM).

---

## Estado actual

El proyecto está en fase de **interfaz**. Ya funcionan:

- **Login "Atlas del viento"**: fondo ilustrado, corrientes de viento, semillas de diente de león y una rosa de los vientos donde cada carácter de la contraseña traza una línea del sigilo propio de cada usuario. Incluye botón para mostrar u ocultar la contraseña.
- **Navegación "Corriente de viento"**: el menú superior es una corriente ondulada con un nodo por sección; al cambiar de sección, una ráfaga de viento trae el nuevo contenido desde el lado elegido.
- **Biblioteca**: rejilla de 3 columnas con tarjetas de borde holográfico, paralaje, destello que sigue al cursor, barrido de luz, semillas y botón de inicio en la esquina inferior.
- **Vista cinematográfica de cada juego**: la tarjeta crece hasta ocupar la pantalla; banner con paralaje, logo, personaje en capas, datos del juego, constelación de sesiones y pestañas de Resumen, Lanzamiento y Notas.

> Los datos que se ven en las vistas de juego (horas, sesiones, descripción…) son **de ejemplo** mientras se construye la lógica.

---

## Próximo objetivo: la lógica en Rust

La siguiente etapa es dejar atrás los datos de ejemplo y construir el **núcleo en Rust**:

1. **Cuentas locales seguras**
   - Contraseñas con hash **Argon2id** y sal única por usuario.
   - Roles **admin** y **usuario** en una sola cuenta, con verificación extra para el modo admin.
   - Bloqueo progresivo tras intentos fallidos y clave de recuperación en el primer arranque.
2. **Persistencia con SQLite**
   - Usuarios, juegos, perfiles de lanzamiento, sesiones y notas.
   - Identificadores UUID, listos para sincronizar con un servidor en el futuro.
3. **Lanzamiento de juegos**
   - Ejecutar cualquier juego con su carpeta de trabajo, argumentos y variables de entorno.
   - Seguir el proceso real del juego para contar el tiempo jugado y mostrar el estado **en vivo**.
4. **Biblioteca real**
   - Añadir juegos a mano y detectarlos automáticamente (Steam, Lutris, Heroic).
   - Gestión de banner, logo y personaje de cada juego.
5. **Arquitectura preparada para crecer**
   - Una capa de autenticación y datos intercambiable, para pasar de local a servidor sin tocar la interfaz.

Más adelante: actividad desde el móvil (Discord y Atajos de iOS), color dinámico extraído de cada portada, modo cine y "mantener para jugar".

---

## Tecnología

| Capa | Herramienta |
|---|---|
| Lenguaje | Rust |
| Interfaz | Slint (renderizado `femtovg` / OpenGL, con respaldo por software) |
| Datos (próximamente) | SQLite con `rusqlite` |
| Seguridad (próximamente) | `argon2` |

Sin navegador embebido ni frameworks web: un solo framework de interfaz y dependencias mínimas.

---

## Estructura

```
crates/app/src/
├─ mithyia.rs                 punto de entrada
├─ Mithyia.slint              ventana raíz
├─ assets/                    imágenes
├─ ui/                        login (interfaz, lógica y estilo)
└─ interface/
   ├─ module/
   │  ├─ layout/              barra de navegación
   │  └─ body/
   │     └─ LibraryBody/      biblioteca y tarjetas de juego
   └─ components/
      └─ library/             vista interior de cada juego
```

Cada pieza se organiza en tres archivos: **interfaz** (`.slint`), **lógica** (`.rs`) y **estilo** (`Style.slint`).

---

## Compilar y ejecutar

Probado en **Linux Mint** (x86_64).

**1. Instalar Rust**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**2. Dependencias del sistema**
```bash
sudo apt install build-essential pkg-config libfontconfig1-dev libxkbcommon-dev libxkbcommon-x11-dev libx11-dev libxcursor-dev libxrandr-dev libxi-dev libgl1-mesa-dev
```

**3. Ejecutar**
```bash
cargo run -p mithyia-app --release
```

> Usa siempre `--release` para evaluar la fluidez: la versión de desarrollo no está optimizada.

### Imágenes de los juegos

Las imágenes de juegos de terceros **no se incluyen en el repositorio** porque pertenecen a sus estudios. Para verlas, colócalas en `crates/app/src/assets/` como PNG real:

| Archivo | Contenido |
|---|---|
| `celeste.png`, `genshin.png` | Portada de la tarjeta |
| `*-banner.png` | Banner de la vista del juego |
| `*-logo.png` | Logo con fondo transparente |
| `*-personaje.png` | Personaje recortado con fondo transparente |

Si falta alguna, la aplicación funciona igual con su alternativa.

---

## Licencia

© 2026 **@Mitsnightw** · Todos los derechos reservados.

Mithyia-Haven utiliza [Slint](https://slint.dev), que tiene su propia licencia.
