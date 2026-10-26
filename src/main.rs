mod cli;
mod drag;
mod ui;

use gtk::gio;
use gtk::prelude::*;

const APP_ID: &str = "io.github.joshuarosato.Dragon";

fn main() {
    let cli = match cli::Cli::parse() {
        Ok(cli) => cli,
        Err(cli::CliExit::Success) => return,
        Err(cli::CliExit::Failure(message)) => {
            eprintln!("dragon: {message}");
            eprintln!("Try 'dragon --help' for usage.");
            std::process::exit(2);
        }
    };

    let app = adw::Application::builder()
        .application_id(APP_ID)
        .flags(gio::ApplicationFlags::NON_UNIQUE)
        .build();

    app.connect_activate(move |app| {
        ui::build_window(app, &cli.paths, cli.keep_open);
    });

    // The CLI has already consumed the process arguments. Passing only argv[0]
    // prevents GApplication from trying to interpret file names as its own args.
    app.run_with_args(&["dragon"]);
}
