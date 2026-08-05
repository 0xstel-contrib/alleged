#![allow(clippy::single_component_path_imports)]
#![warn(clippy::unwrap_used)]

#[cfg(feature = "api")]
mod api;
#[cfg(feature = "caldav")]
mod caldav;
mod cli;
mod consts;
#[cfg(feature = "tui")]
mod tui;
#[cfg(feature = "api")]
pub use api::*;
#[cfg(feature = "caldav")]
pub use caldav::*;
pub use cli::*;
pub use consts::*;
#[cfg(feature = "tui")]
pub use tui::*;

#[cfg(feature = "api")]
use actix_web::{App, HttpServer, web};
use alleged_lib::graph::Graph;
use anyhow::{Result, anyhow};
use argh::FromArgs;
use std::env;
#[cfg(any(feature = "api", feature = "tui"))]
use std::sync::Arc;
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    let args_env: Vec<String> = env::args().collect();
    let mut args_env: Vec<&str> = args_env.iter().map(String::as_str).collect();
    let current_exe = args_env.remove(0);

    match Cli::from_args(&[current_exe], &args_env) {
        Ok(args) => {
            #[cfg(feature = "tui")]
            let command = args.command.unwrap_or_default();
            #[cfg(not(feature = "tui"))]
            let command = args.command;
            let graph = Graph::builder()
                .root(args.graph)
                .populate_ids()
                .exclude(args.exclude)
                .build()?;

            match command {
                #[cfg(feature = "api")]
                CliSubCommand::Api(api_cmd) => {
                    let graph = Arc::new(graph);

                    println!("Listening on {}:{}...", api_cmd.host, api_cmd.port);

                    HttpServer::new(move || {
                        App::new()
                            .app_data(web::Data::new(State {
                                graph: Arc::clone(&graph),
                            }))
                            .service(favicon)
                            .service(journal_append_block)
                    })
                    .bind((api_cmd.host, api_cmd.port))?
                    .run()
                    .await?;
                }
                #[cfg(feature = "caldav")]
                CliSubCommand::CalDavSync(caldav_cmd) => {
                    let password = env::var("CALDAV_PASS")
                        .expect("ERROR: environment variable `CALDAV_PASS` unset!");
                    caldav_cmd.sync(password, graph).await?;
                }
                #[cfg(feature = "tui")]
                CliSubCommand::Tui(_) => {
                    let graph = Arc::new(graph);
                    let mut app = Tui::new(&graph);

                    ratatui::run(|terminal| app.run(terminal))?;
                }
            }
        }
        Err(early_exit) => {
            if early_exit.status.is_ok() {
                println!("{}", early_exit.output.trim());
            } else {
                println!("{}", *HELP);
                return Err(anyhow!(early_exit.output.trim().to_string()));
            }
        }
    }

    Ok(())
}
