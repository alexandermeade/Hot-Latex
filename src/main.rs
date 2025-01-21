use std::fs;
use std::time::Duration;
use notify::{RecommendedWatcher, RecursiveMode, recommended_watcher};
use tokio::sync::mpsc;
use std::env;
use colored::*;


mod logger;
mod pin;
mod ui;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();  



    match logger::Logger::read_pins() {
        Ok(mut logger) => {
//            logger.toggle_pins();
            let home_options = vec![
            ui::SelectionOption::new(
                format!("Add new Group"),
                Box::new(|mut logger| {
                    logger.ui_add_pin(format!("./"));
                    logger.save_pins();                   
                })
                ),
            ];


            ui::selection_screen(home_options, logger);
        },
        Err(_) => {
            let mut logger = logger::Logger::new(); 
            logger.ui_add_pin(format!("./"));
            logger.save_pins();

        }
    }
}

