use clap::Command;
use comfy_table::Table;
use rusqlite::Connection;
use std::io;

enum Options {
    Budget = 1,
    Reports = 2,
    Todo = -1
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
        .add_row(vec![
            "--budget",
            "--reports",
            "--todo",
        ]);

    println!("  ==> \n{table}\n\n");
}

async fn try_option() -> isize{
    let command_option = read_option().await;

    match command_option.as_str() {
        "--budget" => Options::Budget as isize,
        "--reports" => Options::Reports as isize,
        _ => Options::Todo as isize
    }
}

async fn prestart(){
    let conn = Connection::open(
        "test.db"
    ).unwrap();

    let attach = "ATTACH DATABASE IF NOT EXISTS 'test.db' AS test;";
    let _ = conn.execute(attach, ());


    let create_table_budget = "CREATE TABLE IF NOT EXISTS budgets(
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            sum FLOAT NOT NULL,
            duration REAL NOT NULL,
            ts_start REAL NOT NULL
        );";
    conn.execute(
        create_table_budget,
        (),
    ).unwrap();
    // conn.restore(rusqlite::DatabaseName::Attached("test.db"), "test.db", Some(()));

}

async fn on_stop() {
    let conn = Connection::open_in_memory().unwrap();

    // conn.backup(name, dst_path, progress)
}


#[tokio::main]
async fn main() {
    let _matches = Command::new("RCLI APP")
        .version("0.1")
        .about("My RCLI app to solve basic routines")
        .get_matches();

    prestart().await;

    println!("@@@@@@@@@@@@@@ START UP @@@@@@@@@@@@@@\n");
    println!("Hello, User!\n");
    println!("There are some functions u can use with me:\n\n");
    option_table_reference().await;

    let mut option_code = try_option().await;

    while option_code < 0 {
        println!("~~~~~~~~~Ooooops! Try again!~~~~~~~~~\n\n");
        option_table_reference().await;
        option_code = try_option().await;
    }

    match option_code {
        1 => println!("BUDGET"),
        2 => println!("REPORTS"),
        _ => println!("NOT IMPLEMENTED YET")
    }

    println!("@@@@@@@@@@@ CYCLE ENDED @@@@@@@@@@@@\n\n")
}