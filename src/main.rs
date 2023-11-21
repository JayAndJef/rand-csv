pub mod gen;

use std::fs::File;

use argh::FromArgs;
use gen::{parse_args, generate_random_csv};

#[derive(FromArgs)]
/// Generate random CSVs blazingly fast
struct RandCSV {
    /// file to write csv to
    #[argh(option)]
    output_file: String,

    /// number of rows
    #[argh(option)]
    row_count: u32,

    /// column types
    #[argh(positional)]
    columns: Vec<String>,
}

fn main() {
    let args: RandCSV = argh::from_env();
    
    match parse_args(args.columns) {
        Ok(column_tokens) => {
            let out_file = File::create(args.output_file)
                .expect("Could not create new file");
            if let Err(e) = generate_random_csv(out_file, args.row_count, column_tokens) {
                println!("error: {}", e);
            }
        },

        Err(e) => println!("error while parsing arguments: {:?}", e)
    }
}
