use std::fs;
use std::time::Duration;
use notify::{RecommendedWatcher, RecursiveMode, recommended_watcher};
use tokio::sync::mpsc;
use std::env;
use colored::*;
use std::sync::Arc;
use std::io;
mod logger;
mod pin;
mod ui;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();  


    loop {
        match logger::Logger::read_pins() {
            Ok(mut logger) => {
//                logger.toggle_pins();
                let home_options = Arc::new(vec![

                ui::SelectionOption::new(
                    format!("Start Hot Latex"),
                    Box::new(|mut logger| {

                    })
                ),
                ui::SelectionOption::new(
                    format!("Add new Group"),
                    Box::new(|mut logger| {
                        logger.ui_add_pin(format!("./"));
                        logger.save_pins();                   
                    })
                ),
                ui::SelectionOption::new(
                    format!("keep/remove groups"),
                    Box::new(|mut logger| {
                        logger.toggle_pins();
                    })
                ),

                ui::SelectionOption::new(
                    format!("view pins"),
                    Box::new(|mut logger| {
                        logger.view_pins();
                    })
                ), 

                ui::SelectionOption::new(
                    format!("exit program"),
                    Box::new(|mut logger| {
                        std::process::exit(0);
                    })
                ),
            ]);
        

            ui::select_screen(&mut io::stdout(), Arc::clone(&home_options), logger.clone());
            
        },
        Err(_) => {
            let mut logger = logger::Logger::new(); 
            logger.save_pins();

        }
    }
    }
}

