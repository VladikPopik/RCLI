use std::io;

enum Options {
    Insert = 1,
    Delete = 2,
    Calculate = 3,
    Show = 4,
    Return = 5,
    Todo = -1,
}

#[derive(Debug)]
pub(crate) struct Budget {
    pub(crate) id: usize,
    pub(crate) budget_type: usize,
    pub(crate) sum: f64,
    pub(crate) duration: f64,
    pub(crate) ts_start: f64,
    pub(crate) description: Option<String>,
}

async fn read_option() -> String {
    println!("Paste option in here\n=>");
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer).unwrap();
    buffer.trim().to_string()
}

async fn try_option() -> isize {
    println!("Options are:\n    1: Insert new budget\n    2: Delete budget\n    3: Calculate period and dt\n    4: Show budgets\n    5: RETURN");

    let command_option = read_option().await;

    match command_option.as_str() {
        "1" => Options::Insert as isize,
        "2" => Options::Delete as isize,
        "3" => Options::Calculate as isize,
        "4" => Options::Show as isize,
        "5" => Options::Return as isize,
        _ => Options::Todo as isize,
    }
}

pub async fn budget_cycle() {
    println!("@@@@@@@@@@@@@@ BUDGET MANAGEMENT @@@@@@@@@@@@@@");
    crate::budget::show::show_budgets().await;

    let mut option_code = try_option().await;
    while option_code < 0 {
        println!("~~~~~~~~~Ooooops! Try again!~~~~~~~~~\n\n");
        option_code = try_option().await;
    }

    // Continue to next phases
    match option_code {
        1 => println!("INSERT"),
        2 => println!("DELETE"),
        3 => println!("CALCULATE"),
        4 => crate::budget::show::show_budgets().await,
        _ => println!("NOT IMPLEMENTED YET"),
    }
}
