#[path = "ui/LoginLogic.rs"]
mod login_logic;

#[path = "interface/module/layout/LayoutLogic.rs"]
mod layout_logic;

#[path = "interface/module/body/BodyLogic.rs"]
mod body_logic;

#[path = "interface/module/body/LibraryBody/LibraryBodyLogic.rs"]
mod library_body_logic;

#[path = "interface/module/body/LibraryBody/01-CardGame/01-CardGameLogic.rs"]
mod card_game_logic;

#[path = "interface/module/body/LibraryBody/02-CardGame/02-CardGameLogic.rs"]
mod card_game_02_logic;

#[path = "interface/components/library/01-game/01-GameLogic.rs"]
mod game_01_logic;

#[path = "interface/components/library/02-game/02-GameLogic.rs"]
mod game_02_logic;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let app = AppWindow::new()?;
    login_logic::conectar(&app);
    layout_logic::conectar(&app);
    body_logic::conectar(&app);
    library_body_logic::conectar(&app);
    app.run()
}