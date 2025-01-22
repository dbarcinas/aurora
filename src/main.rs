mod app;
mod data;
mod events;
mod ui;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let terminal = ratatui::init();

    // Initialize the app
    let mut app = app::App::new();

    // Fetch SpaceX data before starting the app
    app.spacex_data = tokio::runtime::Runtime::new()?.block_on(async {
        crate::data::fetch_spacex_launches()
            .await
            .unwrap_or_default()
    });

    // Run the app
    let result = app.run(terminal);

    ratatui::restore();
    result
}
