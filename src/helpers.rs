use owo_colors::OwoColorize;

pub async fn handle_response(r: reqwest::Response, dbg: bool, o: String) -> anyhow::Result<()> {
    if dbg {
        println!("{:#?}", r);
        write_output_to_file(&r.text().await?, o)?;
        return Ok(())
    }
    
    println!("{}: {}", "Status".green().bold(), r.status().magenta());
    println!("{}:", "headers".green().bold());
    for (k, v) in r.headers() {
        match v.to_str() {
            Ok(s) => println!("{}: {}", k.blue(), s.cyan()),
            Err(_) => continue
        }
    }

    println!("\n{}", "Body:".green().bold());
    match r.text().await {
        Err(_) => anyhow::bail!("Error while reading body.".red()),
        Ok(b) => {
            if b == "null" || b == "" {
                println!("{}", "[ Empty Body ]".yellow());
            } else {
                println!("{b}");
                write_output_to_file(&b, o)?;
            }
        }
    };

    Ok(())
}

fn write_output_to_file(text: &String, path: String) -> anyhow::Result<()> {
    if path.is_empty() || text.is_empty() { return Ok(()) }
    if let Err(e) = std::fs::write(path, text) {
        anyhow::bail!("{} {}", "Error while writing to file:".red(), e.red())
    };

    Ok(())
}
