use std::fmt::format;
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::fs;
use std::path::{ Path, PathBuf };
use std::sync::Mutex;
use serde::Serialize;

#[derive(Serialize, Clone)]
struct Transaction {
    date: String,
    description: String,
    amount: i64,
    account_name: String,
    category: String,
    transaction_type: String,
    sub_category: String,
    hidden: bool,
}

static TRANSACTIONS: Mutex<Vec<Transaction>> = Mutex::new(Vec::new());

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn list_dir(path: String) -> Result<Vec<String>, String> {
    let dir: &Path = Path::new(&path);
    let rd = fs::read_dir(&dir).map_err(|e| format!("read_dir error: {}", e))?;
    let mut entries = Vec::new();
    for entry in rd {
        let entry = entry.map_err(|e| format!("dir entry error: {}", e))?;
        let name = entry.file_name().to_string_lossy().into_owned();
        entries.push(name.trim().to_string());
    }
    Ok(entries)
}

#[tauri::command]
fn parse_csv(path: String) -> Result<Vec<Vec<String>>, String> {
    let mut rdr = csv::Reader::from_path(path).map_err(|e| format!("csv open error: {}", e))?;
    let mut rows: Vec<Vec<String>> = Vec::new();
    for result in rdr.records() {
        let record = result.map_err(|e| format!("csv record error: {}", e))?;
        let row = record
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        rows.push(row);
    }
    Ok(rows)
}

#[tauri::command]
fn parse_transactions(path: String) -> Result<String, String> {
    let mut rdr = csv::Reader::from_path(path).map_err(|e| format!("csv open error: {}", e))?;
    let mut transactions: Vec<Transaction> = Vec::new();
    let mut count: i32 = 0;

    for result in rdr.records() {
        let record = result.map_err(|e| format!("csv record error: {}", e))?;
        let row = record
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        if count >= 1 {
            // Skip header row
            transactions.push(build_transaction(row));
        }
        count += 1;
    }

    // Store in global variable
    let mut global_transactions = TRANSACTIONS.lock().unwrap();
    *global_transactions = transactions;
    let transaction_count = global_transactions.len();
    drop(global_transactions); // Release lock early

    Ok(format!("Loaded {} transactions into memory", transaction_count))
}
#[tauri::command]
fn get_transactions() -> Vec<Transaction> {
    let transactions = TRANSACTIONS.lock().unwrap();
    transactions.clone()
}

#[tauri::command]
fn get_transaction_count() -> usize {
    let transactions = TRANSACTIONS.lock().unwrap();
    return transactions.len();
}

#[tauri::command]
fn get_income() -> f64 {
    let transactions = TRANSACTIONS.lock().unwrap();

    let mut total_income_cents = 0;
    for transaction in transactions.iter() {
        if transaction.transaction_type == "Income" {
            total_income_cents += transaction.amount;
        }
    }

    let total_income_dollars = (total_income_cents as f64) / 100.0;

    return total_income_dollars;
}

#[tauri::command]
fn get_expenses() -> f64 {
    let transactions = TRANSACTIONS.lock().unwrap();

    let mut total_expense_cents = 0;
    for transaction in transactions.iter() {
        println!("{}", transaction.transaction_type);

        if transaction.transaction_type == "Expenses" {
            total_expense_cents += transaction.amount;
        }
    }
    println!("{}", total_expense_cents);
    let total_expense_dollars = (total_expense_cents as f64) / 100.0;

    return total_expense_dollars;
}

fn build_transaction(row: Vec<String>) -> Transaction {
    // Debug output to see what's in each column

    let amount_str = row
        .get(2)
        .map(|s| s.as_str())
        .unwrap_or("0");

    // Parse as float first, then convert to cents (multiply by 100)
    let amount = amount_str.parse::<f64>().unwrap_or_else(|e| {
        println!("Failed to parse amount '{}': {}", amount_str, e);
        0.0
    });

    // Convert to cents (i64) to avoid floating point precision issues
    let amount_cents = (amount * 100.0).round() as i64;

    Transaction {
        date: row.get(0).unwrap_or(&"".to_string()).clone(),
        description: row.get(1).unwrap_or(&"".to_string()).clone(),
        amount: amount_cents,
        account_name: row.get(3).unwrap_or(&"".to_string()).clone(),
        transaction_type: row.get(4).unwrap_or(&"".to_string()).clone(),
        category: row.get(5).unwrap_or(&"".to_string()).clone(),
        sub_category: row.get(6).unwrap_or(&"".to_string()).clone(),
        hidden: row.get(7).unwrap_or(&"false".to_string()).parse::<bool>().unwrap_or(false),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder
        ::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(
            tauri::generate_handler![
                greet,
                list_dir,
                parse_csv,
                parse_transactions,
                get_transaction_count,
                get_income,
                get_expenses,
                get_transactions
            ]
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
