use clap::Parser;
use std::fs::File;
use std::io::{Read, Write};
use zip::read::ZipArchive;
use zip::write::ZipWriter;

#[derive(Parser)]
struct Cli {
    #[arg(long, short)]
    input: String,

    #[arg(long, short)]
    output: String,

    #[arg(long, short)]
    time: humantime::Duration,
}

fn main() {
    let cli = Cli::parse();

    let input = File::open(&cli.input).unwrap();
    let mut input = ZipArchive::new(input).unwrap();

    let mut output = File::create(&cli.output).unwrap();
    let mut output = ZipWriter::new(&mut output);

    for n in 0..input.len() {
        let mut file = input.by_index(n).unwrap();
        output.start_file(file.name(), file.options()).unwrap();

        let mut bin = vec![];
        file.read_to_end(&mut bin).unwrap();

        let bin = if file.name() == "docProps/app.xml" {
            let xml = String::from_utf8(bin).unwrap();
            let mut xml = kiss_xml::parse_str(&xml).unwrap();

            xml.root_element_mut()
                .first_element_by_name_mut("TotalTime")
                .unwrap()
                .set_text(&(cli.time.as_secs() / 60).to_string());

            xml.to_string().into_bytes()
        } else {
            // No modify
            bin
        };

        output.write(&bin).unwrap();
    }

    output.finish().unwrap();
}
