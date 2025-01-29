use crate::budget::budget::Budget;
use rusqlite::Connection;

pub struct DBInstance {
    url: String,
    active: bool,
    connection: Connection,
}

impl DBInstance {
    pub fn new(url: String) -> Self {
        let conn_url = url.clone();
        Self {
            url,
            active: false,
            connection: Connection::open(conn_url).unwrap(),
        }
    }

    pub async fn prestart(&mut self) {
        let conn = &self.connection;

        let attach = "ATTACH DATABASE IF NOT EXISTS 'test.db' AS test;";
        let _ = conn.execute(attach, ());

        let create_table_budget = "CREATE TABLE IF NOT EXISTS budgets(
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                budget_type INTEGER UNIQUE NOT NULL,
                sum FLOAT NOT NULL,
                duration REAL NOT NULL,
                ts_start REAL NOT NULL,
                description TEXT
            );";
        conn.execute(create_table_budget, ()).unwrap();
        // conn.restore(rusqlite::DatabaseName::Attached("test.db"), "test.db", Some(()));
    }

    pub async fn on_stop() {
        let conn = Connection::open("test.db").unwrap();
        // conn.backup(rusqlite::DatabaseName::Attached("test.db"), "test.db", ());
    }
}

pub(crate) async fn read_budgets() -> Vec<Budget> {
    let db_instance = crate::db::db::DBInstance::new("test.db".to_string());

    let sql = "
        SELECT id, budget_type, sum, duration, ts_start, description FROM budgets
    ";

    let mut stmt = db_instance.connection.prepare(sql).unwrap();
    let budget_iter = stmt
        .query_map([], |row| {
            Ok(crate::budget::budget::Budget {
                id: row.get(0).unwrap(),
                budget_type: row.get(1).unwrap(),
                sum: row.get(2).unwrap(),
                duration: row.get(3).unwrap(),
                ts_start: row.get(4).unwrap(),
                description: row.get(5).unwrap(),
            })
        })
        .unwrap();

    let mut budgets = std::vec::Vec::new();
    for budget in budget_iter {
        budgets.push(budget.unwrap())
    }
    budgets
}
