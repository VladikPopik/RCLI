use crate::budget::budget::Budget;
use crate::db::db::read_budgets;
use comfy_table::Table;

async fn budgets_table_show(budgets: std::vec::Vec<Budget>) {
    let mut table = Table::new();

    table.set_header(vec![
        "id",
        "budget_type",
        "sum",
        "duration",
        "ts_start",
        "description",
    ]);

    for budget in budgets {
        table.add_row([
            budget.id.to_string(),
            budget.budget_type.to_string(),
            budget.sum.to_string(),
            budget.duration.to_string(),
            budget.ts_start.to_string(),
            budget.description.unwrap(),
        ]);
    }

    println!("  ==> \n{table}\n\n");
}

pub async fn show_budgets() {
    let budgets = read_budgets().await;
    budgets_table_show(budgets).await;
}
