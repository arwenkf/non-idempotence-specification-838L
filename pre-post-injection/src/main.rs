use std::collections::HashMap;
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let source_code = fs::read_to_string("src/inputs/ex.rs")
        .expect("Could not read input file");

    // nid states mock for now
    let mut nid_states: HashMap<&str, Vec<(i32, usize)>> = HashMap::new();
    nid_states.insert("x", vec![(0, 8), (5, 3), (3, 4)]);
    nid_states.insert("rb", vec![(-1, 9), (1, 2)]);

    let new_file_content = inject_pre_post(&source_code, &nid_states);

    fs::write("output.rs", new_file_content)?;
    println!("Pre and post conditions injected");
    Ok(())
}

fn inject_pre_post(source: &str, nid_states: &HashMap<&str, Vec<(i32, usize)>>) -> String {
    // line number -> list (nid_var, old_val, new_value)
    let mut lines_to_modify: HashMap<usize, Vec<(&str, Option<i32>, i32)>> = HashMap::new();

    
    for (var_name, history) in nid_states {
        for (index, (current_val, line_num)) in history.iter().enumerate() {
            
            // Look at i-1 for previous value to derive the condition
            let prev_val = if index > 0 {
                Some(history[index - 1].0)
            } else {
                None // has not been init
            };

            lines_to_modify
                .entry(*line_num)
                .or_insert_with(Vec::new)
                .push((var_name, prev_val, *current_val));
        }
    }

    let mut output = String::new();

    for (index, line_content) in source.lines().enumerate() {
        let line_number = index + 1;

        if let Some(modifications) = lines_to_modify.get(&line_number) {
            let indentation: String = line_content
                .chars()
                .take_while(|c| c.is_whitespace())
                .collect();

            // deriving pre-conditions
            for (var_name, old_val, _new_val) in modifications {
                match old_val {
                    Some(v) => {
                        output.push_str(&format!(
                            "{}/* << Pre-condition: {} == {} >> */\n", 
                            indentation, var_name, v
                        ));
                    },
                    None => {
                        // nid has not been initilized yet
                        output.push_str(&format!(
                            "{}/* << Pre-condition: true >> */\n", 
                            indentation
                        ));
                    }
                }
            }
            
            // original line
            output.push_str(line_content);
            output.push_str("\n");

            // deriving post condition
            for (var_name, _old_val, new_val) in modifications {
                output.push_str(&format!(
                    "{}/* << Post-condition: {} == {} >> */\n\n", 
                    indentation, var_name, new_val
                ));
            }
        } else {
            output.push_str(line_content);
            output.push_str("\n");
        }
    }

    output
}