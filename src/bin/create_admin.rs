use anyhow::Result;
use bcrypt::{hash, DEFAULT_COST};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
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

    // Connect to database with foreign keys enabled
    let connection_options = database_url
        .parse::<SqliteConnectOptions>()?
        .foreign_keys(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect_with(connection_options)
        .await?;

    println!("Connected successfully!\n");

    // Check for command-line arguments (non-interactive mode)
    let args: Vec<String> = std::env::args().collect();
    let force_recreate = args.contains(&"--force".to_string());

    let (email, name, password) = if args.len() >= 4 {
        // Non-interactive mode: use command-line arguments
        // Usage: create_admin <email> <name> <password> [--force]
        println!("Using non-interactive mode with provided arguments\n");
        (args[1].as_str(), args[2].as_str(), args[3].as_str())
    } else {
        // Interactive mode: prompt for input
        print!("Enter admin email: ");
        io::stdout().flush()?;
        let mut email = String::new();
        io::stdin().read_line(&mut email)?;
        let email = email.trim().to_string();

        print!("Enter admin name: ");
        io::stdout().flush()?;
        let mut name = String::new();
        io::stdin().read_line(&mut name)?;
        let name = name.trim().to_string();

        print!("Enter admin password: ");
        io::stdout().flush()?;
        let mut password = String::new();
        io::stdin().read_line(&mut password)?;
        let password = password.trim().to_string();

        (
            Box::leak(email.into_boxed_str()) as &str,
            Box::leak(name.into_boxed_str()) as &str,
            Box::leak(password.into_boxed_str()) as &str,
        )
    };

    // Hash password
    println!("\nHashing password...");
    let password_hash = hash(password, DEFAULT_COST)?;

    // Delete existing user if --force flag is set
    if force_recreate {
        println!("Force mode: deleting existing user if present...");
        sqlx::query("DELETE FROM users WHERE email = ?")
            .bind(email)
            .execute(&pool)
            .await?;
    }

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
