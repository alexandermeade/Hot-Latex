use colored::ColoredString;
use std::fmt::Display;
use crossterm::event::Event;
use crossterm::terminal::*;
use crossterm::*;

use std::sync::Arc;
use crossterm::event::*;
//use crossterm::event;
use crossterm::terminal;
use std::time::Duration;
use crossterm::event;
use std::io::Write;
use std::io::Stdout;
use colored::Colorize;

use crate::logger;

pub struct SelectionOption {
    content: String,
    func: Box<dyn Fn(logger::Logger)>,
} 

impl SelectionOption {
    pub fn new(content: String, func: Box<dyn Fn(logger::Logger)> ) -> SelectionOption {
        SelectionOption {
            content,
            func
        }
    }

    pub fn content(&self) -> String {
        self.content.clone()
    }

    pub fn run(&self, logger: logger::Logger) {
        (self.func)(logger);
    }
}

pub fn clear_term(mut stdout:&Stdout) {
    terminal::disable_raw_mode().unwrap();
    // Clear the terminal
    stdout.execute(terminal::Clear(ClearType::All)).unwrap();
    // Optionally move the cursor to the top-left
    stdout.execute(cursor::MoveTo(0, 0)).unwrap();
    stdout.flush().unwrap(); // Make sure everything gets written to the terminaw
}

pub fn select_screen(mut stdout: &Stdout, options: Arc<Vec<SelectionOption>>, logger:logger::Logger) {
    let mut index:usize = 0;
    let mut cursor_index = 0;
    let mut changed = true;

    loop {

        clear_term(&stdout);
        println!("\nPress {} to select, press {} or down arrow to move down, press {} or up arrow to move up. \n", 
            "enter".green().bold(), 
            "k".green().bold(), 
            "j".green().bold(), 
        );
        let options_iter = options.iter();
        for (i, option) in options_iter.enumerate() {
            if i == cursor_index { 
                println!("{}", option.content().black().bold().on_green());
                continue;
            }
            println!("{}", option.content().bold())
        }
     
        terminal::enable_raw_mode().unwrap();

        if event::poll(Duration::from_millis(100)).unwrap() {
            if let Event::Key(KeyEvent {code, modifiers, kind, state}) = event::read().unwrap(){
                match code {
                    KeyCode::Down | KeyCode::Char('k') => {
                        cursor_index = if cursor_index +1 >= options.len() {0} else {cursor_index + 1};
                    },
                    KeyCode::Up | KeyCode::Char('j') => {
                        cursor_index = if cursor_index <= 0 && options.len() > 0 {options.len()} else {cursor_index - 1};
                    },
                    KeyCode::Enter => {
                        clear_term(&stdout);
                        options[cursor_index].run(logger);
                        break;
                    },
                    _ => {

                    }
                }
            }
        }

    }
} 

pub fn confirm_screen(mut stdout: &Stdout, header:ColoredString, option1: ColoredString, option2: ColoredString) -> bool {

    let mut firstOption = true;
    let option1_result = option1.clone().black().on_green();
    let option2_result = option2.clone().black().on_red();
    loop {

        clear_term(&stdout);
        println!("\nPress {} to confirm your selection, press {} or left arrow to move left, press {} or right arrow to move right\n", 
            "enter".green().bold(), 
            "j".green().bold(), 
            "k".green().bold(), 
        );

        println!("\n{}", header);
         
        println!("{}", if firstOption {&option1_result} else {&option1});   

        println!("\r{}", if !firstOption {&option2_result} else {&option2});   
        
        terminal::enable_raw_mode().unwrap();

        if event::poll(Duration::from_millis(100)).unwrap() {
            if let Event::Key(KeyEvent {code, modifiers, kind, state}) = event::read().unwrap(){
                match code {
                    KeyCode::Left | KeyCode::Char('j') | KeyCode::Right | KeyCode::Char('k') => {
                        firstOption = !firstOption; 
                    },
                    KeyCode::Enter => { 
                        clear_term(&stdout);
                        return firstOption;
                    },
                    _ => {

                    }
                }
            }
        }
    }
    return false;
}

