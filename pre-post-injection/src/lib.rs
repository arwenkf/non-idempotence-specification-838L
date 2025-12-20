use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::fmt::Write; 
use std::env;
use std::fs;

pub fn run(nid_track: &HashMap<String, Vec<Option<(i32, usize)>>>, file_to_read: String) -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        std::process::exit(1);
    }
    
    let source_code = fs::read_to_string(&file_to_read)
                .expect("Failed to read input file");


    let new_file_content = inject_pre_post(&source_code, nid_track);
    
    let path = Path::new(&file_to_read);
    
    let file_stem = path.file_name()
        .and_then(|os_str| os_str.to_str())
        .unwrap_or("unknown.rs");

    let output_file = format!("output_{}", file_stem);

    fs::write(&output_file, new_file_content)?;
    println!("Pre and post conditions injected into {}", output_file);
    Ok(())
}

fn inject_pre_post(source: &str, states: &HashMap<String, Vec<Option<(i32, usize)>>>) -> String {
    
    let mut line_lookup: HashMap<usize, Vec<(String, i32)>> = HashMap::new();
    
    // To remove duplicates and None(s)
    let mut seen: HashSet<(&String, i32, usize)> = HashSet::new();

    for (var_name, changes) in states {
        for entry in changes {

            if let Some((value, line_num)) = entry {
                if seen.insert((var_name, *value, *line_num)) {
                    line_lookup.entry(*line_num)
                        .or_insert_with(Vec::new)
                        .push((var_name.to_string(), *value));
                }
            }
        }
    }

    let mut output = String::new();

    output.push_str("use once_cell::sync::Lazy;\n");
    output.push_str("use std::collections::HashMap;\n\n");
    
    output.push_str("static NID_TRACK: Lazy<HashMap<&'static str, Vec<Option<(i32, usize)>>>> =\n");
    output.push_str("    Lazy::new(|| {\n");
    output.push_str("        let mut m = HashMap::new();\n");

    for (var_name, vec_data) in states {
        write!(output, "        m.insert(\"{}\", vec![", var_name).unwrap();
        let items: Vec<String> = vec_data.iter().map(|opt| {
            match opt {
                Some((val, line)) => format!("Some(({}, {}))", val, line),
                None => "None".to_string(),
            }
        }).collect();
        output.push_str(&items.join(", "));
        output.push_str("]);\n");
    }
    
    output.push_str("        m\n");
    output.push_str("    });\n\n");

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
r#"fn lookup(var: &str, exec_num: i32) -> Option<i32> {
    NID_TRACK
        .get(var)
        .and_then(|v| v.get(exec_num as usize)) 
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
                     "{}(if !(lookup(\"{}\", exec_num).is_none() || {}.smart_eq(lookup(\"{}\", exec_num).unwrap())) {{ panic!(\"Pre-condition failed\"); }});",
                    indentation, var_name, var_name, var_name
                ).unwrap();
            }

            // append original line
            writeln!(output, "{}", line).unwrap();

            // Inject postcondition
            for (var_name, val) in &sorted_values {
                writeln!(
                    output, 
                     "{}(if !{}.smart_eq({}) {{ panic!(\"Post-condition failed\"); }});",
                    indentation, var_name, val
                ).unwrap();
            }
        } else {
            writeln!(output, "{}", line).unwrap();
        }
    }

    output
}