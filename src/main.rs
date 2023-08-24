extern crate chrono;
use chrono::prelude::*;
use std::env;
use std::fs::{ReadDir, Metadata};
use std::path::PathBuf;
use std::time::SystemTime;
use eframe::egui;
use open;

mod utils;

struct FileDetailsState {
    file_path: PathBuf,
    metadata: Option<Metadata>,
    show_window: bool,
}

fn format_system_time(system_time: Result<SystemTime, std::io::Error>) -> String {
    match system_time {
        Ok(time) => {
            let datetime: DateTime<Local> = time.into();
            datetime.format("%m/%d/%Y %I:%M %p").to_string()
        }
        Err(_) => String::from("Error fetching time"),
    }
}

fn format_type(metadata: &Metadata) -> String {
    if metadata.is_dir() {
        "Directory".to_string()
    } else if metadata.is_file() {
        "File".to_string()
    } else if metadata.is_symlink() {
        "Symlink".to_string()
    } else {
        "Unknown".to_string()
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };

    let mut search_str: String = String::default();
    let mut prev_dirs: Vec<PathBuf> = Vec::new();
    let mut new_dir_path: PathBuf = env::current_dir().unwrap();
    let mut file_details_state: Option<FileDetailsState> = None;

    eframe::run_simple_native("Oxidized Explorer", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal_top(|hui| {
                if hui.button("<<<").clicked() {
                    if let Some(last_dir) = prev_dirs.pop() {
                        new_dir_path = last_dir;
                    }
                }
                hui.text_edit_singleline(&mut search_str);
                if hui.button("Search").clicked() {
                    println!("Do search stuff!");
                }
            });
            // cur_dir = utils::get_dir_from_file(std::path::PathBuf)
            let cur_dir: ReadDir = match utils::get_dir_from_file(&new_dir_path) {
                Ok(dir) => dir,
                Err(err) => {
                    println!("Error reading directory: {}", err);
                    return;
                }
            };

            for file in cur_dir {
                let file = match file {
                    Ok(f) => f,
                    Err(err) => {
                        println!("Error reading file: {}", err);
                        continue;
                    }
                };

                if let Some(file_name) = file.file_name().into_string().ok() {
                    if search_str.is_empty() || file_name.contains(&search_str) {
                        ui.horizontal(|lui| {
                            if lui.button(&file_name).clicked() {
                                println!("{} Was clicked", &file_name);
                                if file.metadata().unwrap().is_dir() {
                                    prev_dirs.push(new_dir_path.clone());
                                    new_dir_path = file.path();
                                } else {
                                    let _ = open::that(file.path().to_str().unwrap());
                                }
                            }

                            let mod_time: DateTime<Local> =
                                file.metadata().unwrap().modified().unwrap().into();
                            lui.label(mod_time.format("%m/%d/%Y %I:%M %p").to_string());

                            if lui.button("Show Details").clicked() {
                                if let Ok(metadata) = utils::get_file_details(&file.path()) {
                                    file_details_state = Some(FileDetailsState {
                                        file_path: file.path(),
                                        metadata: Some(metadata),
                                        show_window: true,
                                    });
                                }
                            }
                        });
                    }
                }
            }
        }); 
        
        
        // File Details WIndow
        if let Some(file_details) = &mut file_details_state {
            if file_details.show_window {
                let show_window = &mut file_details.show_window;

                egui::Window::new("File Details")
                    .default_width(300.0)
                    .resizable(false)
                    .open(show_window)
                    .show(ctx, |ui| {
                        if let Some(metadata) = &file_details.metadata {
                            let attributes: Vec<(&str, fn(&Metadata) -> String)> = vec![
                                ("Size", |md| format!("{}", md.len())),
                                ("Permissions", |md| format!("{:?}", md.permissions())),
                                ("Created", |md| format_system_time(md.created())),
                                ("Last Accessed", |md| format_system_time(md.accessed())),
                                ("Last Modified", |md| format_system_time(md.modified())),
                                ("Type", |md| format_type(md)),
                            ];
                
                            for (attr_name, attr_func) in attributes {
                                ui.label(format!("{}: {}", attr_name, attr_func(metadata)));
                            }
                        } else {
                            ui.label("Error fetching file details");
                        }
                    });
            }
        }
    }); 
    Ok(())
}
