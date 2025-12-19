use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::fmt::Write; 
use std::env;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        std::process::exit(1);
    }
    
    let file_to_read: String = args[1].clone();
    let source_code = fs::read_to_string(&file_to_read)
                .expect("Failed to read input file");


    let mut nid_track: HashMap<&str, Vec<Option<(i32, usize)>>> = HashMap::new();

    nid_track.insert("x", vec![
        Some((0, 7)), 
        Some((5, 2)),    
        Some((3, 3))
    ]);

    let new_file_content = inject_pre_post(&source_code, &nid_track);
    
    let path = Path::new(&file_to_read);
    
    let file_stem = path.file_name()
        .and_then(|os_str| os_str.to_str())
        .unwrap_or("unknown.rs");

    let output_file = format!("output_{}", file_stem);

    fs::write(&output_file, new_file_content)?;
    println!("Pre and post conditions injected into {}", output_file);
    Ok(())
}

fn inject_pre_post(source: &str, states: &HashMap<&str, Vec<Option<(i32, usize)>>>) -> String {
    
    let mut line_lookup: HashMap<usize, Vec<(&str, i32)>> = HashMap::new();
    
    // To remove duplicates and None(s)
    let mut seen: HashSet<(&str, i32, usize)> = HashSet::new();

    for (var_name, changes) in states {
        for entry in changes {

            if let Some((value, line_num)) = entry {
                if seen.insert((*var_name, *value, *line_num)) {
                    line_lookup.entry(*line_num)
                        .or_insert_with(Vec::new)
                        .push((*var_name, *value));
                }
            }
        }
    }

    let mut output = String::new();

    // to check equality without worrying about deref
    output.push_str(
r#"trait SmartEq<T> {
    fn smart_eq(&self, other: T) -> bool;
}

impl<T: PartialEq + Copy> SmartEq<T> for T {
    fn smart_eq(&self, other: T) -> bool {
        *self == other
    }
}

"#);

    // lookup util for nid_track
    output.push_str(
r#"fn lookup(
    nid_track: &HashMap<&str, Vec<Option<(i32, usize)>>>, 
    var_name: &str, 
    exec_num: usize
) -> Option<i32> {
    nid_track.get(var_name)            
        .and_then(|v| v.get(exec_num))  
        .and_then(|opt| *opt)           
        .map(|(val, _)| val)           
}

"#);

    
    for (i, line) in source.lines().enumerate() {
        let line_num = i + 1;
        let indentation = line.chars().take_while(|c| c.is_whitespace()).collect::<String>();

        if let Some(values) = line_lookup.get(&line_num) {
            let mut sorted_values = values.clone();
            sorted_values.sort();

            // Inject precondition
            for (var_name, _val) in &sorted_values {
                writeln!(
                    output, 
                     "{}if !(lookup({}, exec_num).is_none() || {}.smart_eq(lookup({}, exec_num).unwrap())) {{ panic!(\"Pre-condition failed\"); }}",
                    indentation, var_name, var_name, var_name
                ).unwrap();
            }

            // append original line
            writeln!(output, "{}", line).unwrap();

            // Inject postcondition
            for (var_name, val) in &sorted_values {
                writeln!(
                    output, 
                     "{}if !{}.smart_eq({}) {{ panic!(\"Post-condition failed\"); }}",
                    indentation, var_name, val
                ).unwrap();
            }
        } else {
            writeln!(output, "{}", line).unwrap();
        }
    }

    output
}