use std::path::Path;
use std::io::Stdout;

use std::io::stdout;
use serde::Serialize;
use serde::Deserialize;
use serde::ser::Error;
use std::cell::RefCell;
use std::rc::Rc;
use crate::pin::Pin;
use crossterm::{
    event::{self, KeyCode, KeyEvent, KeyModifiers, Event},
    ExecutableCommand,
    terminal::{self, ClearType},
    cursor,
};
use std::{io::{self, Write}, thread, time::Duration}; 

use colored::*;
use std::fs;

use crate::pin;
use crate::ui;

use std::fmt;

#[derive(Deserialize, Serialize, Clone)]
pub struct Logger {
   pub pins: Vec<pin::Pin>,
} 

use std::path::PathBuf;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PathBuffer(pub PathBuf);

impl fmt::Display for PathBuffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.to_string_lossy())
    }
}

impl PathBuffer {
    pub fn new(path: PathBuf) -> Self {
        PathBuffer(path)
    }
}



impl Logger {

    pub fn new() -> Logger {
        Logger {
            pins: Vec::new()
        }
    }
    
    pub fn read_pins() -> Result<Logger, Box<dyn std::error::Error>> {
        let file_contents = fs::read_to_string("./HotLatex-config.json")
            .map_err(|e| serde_json::Error::custom(e.to_string()))?;

        return Ok(serde_json::from_str(&file_contents)?);
    }


    pub fn view_pins(&mut self) {

        let header = 
            format!("List of pins: \n{}", 
                format!("\n{}", self.pins.clone().into_iter().map(|pin| pin.to_string()).collect::<String>())).bold();
        let option1 = "exit".green().bold();
        let option2 = "exit".green().bold();
        
        if ui::confirm_screen(&mut io::stdout(), header, option1, option2) {
            self.pins.clear();
        }
    }

    pub fn toggle_pins(&mut self) {

        if self.pins.len() <= 0 {
            return;
        }

        let stdout = io::stdout();
        let header = format!("Toggle what groups you want to keep active").green().bold();
        let warning = "if you have zero groups picked nothing will be reloaded. press enter again to continue ".bold();

        self.pins = ui::check_box(
            self.pins.clone(), 
            |pin_tuple| {pin_tuple.active()}, 
            &stdout,
            header,
            warning,
            false, //check defualt
            true, //check from function
            false // keep thrown out options
        ); 
        
        self.save_pins();
    }

    pub fn clear_vec(&mut self) {

        let header = "If you want to clear the current listings of pins press confirm else press decline".bold();
        let option1 = "Confirm ✓".green().bold();
        let option2 = "Decline X".red().bold();
        if ui::confirm_screen(&mut io::stdout(), header, option1, option2) {
            self.pins.clear();
        }
    }

    pub fn save_pins(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let json_data = serde_json::to_string_pretty(&self)?;
        fs::write("./HotLatex-config.json", json_data).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
        Ok(())
    }

    pub fn ui_add_pin(&mut self, path: String) {
        let mut stdout = io::stdout();
        let pin_path = path.clone();
        // Create the pin and set its variables
        let mut pin = pin::Pin::create_pin(&pin_path);  // Make sure to clone pin_path if necessary
        pin.set_vars();  // Set variables after pin creation
        
        let pin_str = pin.to_string().clone();  // To string representation of the pin
        let header = format!("Confirm if you want to keep the group\n {} ", pin_str).normal();
        let option1 = "Confirm ✓".green().bold();
        let option2 = "Decline X".red().bold();

        // Call confirm_screen UI function
        if ui::confirm_screen(&stdout, header, option1.clone(), option2.clone()) {
            // Take ownership of the pin by using `std::mem::take` (make sure it's mutable)

            for (i, p) in self.pins.clone().into_iter().enumerate() {
                if p.name() == pin.name() {
                     let header2 = format!("there's another group that shares the same name as the one you have created do you want to remove the other group and use this one instead? \n group 1:\n {} \n group 2: \n {}", p.to_string(), pin.to_string()).bold();
                    if ui::confirm_screen(&stdout, header2, option1.clone(), option2.clone()) {
                        self.pins.remove(i);
                    }
                }
            }
            self.pins.push(std::mem::take(&mut pin));
        }
    }

}
