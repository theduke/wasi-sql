use wasi_sql::ValueParam;

fn main() -> Result<(), ()> {
    let driver = wasi_sql::load_driver("sqlite").unwrap();
    let con = driver.connect_uri("/tmp/sqlite1.sqlite3").unwrap();

    eprintln!("executing");
    con.execute(
        "CREATE TABLE IF NOT EXISTS test (id INTEGER PRIMARY KEY, name TEXT)",
        &[],
    )
    .unwrap();

    con.execute(
        "INSERT INTO test (name) VALUES (?)",
        &[ValueParam::Str("hello")],
    )
    .unwrap();

    eprintln!("querying items");
    let items = con
        .query("SELECT * from test", &[])
        .unwrap()
        .values()
        .unwrap();
    eprintln!("got rows");
    assert!(items.len() > 0);
    eprintln!("queried!");
    dbg!(&items);

    eprintln!("done...");

    Ok(())
}
