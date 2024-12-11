use actix_web::{web, App, HttpServer, HttpResponse};
use actix_cors::Cors;
use serde::{Deserialize, Serialize};
use std::process::Command;
use std::fs;
use std::io::Write;

#[derive(Deserialize)]
struct InputArray {
    data: Vec<i32>
}

#[derive(Serialize)]
struct OutputArray {
    result: Vec<i32>
}

async fn update_and_run(array: web::Json<InputArray>) -> HttpResponse {
    // Read cairo file
    let content = fs::read_to_string("../src/lib.cairo")
        .expect("Unable to read file");
    
    // Update array
    let array_str = format!("let arr = array![{}];", 
        array.data.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", "));
    
    // Replace line 5
    let lines: Vec<&str> = content.lines().collect();
    let mut new_lines = lines.clone();
    new_lines[4] = &array_str;
    
    // Write back to file
    let mut file = fs::File::create("../src/lib.cairo")
        .expect("Unable to create file");
    file.write_all(new_lines.join("\n").as_bytes())
        .expect("Unable to write file");

    // Execute scarb cairo-run
    let output = Command::new("scarb")
        .args(["cairo-run"])
        .current_dir("..")
        .output()
        .expect("Failed to execute command");

    // Get stdout and stderr
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    // If command executed successfully and found result
    if let Some(result) = parse_output(&stdout) {
        // Print success information
        println!("Success: \nstdout: {}\nResult: {:?}", stdout, result);
        HttpResponse::Ok().json(OutputArray { result })
    } else {
        // Print and return error message
        let error_message = format!("Error: \nstdout: {}\nstderr: {}", stdout, stderr);
        println!("{}", error_message); // Print to console
        HttpResponse::InternalServerError().body(error_message)
    }
}

fn parse_output(output: &str) -> Option<Vec<i32>> {
    output.lines()
        .find(|line| line.contains("Run completed successfully"))
        .and_then(|line| {
            let start = line.find('[')?;
            let end = line.find(']')?;
            let numbers = &line[start+1..end];
            let result: Vec<i32> = numbers.split(',')
                .map(|s| s.trim().parse::<i32>().unwrap_or(0))
                .collect();
            Some(result)
        })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .route("/run", web::post().to(update_and_run))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
