use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::Path;

pub struct Tasks {
    tasks: Vec<String>,
    file: File,
}

impl Tasks {
    pub fn new(file_path: &str) -> Tasks {
        // Check if the file exists, if not create it
        let file_exists = Path::new("db.txt").exists();

        let mut file: File;

        if !file_exists {
            file = File::create("db.txt").expect("Error creating the file.");
        } else {
            // If file already exists, open it in append mode
            file = OpenOptions::new()
                .append(true)
                .open("db.txt")
                .expect("Error opening the file.");
        }

        let file_data = fs::read_to_string(file_path).expect("Error reading from file.");
        let tasks: Vec<String> = file_data
            .trim()
            .split("\n")
            .map(|s| s.to_string())
            .collect();

        return Tasks { tasks, file };
    }

    pub fn add_task(&mut self, task: &str) {
        self.tasks.push(task.to_owned());

        // Write to the file
        self.file
            .write_all(task.as_bytes())
            .expect("Error writing to file.");
    }

    pub fn list_tasks(&self) {
        // Print neatly
        for (index, task) in self.tasks.iter().enumerate() {
            println!("{}. {}", index + 1, task);
        }
    }

    pub fn delete_task(&mut self, task_id: usize) {
        self.tasks.remove(task_id);

        // Flush the entire file & update it
        self.file.flush().expect("Error flushing the file.");

        for task in self.tasks.iter() {
            self.file
                .write_all(task.as_bytes())
                .expect("Error writing to file.");
        }
    }
}
