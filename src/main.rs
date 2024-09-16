use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;

use clap::Parser;

#[derive(clap::Parser, Debug)]
#[command(long_about = None)]
struct Args {
    /// Path to a file with URLs
    #[arg(short, long)]
    path: String,

    #[arg(short, long)]
    github_token: String,

    /// Result filepath
    #[arg(short, long)]
    result: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let octocrab = octocrab::Octocrab::builder()
        .personal_token(args.github_token)
        .build()?;

    let mut min_size: u32 = 1080;
    let size_step: u32 = 1000;

    // Loop over all file sizes
    loop {
        let max_size = min_size + size_step;

        let search_query = format!(
            r#""lto = \"off\"" filename:Cargo.toml size:{}..{}"#,
            min_size, max_size
        );
        println!("Search query: {}", search_query);

        // Request repeater in the throttling case
        let mut page;
        loop {
            match octocrab
                .search()
                .code(search_query.as_str())
                .per_page(100)
                .send()
                .await
            {
                Ok(new_page) => {
                    page = new_page;
                    break;
                }
                Err(..) => {
                    println!("Rate limit triggered for initial request triggered");
                    tokio::time::sleep(tokio::time::Duration::from_secs(7)).await;
                    continue;
                }
            }
        }

        println!(
            "Total results for the range [{}; {}]: {}",
            min_size,
            max_size,
            page.total_count.unwrap()
        );

        let filename = format!("{}_{}-{}.txt", args.result, min_size, max_size);
        let mut file = File::create(filename).expect("Cannot create a file");

        // Loop over all pages for specific size
        let mut is_size_finished = false;
        loop {
            for code in &page {
                let _ = file.write_fmt(format_args!(
                    "{}\n",
                    code.repository
                        .html_url
                        .clone()
                        .expect("Cannot find HTML URL for a repository"),
                ));
            }

            // Request repeater in the throttling case
            loop {
                if page.next.is_none() {
                    println!(
                        "Page for the range [{}; {}] is finished early",
                        min_size, max_size
                    );
                    is_size_finished = true;
                    break;
                }

                match octocrab.get_page(&page.next).await {
                    Ok(next_page) => match next_page {
                        Some(next_page) => {
                            page = next_page;
                            break;
                        }
                        None => {
                            println!(
                                "Page for the range [{}; {}] is finished lately",
                                min_size, max_size
                            );
                            is_size_finished = true;
                            break;
                        }
                    },
                    Err(err) => {
                        println!("Rate limit triggered during next paging");
                        tokio::time::sleep(tokio::time::Duration::from_secs(7)).await;
                        continue;
                    }
                }
            }

            // No pages left in this size - stop and move to the next size
            if is_size_finished {
                file.flush().expect("Cannot flush a file");
                break;
            }
        }

        min_size += size_step;

        if min_size > 1000000 {
            println!("Min size limit achieved");
            break;
        }
    }

    Ok(())
}
