use std::io::{Read, Seek, Write};
use std::time::Duration;

use zip::{ZipArchive, ZipWriter};

pub fn process<W: Write + Seek, R: Read + Seek>(input: R, output: &mut W, time: Duration) {
    let mut input = ZipArchive::new(input).unwrap();
    let mut output = ZipWriter::new(output);

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
                .set_text((time.as_secs() / 60).to_string());

            xml.to_string().into_bytes()
        } else {
            // No modify
            bin
        };

        output.write_all(&bin).unwrap();
    }

    output.finish().unwrap();
}
