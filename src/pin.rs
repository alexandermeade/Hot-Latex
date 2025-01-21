use std::fs;
use std::fmt;

use serde::Serialize;
use serde::Deserialize;


use colored::ColoredString;
use std::io::Stdout;
use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;
use crossterm::event::Event;
use std::time::Duration;
use crossterm::event;
use crossterm::terminal;
use std::path::Path;
use std::rc::Rc;

use std::path::PathBuf;
use crate::logger;

use colored::Colorize;

use std::io;
use crate::ui;

#[derive(Serialize, Deserialize,Debug, Default, Clone)]
pub struct Pin {
    args: String,
    paths: Vec<logger::PathBuffer>,
    name: String,
    active: bool
}


impl fmt::Display for Pin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let result = format!("\n\t┍─{}──{}\n\t┖──\n\tgroup gets applied as latex {} [filename]", self.name.blue().bold(), 
            self.paths.clone()
            .into_iter()
            .map(
                |path| format!("\n\t│ {}", path.to_string())
            ).collect::<String>(),
            self.args
        );
        write!(f, "{}", result)
    }
}


impl Pin {

    pub fn new(paths: Vec<logger::PathBuffer>) -> Pin {
        Pin {
            args: String::new(),
            paths,
            name: String::new(),
            active: true
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn active(&self) -> bool {
        self.active
    }
   
    pub fn create_pin(path:&str) -> Pin {

        let paths = fs::read_dir(path)
            .unwrap()
            .map(|file| logger::PathBuffer::new(file.expect("unable to get path from file").path()))
            .collect::<Vec<_>>(); 

        let mut stdout = io::stdout();
        
        let header = "\nA pin group is a set of files that will have the same arguments applied to them at compile time.".normal().bold();
        let warning = "\nYou have to have at least one file selected for hot latex to hot reload your latex files. if this wasn't a mistake press enter again to confirm".green().bold();

        let pathbuffs = ui::check_box(
            paths, 
            |file| match file.0.extension() { 
                Some(ext) => ext == "latex" || ext == "tex",
                None => false
            },
            &stdout,
            header,
            warning,
            false,
            true
        );

        return Self::new(pathbuffs);        
    }

    pub fn set_vars(&mut self) -> &mut Self{

        let mut stdout = io::stdout();
        let mut pressed_twice = false;
        let mut cursor_index:usize = 0;
        let mut args = String::new();

        let header_msg =  format!("Set the arguments that will be appiled to this pin group on run time example: {}", "latex <inputed args> [file name inserted by hot-latex]".blue().bold());
        let input_msg = format!("input args: ");
        let empty_msg = format!("{}", "if you do not input anything it will compile without flags/args. if this isn't a mistake press enter again to proceed".green().bold());
        self.args = ui::get_input(&stdout, header_msg, input_msg, empty_msg);
        let mut name_input = String::new();
        loop { 

            if name_input != "" {
                break;
            }
            let header_msg = format!("Set the name of the group");
            let input_msg = format!("input group name: ");
            let empty_msg = format!("{}", "you cannot have a empty name".green().bold());
            name_input = ui::get_input(&stdout, header_msg, input_msg, empty_msg);
        }

        self.name = name_input;
        return self;
    }
}
