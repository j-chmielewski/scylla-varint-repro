use num_bigint::BigInt;
use scylla::{IntoTypedRows, SessionBuilder};
use std::{str::FromStr, error::Error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    // Insert a varint into the table
    let to_insert: BigInt = BigInt::from_str("12345")?;
    let session = SessionBuilder::new().known_node("locahost:9042").build().await.expect("Connection failed");
    session
        .query("INSERT INTO keyspace.table (a) VALUES(?)", (to_insert,))
        .await?;

    // Read a varint from the table
    if let Some(rows) = session
        .query("SELECT a FROM keyspace.table", &[])
        .await?
        .rows
    {
        for row in rows.into_typed::<(BigInt,)>() {
            let (varint_value,): (BigInt,) = row?;
        }
    }

    Ok(())
}
