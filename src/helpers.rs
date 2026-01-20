use owo_colors::OwoColorize;

use crate::flags::Flags;

pub async fn handle_response(r: reqwest::Response, f: &Flags) -> anyhow::Result<()> {
    if f.debug {
        println!("{:#?}", r);
        return Ok(());
    }

    println!("{}: {}", "Status".green().bold(), r.status().magenta());
    println!("{}:", "Headers".green().bold());
    for (k, v) in r.headers() {
        match v.to_str() {
            Ok(s) => println!("{}: {}", k.blue(), s.cyan()),
            Err(_) => continue,
        }
    }

    println!("\n{}", "Body:".green().bold());
    if let Some(o) = &f.output {
        match r.bytes().await {
            Err(e) => anyhow::bail!("{} {}", "Error while reading body:".red(), e.red()),
            Ok(b) => {
                write_output_to_file(&b, o)?;
                println!("{} '{}'", "Wrote body to:".cyan(), o.purple())
            }
        }

        return Ok(());
    }

    match r.text().await {
        Err(_) => anyhow::bail!("Error while reading body.".red()),
        Ok(b) => {
            if b == "null" || b == "" {
                println!("{}", "[ Empty Body ]".yellow());
            } else {
                println!("{b}")
            }
        }
    };

    Ok(())
}

fn write_output_to_file(text: &[u8], path: &String) -> anyhow::Result<()> {
    if let Err(e) = std::fs::write(path, text) {
        anyhow::bail!("{} {}", "Error while writing to file:".red(), e.red())
    };

    Ok(())
}
