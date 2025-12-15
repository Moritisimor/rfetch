pub async fn print_response(r: reqwest::Response) -> Result<(), Box<dyn std::error::Error>> {
    println!("Status: {}", r.status());
    println!("Headers:");
    for (k, v) in r.headers() {
        match v.to_str() {
            Ok(s) => println!("{k}: {s}"),
            Err(_) => continue
        }
    }

    println!("\nBody:");
    match r.text().await {
        Err(_) => println!("Unreadable body."),
        Ok(b) => {
            if b == "null" || b == "" {
                println!("<Empty Body>");
            } else {
                println!("{b}");
            }
        }
    }

    Ok(())
}