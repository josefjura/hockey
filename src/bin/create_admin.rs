use anyhow::Result;
use bcrypt::{hash, DEFAULT_COST};
use sqlx::sqlite::SqlitePoolOptions;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<()> {
    println!("=== Hockey Management - Create Admin User ===\n");

    // Load .env file
    let _ = dotenvy::dotenv();

    // Get database URL
    let database_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:./hockey.db".to_string());

    println!("Connecting to database: {}", database_url);

    // Connect to database
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect(&database_url)
        .await?;

    println!("Connected successfully!\n");

    // Get user input
    print!("Enter admin email: ");
    io::stdout().flush()?;
    let mut email = String::new();
    io::stdin().read_line(&mut email)?;
    let email = email.trim();

    print!("Enter admin name: ");
    io::stdout().flush()?;
    let mut name = String::new();
    io::stdin().read_line(&mut name)?;
    let name = name.trim();

    print!("Enter admin password: ");
    io::stdout().flush()?;
    let mut password = String::new();
    io::stdin().read_line(&mut password)?;
    let password = password.trim();

    // Hash password
    println!("\nHashing password...");
    let password_hash = hash(password, DEFAULT_COST)?;

    // Insert user
    println!("Creating user...");
    sqlx::query(
        "INSERT INTO users (email, password_hash, name, created_at, updated_at) VALUES (?, ?, ?, datetime('now'), datetime('now'))"
    )
    .bind(email)
    .bind(&password_hash)
    .bind(name)
    .execute(&pool)
    .await?;

    println!("\n✅ Admin user created successfully!");
    println!("Email: {}", email);
    println!("Name: {}", name);
    println!("\nYou can now log in at http://localhost:8080/auth/login");

    Ok(())
}
