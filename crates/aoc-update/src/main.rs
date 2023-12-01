use std::{path::Path, sync::Arc};

use config_better::Config;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use tracing::*;

use crate::{env::Environment, output::question_comment};

mod env;
mod env_logger;
mod output;

async fn get_description(
    client: &reqwest::Client,
    year: u32,
    day: u8,
) -> anyhow::Result<(String, TokenStream)> {
    let question_url = format!("https://adventofcode.com/{}/day/{}", year, day);
    let question_page = client.get(&question_url).send().await?.text().await?;
    let fragment = scraper::Html::parse_fragment(&question_page);
    let title_selector =
        scraper::Selector::parse("article.day-desc:nth-child(1) > h2:nth-child(1)")
            .map_err(|_| anyhow::anyhow!("Bad selector"))?;
    let title = fragment
        .select(&title_selector)
        .next()
        .map(|q| q.inner_html().replace("---", "").trim().to_owned())
        .ok_or(anyhow::anyhow!("No title found"))?;

    let selector = scraper::Selector::parse("article.day-desc")
        .map_err(|_| anyhow::anyhow!("Bad selector"))?;
    let mut doc_select = fragment.select(&selector);
    let question_md = doc_select
        .next()
        .map(|q| html2md::parse_html(&q.inner_html()))
        .ok_or(anyhow::anyhow!("No article found"))?;
    let maybe_second_part = doc_select
        .next()
        .map(|q| html2md::parse_html(&q.inner_html()))
        .unwrap_or_else(|| "".to_owned());
    let combined = question_md + "\n\n" + &maybe_second_part;

    Ok((title, question_comment(&combined)))
}

async fn get_input(client: &reqwest::Client, year: u32, day: u8) -> anyhow::Result<String> {
    let input_url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let input = client.get(&input_url).send().await?.text().await?;

    Ok(input)
}

// For any continous string of letters, make the first letter uppercase.
fn title_case(s: &str) -> String {
    let mut result = String::new();
    let mut last_was_letter = false;
    for c in s.chars() {
        if c.is_alphabetic() {
            if !last_was_letter {
                result.push(c.to_ascii_uppercase());
            } else {
                result.push(c);
            }
            last_was_letter = true;
        } else {
            result.push(c);
            last_was_letter = false;
        }
    }
    result
}

