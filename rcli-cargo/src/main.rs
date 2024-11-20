use clap::Parser;
use std::io;
/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}


struct Task{
    name: String,
    description: String,
    number: String,
    task_type: u8,
}

struct Report{
    tasks: Vec<Task>,
}

impl Task{
    fn new(name: String, description: String, number: String, task_type: u8) -> Task{
        Task{
            name,
            description,
            number,
            task_type,
        }
    }
    fn correct(&self, name: String, description: String, number: String, task_type: u8) -> bool{
        if self.name == name && self.description == description && self.number == number && self.task_type == task_type{
            false
        }else{
            &self.name = name;
            &self.description = description;
            &self.number = number;
            &self.task_type = task_type;
            true
        }
    }
}

impl Report{
    fn new() -> Report{
        Report{
            tasks: Vec::new(),
        }
    }
    fn add_task(&mut self, task: Task){
        self.tasks.push(task);
    }
    fn print(&self){
        &self.tasks.sort_by(|a, b| a.number.parse().unwrap().cmp(&b.task_type.parse().unwrap()));

        for task in &self.tasks{
            println!("{} {} {} {}", task.name, task.description, task.number, task.task_type);
        }
    }
}

fn main() {
    println!("@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@\n\n");
    println!("Hello, User!\n\n");
    println!("There are some functions u can use with me:\n");

    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name);
    }

    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer).unwrap();
    println!("You said: {}", buffer);

}