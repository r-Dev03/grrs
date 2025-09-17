use clap::Parser;

#[derive(Parser)]
struct Cli {
  pattern: String,
  path: std::path::PathBuf,
}


fn main() {
  let args = Cli::parse();
  let content = std::fs::read_to_string(&args.path).expect("could not read file");

  for line in content.lines() {
    if line.contains(&args.pattern) {
      println!("{}", line);
    }
  }

  let result = std::fs::read_to_string("test.txt");
  match result {
    Ok(content) => {println!("File content: {}", content); }
    Err(error) => { println!("Oh noes: {}", error); }
}
}
