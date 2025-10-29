use regex::Regex;
use std::fs::{read_to_string, File, OpenOptions};
use std::io::{self, BufReader, BufWriter, Write, BufRead};

fn main() {
    
}



fn copy_sections(file: &str) {
    let re_crash = Regex::new(r"^//crash point").unwrap();
    let re_atomic = Regex::new(r"//atomic start").unwrap();
    let re_nids = Regex::new(r"nids").unwrap();
    let re_nid_vars = Regex::new(r"([a-z\_]+)").unwrap();
    let let_binding = Regex::new(r"").unwrap();


    // at a crash point, copy everything again so it "reexecutes". keep 
    // track of nids and everything else reset.

    let mut r = Vec::new();
    let mut lines_since_last_atomic = vec![];
    let out_file_path = "output.rs";

    let output_file = OpenOptions::new()
    .write(true)
    .create(true)
    .truncate(true)
    .open(out_file_path).unwrap();

    for line in read_to_string(file).unwrap().lines() {
        if re_atomic.is_match(line) {
            lines_since_last_atomic = vec![];
        } else {
            lines_since_last_atomic.push(line);
        }

        if re_nids.is_match(line) {
            let mut nids = vec![];
            for (_, [var]) in re_nid_vars.captures_iter(line).map(|c| c.extract()) {
                nids.push(var);
            }
        }

        if {

        }

        if re_crash.is_match(line) {
            let mut writer = BufWriter::new(output_file);

            for line_result in lines_since_last_atomic {
                let line = line_result; 
                writeln!(writer, "{}", line); 
            }

            writer.flush();
            lines_since_last_atomic = vec![];

        }

    }

}
