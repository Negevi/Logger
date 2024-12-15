mod structs;
fn main() {
    println!("Hello World!");
    let date = chrono::Local::now().date_naive();
    println!("Hoje é dia: {date}")
}
