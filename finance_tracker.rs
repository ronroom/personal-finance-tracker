use std::io;

struct Expense {
    name: String,
    amount: f64,
    category: String,
}

fn add_expense(name: &str, amount: f64, category: &str) -> Expense {
    Expense {
        name: name.to_string(),
        amount,
        category: category.to_string(),
    }
}

fn view_all(expense: &Vec<Expense>) {
    for exp in expense {
        println!("{}: Rp{} [{}]", exp.name, format_currency(exp.amount), exp.category);
    }
}

fn view_by_category(expenses: &Vec<Expense>, category: &str) {
    for exp in expenses {
        if exp.category == category {
            println!("{}: Rp{} [{}]", exp.name, format_currency(exp.amount), exp.category)
            ;
        }
    }
}

fn total_by_category(expenses: &Vec<Expense>, category: &str) -> f64 {
    let mut total = 0.0;
    for exp in expenses {
        if exp.category == category {
            total += exp.amount;
        }
    }
    total
}

fn get_input(prompt: &str) -> String {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn format_currency(amount: f64) -> String {
    let amount_int = amount as i64;
    let amount_str = amount_int.to_string();
    let mut result = String::new();

        for (i, c) in amount_str.chars().rev().enumerate() {
        if i != 0 && i % 3 == 0 {
            result.push('.'); 
        }
        result.push(c);
    }
    result.chars().rev().collect()
}

fn main() {
    let mut expenses: Vec<Expense> = Vec::new();

    loop {
        println!("\nFinance Tracker Menu:");
        println!("1. Add Expense");
        println!("2. View All Expenses");
        println!("3. View Expenses by Category");
        println!("4. Total Expenses by Category");
        println!("5. Exit");

        let choice = get_input("Enter your choice:");

        match choice.as_str() {
            "1" => {
                let name = get_input("Enter expense name:");
                let amount_str = get_input("Enter expense amount:");
                let amount: f64 = amount_str.replace(",", "").parse().unwrap_or(0.0);
                let category = get_input("Enter expense category:");
                let expense = add_expense(&name, amount, &category);
                expenses.push(expense);
            }
            "2" => {
                view_all(&expenses);
            }
            "3" => {
                let category = get_input("Enter category to view:");
                view_by_category(&expenses, &category);
            }
            "4" => {
                let category = get_input("Enter category to total:");
                let total = total_by_category(&expenses, &category);
                println!("Total for {}: Rp{}", category, format_currency(total));
            }
            "5" => break,
            _ => println!("Invalid choice, please try again."),
        }
    }
}