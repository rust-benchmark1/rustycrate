use sqlx::PgPool;
use std::process::Command;

// SQL Injection vulnerability
pub async fn find_product(query: String, pool: &PgPool) -> Result<String, sqlx::Error> {
    let sql_query = format!("SELECT * FROM products WHERE name = '{}'", query);
    let result = sqlx::query(&sql_query).fetch_one(pool).await?;
    Ok(format!("Product found: {:?}", result))
}

// Second-Order SQL Injection vulnerability
pub async fn save_user_input(query: String, pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query(&format!("INSERT INTO user_inputs (input) VALUES ('{}')", query))
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn use_saved_input(pool: &PgPool) -> Result<(), sqlx::Error> {
    let row = sqlx::query("SELECT input FROM user_inputs ORDER BY id DESC LIMIT 1")
        .fetch_one(pool)
        .await?;
    let saved_input: String = row.get(0);
    let sql_query = format!("SELECT * FROM products WHERE description LIKE '%{}%'", saved_input);
    let result = sqlx::query(&sql_query).fetch_one(pool).await?;
    Ok(())
}

// Stored Command Injection vulnerability
pub async fn execute_stored_command(pool: &PgPool) -> Result<(), sqlx::Error> {
    let row = sqlx::query("SELECT input FROM user_inputs ORDER BY id DESC LIMIT 1")
        .fetch_one(pool)
        .await?;
    let saved_input: String = row.get(0);
    let output = Command::new("sh").arg("-c").arg(saved_input).output().expect("Command execution failed");
    println!("Output: {}", String::from_utf8_lossy(&output.stdout));
    Ok(())
}
