use postgres::{Client, NoTls, Error};
use dotenv::dotenv;

fn main () -> Result<(), Error> {
  dotenv().unwrap();

  let dbname = std::env::var("DB_NAME").expect("DB_NAME must be set.");
  let username = std::env::var("DB_USER").expect("DB_USER must be set.");
  let password = std::env::var("DB_PASSWORD").expect("DB_PASSWORD must be set.");

  let client_params = format!(
    "
    host=phys_db
    port=5432
    dbname={dbname}
    user={username}
    password={password}
    connect_timeout=10
    "
  );

  let mut client = Client::connect(&client_params,NoTls)?;

  println!("Connected to DB");

  client.batch_execute("
    CREATE TABLE IF NOT EXISTS users (
      id        SERIAL PRIMARY KEY,
      username  VARCHAR NOT NULL,
      password  VARCHAR NOT NULL,
      email     VARCHAR NOT NULL
    )
  ")?;

  client.batch_execute("
    CREATE TABLE IF NOT EXISTS repos (
      id        SERIAL PRIMARY KEY,
      name      VARCHAR NOT NULL,
      user_id   INTEGER NOT NULL REFERENCES users
    )
  ")?;

  Ok(())
}