pub fn check_box<T, F>(items:Vec<T>, func:F, mut stdout:&Stdout,header: ColoredString, warning:ColoredString, checked:bool, use_match:bool, keep_unchecked:bool) -> Vec<T> 
where 
    F: Fn(T) -> bool,
    T: Clone + Display,
{


    let mut index:usize = 0;

    let mut changed = true;

     let mut items_tup = if use_match {
        items.clone()
        .into_iter()
        .map(|item| (item.clone(), func(item)))  // Clone the Path using to_path_buf()
        .collect::<Vec<_>>()
    } else {
         items.clone()
        .into_iter()
        .map(|item| (item, checked))  // Clone the Path using to_path_buf()
        .collect::<Vec<_>>()
    };


      
    /*let mut items_tup = items.clone()
        .into_iter()
        .map(|item| (item, checked))  // Clone the Path using to_path_buf()
        .collect::<Vec<_>>();
*/
    let mut pressed_twice = false;

    loop {

        clear_term(&stdout);
        println!("{}", header);
        println!("\nPress {} to select, press {} or down arrow to move down, press {} or up arrow to move up, press {} to confirm your choices. {} \n", 
            "s".green().bold(), 
            "k".green().bold(), 
            "j".green().bold(), 
            "enter".green().bold(),
            if pressed_twice {warning.clone()} else {"".normal()}
        );
     
        for (i, item) in items_tup.iter().enumerate() {    
            if !changed {
                changed = !changed
            } 

            let is_match = func(item.0.clone());/*match item.0.extension() {
                Some(ext) => ext == "latex" || ext == "tex",
                None => false
            };*/

            if is_match {
                println!("{} [{}] {}", 
                    item.0.to_string().green().bold(), 
                    if item.1 {"✓".green().bold()} else {"".normal()}, 
                    if i == index {"\t<-".bold()} else {"".normal()}
                );
                continue;
            }             
            println!("{} [{}] {}", 
                item.0.to_string().bold(), 
                if item.1 {"✓".green().bold()} else {"".normal()}, 
                if i == index {"\t<-".bold()} else {"".normal()}
            );
        }
         
        terminal::enable_raw_mode().unwrap();

        if event::poll(Duration::from_millis(500)).unwrap() {
            if let Event::Key(KeyEvent {code, modifiers, kind, state}) = event::read().unwrap(){
                match code {
                    KeyCode::Down | KeyCode::Char('k') => {
                        index = if index +1 >= items.len() {index} else {index + 1};
                    },
                    KeyCode::Up | KeyCode::Char('j') => {

                        index = if index <= 0 {0} else {index - 1};
                    },
                    KeyCode::Char('s') => {
                        pressed_twice = false;
                        items_tup[index].1 = !items_tup[index].1;
                    },                        
                    KeyCode::Enter => {
                        let mut count = 0;
                        for (file, state) in &items_tup {
                            count += if *state {1} else {0}
                        }

                        if count <= 0 && !pressed_twice {
                            pressed_twice = true;
                            continue; 
                        }

                        clear_term(&stdout);
                        break;
                    },
                    _ => {

                    }
                }
            }
        }

    }
    if !keep_unchecked {
        return items_tup 
            .into_iter()
            .filter(|(item, state)| *state)
            .map(|(item, _)| item)
            .collect::<Vec<T>>();        
    }
    
    return items_tup 
        .into_iter()
        .map(|(item, _)| item)
        .collect::<Vec<T>>();        

}

pub fn get_input(mut stdout:&Stdout, header_message:String, input_message:String, empty_container_msg:String) -> String {
     
     let mut index = 0; 
     let mut input = String::new();
     let mut cursor_index = 0;
     let mut pressed_twice = false;
    
     loop {
         if cursor_index >= input.len() && input.len() > 0 {
             cursor_index = input.len() - 1;
         }
        clear_term(&stdout);
        
        println!("{}", header_message);
        if pressed_twice {
            println!("{}", empty_container_msg);
        }
        print!("\n{}", input_message);

        for (i, c) in input.chars().into_iter().enumerate() {
            print!("{}", if i == cursor_index {c.to_string().underline()} else {c.to_string().normal()});
        }
        print!("\n");

        terminal::enable_raw_mode().unwrap();
        if event::poll(Duration::from_millis(500)).unwrap() {
            if let Event::Key(KeyEvent {code, modifiers, kind, state}) = event::read().unwrap(){
                match code {
                     KeyCode::Backspace => {
                        if cursor_index >= input.len() {
                            input.pop();
                            pressed_twice = false;
                            continue;
                        }
                        
                        let mut chars:Vec<_> = input.chars().collect();
                        chars.remove(cursor_index);
                        input = chars.into_iter().collect();

                        pressed_twice = false;
                        //input.pop();
                     },
                     KeyCode::Enter => {
                        let mut count = 0;
                        if input.len() <= 0 && !pressed_twice {
                            pressed_twice = true;
                            continue; 
                        }
                        clear_term(&stdout);
                        return input;
                    }, 
                    KeyCode::Left => {

                        pressed_twice = false;
                        cursor_index -= if cursor_index > 0 {1} else {0}
                    },
                    KeyCode::Right => {

                        pressed_twice = false;
                        cursor_index += if cursor_index >= input.len() {0} else {1}
                    },

                    KeyCode::Char(' ') => {
                        if cursor_index + 1 >= input.len() {
                            input.push_str(" ");
                            cursor_index += 1;

                            pressed_twice = false;
                            continue;
                        }
                        input.insert(cursor_index + 1, ' ');

                        pressed_twice = false;
                        cursor_index += 1;
                    }

                    KeyCode::Char(c) => {

                        if cursor_index + 1 >= input.len() {
                            input.push_str(&c.to_string());
                            cursor_index += 1;

                            pressed_twice = false;
                            continue;
                        }
                        input.insert(cursor_index + 1, c);

                        pressed_twice = false;
                        cursor_index += 1;
                    },
                    _ => {}
                }
            }
        } 
     
     }    
     return input;
}