async fn update_main(year: u32) -> anyhow::Result<TokenStream> {
    let base_path = format!("crates/aoc{}/src", year);
    let output_path = Path::new(&base_path);

    // Get the list of directories in src that start with day
    let mut problems = tokio::fs::read_dir(output_path).await?;
    let mut considered: Vec<Ident> = vec![];
    let mut considered_upper: Vec<Ident> = vec![];
    while let Some(path) = problems.next_entry().await? {
        let metadata = path.metadata().await?;
        let filename = path.file_name().to_string_lossy().to_string();
        if metadata.is_dir() && filename.starts_with("day") {
            considered.push(format_ident!("{}", filename));
            considered_upper.push(format_ident!("{}", title_case(&filename)));
        }
    }

    considered.sort();
    considered_upper.sort();

    let main_content = quote! {
        use mimalloc::MiMalloc;

        #[global_allocator]
        static GLOBAL: MiMalloc = MiMalloc;

        use common::{Problem, BenchmarkCollection};

        #( mod #considered; )*

        #( use #considered::*; )*

        fn main() {
            let args = std::env::args().collect::<Vec<_>>();
            let problems: Vec<Box<dyn Problem>> = vec![
                #( Box::new(#considered_upper) ),*
            ];

            if args.contains(&"bench".to_string()) {
                for problem in problems {
                    let bench = problem.bench_part1();
                    println!("{} - Part 1: {:?} ({} runs)", problem.name(), bench.average(), bench.results.len());

                    let bench = problem.bench_part2();
                    println!("{} - Part 2: {:?} ({} runs)", problem.name(), bench.average(), bench.results.len());
                }
            } else if args.contains(&"bench-md".to_string()) {
                let mut collection = BenchmarkCollection::new(format!("Advent of Code {}", #year));
                for problem in problems {
                    let bench = problem.bench_part1();
                    collection.add(bench);

                    let bench = problem.bench_part2();
                    collection.add(bench);
                }
                println!("{}", collection.to_markdown());
            } else {
                for problem in problems {
                    println!("{} - Part 1: {}", problem.name(), problem.solve_part1());
                    println!("{} - Part 2: {}", problem.name(), problem.solve_part2());
                }
            }
        }
    };

    Ok(main_content)
}

async fn write_main(year: u32) -> anyhow::Result<()> {
    let main_content = update_main(year).await?;
    let main_file = prettyplease::unparse(&syn::parse_file(&main_content.to_string())?);

    tokio::fs::write(Path::new(&format!("crates/aoc{}/src/main.rs", year)), main_file).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenvy::dotenv();
    let env = envy::from_env::<Environment>()?;
    env.init_logger();

    let args = std::env::args().collect::<Vec<_>>();
    if args.len() < 3 {
        error!("Usage: aoc-update <year> <day>");
        return Ok(());
    }

    let year = args[1].parse::<u32>()?;
    if args[2] == "only-main" {
        write_main(year).await?;
        return Ok(());
    }
    let day = args[2].parse::<u8>()?;

    info!(year, day, "Fetching question");

    let config_dirs = Config::new("aoc-update");
    config_dirs.config.create_async().await?;
    let token_path = config_dirs.config.path.join("token.txt");

    info!("Reading auth token from {:?}", token_path);

    let token = tokio::fs::read_to_string(&token_path).await.inspect_err(|e| {
        error!("Failed to read token: {}", e);
    })?;

    info!("Read token!");

    let token_cookie = format!("session={}; Domain=adventofcode.com", token);
    let cookie_jar = Arc::new(reqwest::cookie::Jar::default());
    cookie_jar.add_cookie_str(&token_cookie, &"https://adventofcode.com".parse::<reqwest::Url>()?);
    let client = reqwest::ClientBuilder::new().cookie_provider(cookie_jar).build()?;

    let base_path = format!("crates/aoc{}/src/day{:02}", year, day);
    let output_path = Path::new(&base_path);
    if args.len() > 3 && args[3] == "only-input" {
        tokio::fs::create_dir_all(output_path).await?;
        tokio::fs::write(
            output_path.join("input.txt"),
            get_input(&client, year, day).await.unwrap_or_default(),
        )
        .await?;
        return Ok(());
    }

    let (title, comment_preamble) = get_description(&client, year, day).await?;
    if args.len() > 3 && args[3] == "only-desc" {
        // Read the existing file, and replace the comment block at the top with the
        // fresh description.
        let file = tokio::fs::read_to_string(output_path.join("mod.rs")).await?;
        let tokens = syn::parse_file(&file)?;

        let mut new_stream = TokenStream::new();

        for t in tokens.items {
            match t {
                syn::Item::Struct(mut s) => {
                    if !s.ident.to_string().starts_with("Day") {
                        new_stream.extend(quote! { #s });
                    } else {
                        s.attrs.clear();
                        new_stream.extend(quote! {
                            #comment_preamble
                            #s
                        });
                    }
                }
                _ => {
                    new_stream.extend(quote! { #t });
                }
            }
        }
        tokio::fs::write(
            output_path.join("mod.rs"),
            prettyplease::unparse(&syn::parse_file(&new_stream.to_string())?),
        )
        .await?;
        return Ok(());
    }

    let problem_ident = format_ident!("Day{:02}", day);

    let final_doc = quote! {
        use common::{Problem, Solution};

        #comment_preamble
        pub struct #problem_ident;

        impl Problem for #problem_ident {
            fn problem_input(&self) -> &'static str {
                include_str!("input.txt")
            }

            fn day(&self) -> u8 {
               #day
            }

            fn name(&self) -> &str {
                #title
            }

            fn solve_part1_with(&self, input: &str) -> Solution {
                Solution::Todo
            }

            fn solve_part2_with(&self, input: &str) -> Solution {
                Solution::Todo
            }
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn test_part1_example() {
                assert_eq!(1, 1);
            }

            #[test]
            fn test_part1_real_input() {
                let problem = #problem_ident {};
                assert_eq!(problem.solve_part1(), Solution::Todo);
            }

            #[test]
            fn test_part2_example() {
                assert_eq!(1, 1);
            }

            #[test]
            fn test_part2_real_input() {
                let problem = #problem_ident {};
                assert_eq!(problem.solve_part2(), Solution::Todo);
            }
        }
    };

    let pretty_file = prettyplease::unparse(&syn::parse_file(&final_doc.to_string())?);
    tokio::fs::create_dir_all(output_path).await?;

    tokio::fs::write(
        output_path.join("input.txt"),
        get_input(&client, year, day).await.unwrap_or_default(),
    )
    .await?;
    tokio::fs::write(output_path.join("mod.rs"), pretty_file).await?;

    write_main(year).await?;

    Ok(())
}
