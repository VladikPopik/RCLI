use comfy_table::Table;
use rusqlite::Connection;
use std::io;

enum Options {
    INSERT = 1,
    DELETE = 2,
    CALCULATE = 3,
    SHOW = 4,
    RETURN = 5,
    Todo = -1
}


#[derive(Debug)]
struct Budget {
    id: usize,
    budget_type: usize,
    sum: f64,
    duration: f64,
    ts_start: f64,
    description: Option<String>
}

async fn budgets_table_show(budgets: std::vec::Vec<Budget>) {
    let mut table = Table::new();

    table
        .set_header(vec![
            "id",
            "budget_type",
            "sum", "duration",
            "ts_start",
            "description"
        ]);

    for budget in budgets {
        table.add_row([
            budget.id.to_string(),
            budget.budget_type.to_string(), 
            budget.sum.to_string(),
            budget.duration.to_string(), 
            budget.ts_start.to_string(), 
            budget.description.unwrap()
        ]);
    }

    println!("  ==> \n{table}\n\n");
}

async fn db_connect() -> Connection {
    

    Connection::open(
        "test.db"
    ).unwrap()
}

pub async fn show_budgets() {
    let conn = db_connect().await;

    let sql = "
        SELECT id, budget_type, sum, duration, ts_start, description FROM budgets
    ";

    let mut stmt = conn.prepare(sql).unwrap();

    let budget_iter = stmt.query_map([], |row| {
        Ok(Budget {
            id: row.get(0).unwrap(),
            budget_type: row.get(1).unwrap(),
            sum: row.get(2).unwrap(),
            duration: row.get(3).unwrap(),
            ts_start: row.get(4).unwrap(),
            description: row.get(5).unwrap()
        })
    }).unwrap();

    let mut budgets = std::vec::Vec::new();
    for budget in budget_iter {
        budgets.push(budget.unwrap())
    }
    budgets_table_show(budgets).await;
}

async fn read_option() -> String {
    println!("Paste option in here\n=>");
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer).unwrap();
    buffer.trim().to_string()
}

async fn try_option() -> isize{
    println!("    Options are:\n    1: Insert new budget\n    2: Delete budget\n    3: Calculate period and dt\n    4: Show budgets\n    5: RETURN");

    let command_option = read_option().await;

    match command_option.as_str() {
        "1" => Options::INSERT as isize,
        "2" => Options::DELETE as isize,
        "3" => Options::CALCULATE as isize,
        "4" => Options::SHOW as isize,
        "5" => Options::RETURN as isize,
        _ => Options::Todo as isize
    }
}

pub async fn budget_cycle() {
    println!("@@@@@@@@@@@@@@ BUDGET MANAGEMENT @@@@@@@@@@@@@@");
    show_budgets().await;

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
        4 => show_budgets().await,
        _ => println!("NOT IMPLEMENTED YET")
    }

}