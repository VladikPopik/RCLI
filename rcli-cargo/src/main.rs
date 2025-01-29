use clap::Command;
use comfy_table::Table;
use rusqlite::Connection;
use std::io;

use budget::budget::budget_cycle;
use db::db::DBInstance;
pub mod budget;
pub mod db;

enum Options {
    Budget = 1,
    Reports = 2,
    Todo = -1,
}

async fn read_option() -> String {
    println!("Paste option in here\n=>");
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer).unwrap();
    println!("You said: {}", buffer);
    buffer.trim().to_string()
}

async fn option_table_reference() {
    println!("Please input numbers that represent this cli functions:");
    let mut table = Table::new();

    table
        .set_header(vec!["Budget", "Reports", "TODO"])
        .add_row(vec!["--budget", "--reports", "--todo"]);

    println!("  ==> \n{table}\n\n");
}

async fn try_option() -> isize {
    let command_option = read_option().await;

    match command_option.as_str() {
        "--budget" => Options::Budget as isize,
        "--reports" => Options::Reports as isize,
        _ => Options::Todo as isize,
    }
}

#[tokio::main]
async fn main() {
    // -h --version tags for CLI
    let _matches = Command::new("RCLI APP")
        .version("0.1")
        .about("My RCLI app to solve basic routines")
        .get_matches();
    // Initialize or connect to db
    let mut db_instance = DBInstance::new("test.db".to_string());
    db_instance.prestart().await;

    // Start Up of RCLI
    println!("@@@@@@@@@@@@@@ START UP @@@@@@@@@@@@@@\n");
    println!("Hello, User!\n");
    println!("There are some functions u can use with me:\n\n");
    option_table_reference().await;

    // Try to get reserved options and only them
    let mut option_code = try_option().await;
    while option_code < 0 {
        println!("~~~~~~~~~Ooooops! Try again!~~~~~~~~~\n\n");
        option_table_reference().await;
        option_code = try_option().await;
    }

    // Continue to next phases
    match option_code {
        1 => budget_cycle().await,
        2 => println!("REPORTS"),
        _ => println!("NOT IMPLEMENTED YET"),
    }

    // End of working cycle
    println!("@@@@@@@@@@@ CYCLE ENDED @@@@@@@@@@@@\n\n");
}
