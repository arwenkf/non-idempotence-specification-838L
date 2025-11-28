
use regex::Regex;
use std::collections::HashMap;
use std::fmt::format;
use std::fs::{read_to_string, File, OpenOptions};
use std::io::{self, BufReader, BufWriter, Write, BufRead};


#[derive(Debug, Clone)]
enum Value {
    Int(i64),
    Str(String),
    Bool(bool),
    Other(String),  // fallback for now
}

fn parse_value(s: &str) -> Value {
    let s = s.trim();

    // integer
    if let Ok(num) = s.parse::<i64>() {
        return Value::Int(num);
    }

    // string
    if s.starts_with('"') && s.ends_with('"') {
        return Value::Str(s.trim_matches('"').to_string());
    }

    // boolean
    if s == "true" { return Value::Bool(true); }
    if s == "false" { return Value::Bool(false); }

    // anything else
    Value::Other(s.to_string())
}

fn value_to_rust(v: &Value) -> String {
    match v {
        Value::Int(n) => format!("{}", n),
        Value::Str(s) => format!("\"{}\"", s),
        Value::Bool(b) => format!("{}", b),
        Value::Other(expr) => expr.clone(),
    }
}

fn main() {
    let _ = create_simul("src/input/ex1.rs", &[4]);
}



fn create_simul(file: &str, line_nums: &[u32]) -> io::Result<()> {
    let re_crash = Regex::new(r"^//crash point").unwrap();
    let re_atomic = Regex::new(r"//atomic start").unwrap();
    let re_nids = Regex::new(r"nids").unwrap();
    let re_nid_vars = Regex::new(r"([a-z\_]+)").unwrap();
    let let_binding = Regex::new(r"let (mut)? *([a-z\_]+) *=.+").unwrap();


    // TODO:
    // find a better way to integrate with ast part to update values
    // annotate with the pre and post conditions and raise error if that doesnt happen

    // let mut r = Vec::new();
    // let mut lines_since_last_atomic = vec![];
    let out_file_path = "output.rs";
    let mut curr_line_num = 0;
    let mut loop_label = 0;
    let mut exec_num = 0;
    let mut exec_var = "exec_num";
    let mut nids_mem: HashMap<&str, Option<Value>> = HashMap::new();
    let mut pend_atom = false;
    let mut in_atomic = false;
    let mut brace_depth = 0;


    let mut output = File::create(out_file_path)?;

    for line in read_to_string(file).unwrap().lines() {

        if re_nids.is_match(line) {
            let mut nids = vec![];
            for (_, [var]) in re_nid_vars.captures_iter(line).map(|c| c.extract()) {
                nids.push(var);
            }
            for nid in nids {
                nids_mem.insert(nid, None); //initialize nids
            }
        }
        // println!("{}", line);

        if re_atomic.is_match(line) {
            // if we see we are at the beginning of an atomic region
            loop_label += 1;
            pend_atom = true;
        }

        if pend_atom && line.trim_start().starts_with("fn") {
            // if there was an atomic above and we're now at the function
            // write the func header
            in_atomic = true;
            writeln!(output, "{}", line)?;

            // if the `{` is on the same line, inject right away
            if line.contains("{") {
                println!("{}", line);
                // println!("Hellloooo");

                let to_write = format!("let mut exec_num = 1; \n 'label{loop_label}: loop {{");
                writeln!(output, "{}", to_write)?;
                pend_atom = false;
               
                brace_depth = 1;

            } else {
                brace_depth = 0;
            }
            // continue reading normally
            curr_line_num+=1;
            continue;
        }

        if pend_atom && line.contains("{") {
            // in case the brace is elsewhere
            writeln!(output, "{}", line)?;
            let to_write = format!("let mut exec_num = 1; \n 'label{loop_label}: loop {{");
            writeln!(output, "{}", to_write)?;
            
            pend_atom = false;
            curr_line_num+=1;
            continue;
        }

        if in_atomic {
            if brace_depth == 0 && line.contains("{") {
                // detect opening if wasn't on previous line
                brace_depth = 1;
                writeln!(output, "{}", line)?;
                let to_write = format!("let mut exec_num = 1; \n 'label{loop_label}: loop {{");
                writeln!(output, "{}", to_write)?;
                
                pend_atom = false;
                curr_line_num+=1;
                continue;
            }

            // count braces in this line
            let opens = line.matches('{').count() as i32;
            let closes = line.matches('}').count() as i32;

            // if this line has the closing brace that ends the function
            if brace_depth + opens - closes == 0 {
                // end loop
                writeln!(output, "break; \n }}")?;

                // now write the closing brace line TODO-- does this need fixing?
                writeln!(output, "{}", line)?;

                in_atomic = false;
                brace_depth = 0;
                curr_line_num+=1;
                continue;
            }

            if line_nums.contains(&curr_line_num) {
                println!("MEWO {}", curr_line_num);
                
                let to_write = format!("if ({} == {}) {{
                    {}+=1;
                    continue \'label{};
                }}", exec_var, exec_num, exec_var, loop_label);
                // get values operations -- may need to consult w anjali for this


                // update values to their current thing following the reboot 
                for (name, val) in &nids_mem {
                    if val.is_some() {
                        let expr = value_to_rust(&val.clone().unwrap());
                        writeln!(output, "{name} = {expr};")?;
                    }
                }

                exec_num += 1;
                writeln!(output, "{}", to_write)?;
            } 

            // otherwise, just update depth and forward the line
            brace_depth += opens - closes;
            writeln!(output, "{}", line)?;
            curr_line_num+=1;
            continue;
        }

        // if we are at a line number where there should be a crash point
        println!("{}", curr_line_num);



        writeln!(output, "{}", line)?;
        curr_line_num+=1;

    }
    Ok(())
}
