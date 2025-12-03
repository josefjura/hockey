use clap::Parser;
use dotenvy::dotenv;
use jura_hockey::{auth::hash_password, config::Config, errors::AppError};
use sqlx::SqlitePool;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), AppError> {
    // Load environment variables from .env file
    dotenv().ok();

    // Parse config from environment
    let config = Config::parse();

    // Connect to database
    let db = SqlitePool::connect(&config.database_url).await?;

    println!("=== Create Admin User ===\n");

    // Prompt for email
    print!("Email: ");
    io::stdout().flush().unwrap();
    let mut email = String::new();
    io::stdin()
        .read_line(&mut email)
        .expect("Failed to read email");
    let email = email.trim().to_string();

    // Validate email format (basic check)
    if !email.contains('@') || !email.contains('.') {
        eprintln!("Error: Invalid email format");
        std::process::exit(1);
    }

    // Check if user already exists
    let existing_user: Option<(i64,)> = sqlx::query_as("SELECT id FROM users WHERE email = ?")
        .bind(&email)
        .fetch_optional(&db)
        .await?;

    if existing_user.is_some() {
        eprintln!("Error: User with email '{}' already exists", email);
        std::process::exit(1);
    }

    // Prompt for password
    print!("Password: ");
    io::stdout().flush().unwrap();
    let password = rpassword::read_password().expect("Failed to read password");

    if password.len() < 8 {
        eprintln!("Error: Password must be at least 8 characters long");
        std::process::exit(1);
    }

    // Prompt for password confirmation
    print!("Confirm Password: ");
    io::stdout().flush().unwrap();
    let password_confirm = rpassword::read_password().expect("Failed to read password");

    if password != password_confirm {
        eprintln!("Error: Passwords do not match");
        std::process::exit(1);
    }

    // Prompt for name
    print!("Name (optional): ");
    io::stdout().flush().unwrap();
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read name");
    let name = name.trim().to_string();
    let name = if name.is_empty() { None } else { Some(name) };

    // Hash password
    println!("\nHashing password...");
    let password_hash = hash_password(&password)?;

    // Insert user into database
    println!("Creating admin user...");
    let result = sqlx::query("INSERT INTO users (email, name, password_hash) VALUES (?, ?, ?)")
        .bind(&email)
        .bind(&name)
        .bind(&password_hash)
        .execute(&db)
        .await?;

    let user_id = result.last_insert_rowid();

    println!("\n✓ Admin user created successfully!");
    println!("  ID: {}", user_id);
    println!("  Email: {}", email);
    if let Some(n) = name {
        println!("  Name: {}", n);
    }

    Ok(())
}
