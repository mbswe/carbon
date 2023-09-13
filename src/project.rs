use std::{env, fs};
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use chrono::{Local, DateTime, Duration};
use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Project {
    id: u32,
    title: String,
    sessions: Vec<Session>,
    completed: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct Session {
    start_time: DateTime<Local>,
    end_time: Option<DateTime<Local>>,
}

pub(crate) fn start(title: String) {
    let file_path = get_file_path();

    let mut projects = vec![];

    if let Ok(file) = File::open(file_path.clone()) {
        let mut data = String::new();
        let mut reader = io::BufReader::new(file);
        reader.read_to_string(&mut data).unwrap_or_default();

        if let Ok(existing_projects) = serde_json::from_str::<Vec<Project>>(&data) {
            projects = existing_projects;
        }
    }

    let new_id = projects.iter().map(|p| p.id).max().unwrap_or(0) + 1;

    let project = Project {
        id: new_id,
        title: title.clone(),
        sessions: vec![Session {
            start_time: Local::now(),
            end_time: None,
        }],
        completed: false,
    };

    projects.push(project);

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(file_path)
        .expect("Failed to open or create file");
    let data = serde_json::to_string(&projects).expect("Failed to serialize data");
    write!(file, "{}", data).expect("Failed to write data to file");
    
    println!("Started tracking time for '{}' with ID {}", title, new_id);
}

pub(crate) fn stop(id: u32) {
    let file_path = &get_file_path();
    let mut projects: Vec<Project> = read_projects_from_file(file_path);

    if let Some(project) = projects.iter_mut().find(|p| p.id == id) {
        if project.completed {
            println!("The project with ID: {} is already completed.", id);
            return;
        }
        
        if let Some(last_session) = project.sessions.last_mut() {
            if last_session.end_time.is_none() {
                last_session.end_time = Some(Local::now());
            }
        }
        
        project.completed = true;
        write_projects_to_file(file_path, &projects);
        
        println!("Stopped project with ID: {}. The project is now complete.", id);
    } else {
        println!("No project found with ID: {}", id);
    }
}

pub(crate) fn pause(id: u32) {
    let file_path = &get_file_path();
    let mut projects: Vec<Project> = read_projects_from_file(file_path);

    if let Some(project) = projects.iter_mut().find(|p| p.id == id) {
        if let Some(last_session) = project.sessions.last_mut() {
            if last_session.end_time.is_none() {
                last_session.end_time = Some(Local::now());
                write_projects_to_file(file_path, &projects);
                println!("Paused project with ID: {}", id);
            } else {
                println!("The last session of this project is already paused.");
            }
        } else {
            println!("No active session found for project with ID: {}", id);
        }
    } else {
        println!("No project found with ID: {}", id);
    }
}

pub(crate) fn resume(id: u32) {
    let file_path = &get_file_path();
    let mut projects: Vec<Project> = read_projects_from_file(file_path);

    if let Some(project) = projects.iter_mut().find(|p| p.id == id) {
        if let Some(last_session) = project.sessions.last() {
            if project.completed {
                println!("The project with ID: {} is completed and cannot be resumed.", id);
                return;
            }
            
            if last_session.end_time.is_some() {
                project.sessions.push(Session {
                    start_time: Local::now(),
                    end_time: None,
                });
                write_projects_to_file(file_path, &projects);
                println!("Resumed project with ID: {}", id);
            } else {
                println!("The last session of this project is already running.");
            }
        } else {
            println!("No sessions found for project with ID: {}", id);
        }
    } else {
        println!("No project found with ID: {}", id);
    }
}

pub(crate) fn status() {
    let file_path = &get_file_path();
    let projects: Vec<Project> = read_projects_from_file(file_path);

    for project in projects.iter().filter(|p| !p.completed) {
        println!("Project ID: {}, Title: {}", project.id, project.title);

        let mut total_duration = Duration::zero();
        
        for (i, session) in project.sessions.iter().enumerate() {
            let end_time = session.end_time.unwrap_or(Local::now());
            let duration = end_time - session.start_time;
            total_duration = total_duration + duration;

            println!("  Session {}: started at {}, ended at {}. Duration: {}",
                     i + 1, 
                     session.start_time.format("%Y-%m-%d %H:%M:%S").to_string(), 
                     end_time.format("%Y-%m-%d %H:%M:%S").to_string(), 
                     format_duration(duration));
        }

        println!("  Total time spent on project: {}", format_duration(total_duration));
    }
}

pub(crate) fn list_all() {
    let file_path = &get_file_path();
    let projects: Vec<Project> = read_projects_from_file(file_path);

    for project in projects.iter().filter(|p| p.completed) {
        display_project_summary(project);
        // New row for spacing unless it's the last project
        if project.id != projects.last().unwrap().id {
            println!();
        }
    }
}

pub(crate) fn list_today() {
    let today = Local::now();
    list_projects_on_date(today);
}

pub(crate) fn list_yesterday() {
    let yesterday = Local::now() - Duration::days(1);
    list_projects_on_date(yesterday);
}

fn list_projects_on_date(date: DateTime<Local>) {
    let file_path = &get_file_path();
    let projects: Vec<Project> = read_projects_from_file(file_path);

    let target_date = date.date_naive();

    for project in projects.iter().filter(|p| p.completed && p.sessions.iter().any(|s| s.end_time.map_or(false, |end_time| end_time.date_naive() == target_date))) {
        display_project_summary(project);

        // New row for spacing unless it's the last project
        if project.id != projects.last().unwrap().id {
            println!();
        }
    }
}

fn display_project_summary(project: &Project) {
    let total_duration = project.sessions.iter()
        .filter_map(|session| session.end_time)
        .zip(project.sessions.iter().map(|session| session.start_time))
        .map(|(end_time, start_time)| end_time - start_time)
        .fold(Duration::zero(), |acc, duration| acc + duration);

    println!("Project ID: {}, Title: {}", project.id, project.title);
    println!("  Total time spent on project: {}", format_duration(total_duration));
}

fn read_projects_from_file(file_path: &str) -> Vec<Project> {
    let mut projects = vec![];
    if let Ok(file) = File::open(file_path) {
        let mut data = String::new();
        let mut reader = io::BufReader::new(file);
        reader.read_to_string(&mut data).unwrap_or_default();

        if let Ok(existing_projects) = serde_json::from_str::<Vec<Project>>(&data) {
            projects = existing_projects;
        }
    }
    projects
}

fn write_projects_to_file(file_path: &str, projects: &Vec<Project>) {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_path)
        .expect("Failed to open or create file");
    let data = serde_json::to_string(&projects).expect("Failed to serialize data");
    write!(file, "{}", data).expect("Failed to write data to file");
}

fn format_duration(duration: Duration) -> String {
    let total_seconds = duration.num_seconds();
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;
    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}

fn get_file_path() -> String {
    let config_home = env::var("XDG_CONFIG_HOME").or_else(|_| env::var("HOME").map(|home|format!("{}/.config", home))).unwrap();
    
    // Create carbon directory if not exist
    let carbon_dir = format!("{}/carbon", config_home);
    fs::create_dir_all(carbon_dir).expect("Failed to create carbon directory");
    
    let file_path = "projects.json";
    format!("{}/carbon/{}", config_home, file_path)
}
