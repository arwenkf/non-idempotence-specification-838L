
use regex::Regex;
use std::collections::HashMap;
use std::fmt::{format, write};
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


// [ Some(0, 9),  Some(5, 2), Some(3, 3)] for the second example and not [None, None, Some(0, 9),  Some(5, 2), Some(5, 2), Some(3, 3)]

// input is: HashMap<String, Vec<(i32, i32)>>
// variable name -> [(value, line number)]
// only where they are modified
// will return HashMap<String, Vec<Option<(i32, usize)>>> to send to pre/post

fn build_struct(file: &str, nid_track: &HashMap<String, Vec<(i32, i32)>>) -> HashMap<String, Vec<Option<(i32, usize)>>> {
    let mut nid_return: HashMap<String, Vec<Option<(i32, usize)>>> = HashMap::new(); // where values will go as it updates
    let mut nids_mem: HashMap<String, Option<i32>> = HashMap::new();
    let mut by_line: HashMap<i32, Vec<(String, i32)>> = HashMap::new(); // track by line number for easier processing
    let re_nids = Regex::new(r"nids").unwrap();
    let re_nid_vars = Regex::new(r"([a-z\_]+)").unwrap();
    let mut curr_line_num = 1;


    let mut line_nums = vec![];
    for (_ ,v) in nid_track {
        for (_, line) in v.iter().skip(1) { // currently makes it so that the init line is not counted since that's the init
            line_nums.push(line.clone()); // making the line numbers where crashes should occur
            
        }
    }

    for (var ,v) in nid_track {
        for (value, line) in v.iter() { 
            by_line
                .entry(*line)
                .or_default()
                .push((var.clone(), *value));
        }
        nid_return.entry(var.to_string()).or_default().push(None); // manually start at None
    }

    for line in read_to_string(file).unwrap().lines() {
        if re_nids.is_match(line) {
            let mut nids = vec![];
            for (_, [var]) in re_nid_vars.captures_iter(line).map(|c| c.extract()) {
                nids.push(var);
            }
            for nid in nids {
                nids_mem.insert(nid.to_string(), None); //initialize nids -- this is the "current" value
                // and will be added to the bigger ds each time
            }
        }

        if by_line.contains_key(&curr_line_num) { // if the current line number has a relevant mod (should be including updates)

            println!("{:?}", by_line);
            for (var, value) in by_line.get(&curr_line_num).unwrap() {
                println!("line {curr_line_num} reached, value added for {var}");
                if let Some(v) = nids_mem.get_mut(var) {
                    *v = Some(*value); // putting the current value in the nids_mem
                }
                println!("{:?}", nids_mem);

                if !line_nums.contains(&curr_line_num) { // ie it is the instantiated -- 
                    // this should always be first or right after the None
                    let temp = nids_mem.get(var).unwrap().unwrap();

                    let vec = nid_return.entry(var.to_string()).or_default();

                    let insert_at = vec.iter().take_while(|x| x.is_none()).count();
                    vec.insert(insert_at, Some((temp, curr_line_num as usize )));

                    // nid_return.entry(var.clone()).or_default().push(Some((temp, curr_line_num as usize)));
                }
            }
        }

        if line_nums.contains(&(curr_line_num)) {
            // println!("MEWO {}", curr_line_num);
        

            // update values to their current thing following the reboot 
            // println!("{:?}", nids_mem);
            for (name, val) in &nids_mem {
                // here we go, just have to update this
                if val.is_some() {
                    nid_return.entry(name.clone()).or_default().push(Some((val.unwrap(), curr_line_num as usize)));
                    // updates nid_return to contain the current value held
                } else {
                    nid_return.entry(name.clone()).or_default().push(None); // push a None if it's not instantiated yet :)
                }
                
            }
        }

        curr_line_num +=1;
    }



    nid_return.remove_entry("nids");
    println!("{:?}", nid_return);
    nid_return
}

fn update_struct(struc: &mut HashMap<String, Vec<Option<(i32, usize)>>>, map: &HashMap<i32,i32>) {
    for vec in struc.values_mut() {
        for opt in vec.iter_mut() {
            if let Some((_, x)) = opt {
                if let Some(new_x) = map.get(&(*x as i32)) {
                    *x = *new_x as usize;
                }
            }
        }
    }      
}

fn rewrite_calls(line: &str) -> String {
    let re_nonempty_call = Regex::new(r"\b([A-Za-z_][A-Za-z0-9_]*)(::<[^>]*>)?\s*\(([^()]*)\)").unwrap();
    let re_empty_call = Regex::new(r"\b([A-Za-z_][A-Za-z0-9_]*)(::<[^>]*>)?\s*\(\s*\)").unwrap();

    if let Some(caps) = re_empty_call.captures(&line) {
        let name = &caps[1];
        let generics = caps.get(2).map_or("", |m| m.as_str());

        return format!(
            "{}{}(&mut exec_num)",
            name, generics
        );
    } else

    if let Some(caps) = re_nonempty_call.captures(&line) {
        let name = &caps[1];
        let generics = caps.get(2).map_or("", |m| m.as_str());
        let args = &caps[3];

        return format!(
            "{}{}({}, &mut exec_num)",
            name, generics, args.trim()
        );
    };
    return "".to_string();
}

pub fn create_simul(file: &str, nid_track: HashMap<String, Vec<(i32, i32)>>) -> io::Result<HashMap<String, Vec<Option<(i32, usize)>>>> {
    // let re_crash = Regex::new(r"^//crash point").unwrap();
    let re_atomic = Regex::new(r"//atomic start").unwrap();
    let re_nids = Regex::new(r"nids").unwrap();
    let re_nid_vars = Regex::new(r"([a-z\_]+)").unwrap();
    let re_ref = Regex::new(r"([a-z]+)\s*:\s*&").unwrap();
    let re_fn:Regex = Regex::new(r"(fn\s*[A-Za-z_][A-Za-z0-9_]*)\s*\(([^)]*)\)(.*)").unwrap();
    let re_fn_no_param = Regex::new(r"(fn\s+[A-Za-z_][A-Za-z0-9_]*)\s*\(\s*\)(.*)").unwrap();
    let re_main = Regex::new(r"fn\s*main\(\s*\)").unwrap();
    let re_call = Regex::new(r"\b([A-Za-z_][A-Za-z0-9_]*)(::<[^>]*>)?\s*\(").unwrap();
    let mut line_mappings: HashMap<i32, i32> = HashMap::new();
    let let_binding = Regex::new(r"let (mut)? *([a-z\_]+) *=.+").unwrap();
    // let mut nid_return: HashMap<String, Vec<Option<(i32, usize)>>> = HashMap::new(); // where values will go as it updates
    // let mut by_line: HashMap<i32, Vec<(String, i32)>> = HashMap::new(); // track by line number for easier processing
    let mut ref_vars: Vec<String> = vec![];
    let mut data = build_struct(file, &nid_track);
    // println!("HELLO {:?}", data);

    // println!("{:?}", data);
    let mut line_nums = vec![];
    for (_ ,v) in &nid_track {
        for (_, line) in v.iter().skip(1) { // currently makes it so that the init line is not counted since that's the init
            line_nums.push(line.clone()); // making the line numbers where crashes should occur
            
        }
    }

    // TODO:
    // integrate with ast part to update values
    // make data struct as ceren specified
    // insert crashes (after mod and after each line)
    // eval with test suite

    // let mut r = Vec::new();
    // let mut lines_since_last_atomic = vec![];
    let out_file_path = "output.rs";
    let mut curr_line_num = 1;
    let mut loop_label = 0;
    let mut exec_num = 1;
    let exec_var = "exec_num";
    let mut nids_mem: HashMap<String, Option<i32>> = HashMap::new();
    let mut pend_atom = false;
    let mut in_atomic = false;
    let mut brace_depth = 0;
    let mut output_line_num = 1;
    
    

    let mut output = File::create(out_file_path)?;
    let mut defined_vars: Vec<String> = vec![]; //UGH

    for line in read_to_string(file).unwrap().lines() {
        println!("{curr_line_num}, {line}");
        if line.starts_with("//") {
            if re_nids.is_match(line) {
                let mut nids = vec![];
                for (_, [var]) in re_nid_vars.captures_iter(line).map(|c| c.extract()) {
                    nids.push(var);
                }
            }
            if re_atomic.is_match(line) {
                // if we see we are at the beginning of an atomic region
                loop_label += 1;
                pend_atom = true;
            }

            writeln!(output, "{}", line)?;
            curr_line_num+=1;
            output_line_num +=1;
            continue;
        }

        // println!("{}", line);

        if let_binding.is_match(line) {
            for (_, [var]) in re_nid_vars.captures_iter(line).map(|c| c.extract()) {
                defined_vars.push(var.to_string());
            }
            line_mappings.insert(curr_line_num, output_line_num);
        }
        



        if pend_atom && line.trim_start().starts_with("fn") {
            println!("A:LSKJD:LSKAJD:LJK {}", line);

            for (_, [var]) in re_ref.captures_iter(line).map(|c| c.extract()) {
                ref_vars.push(var.to_string());
                defined_vars.push(var.to_string());
            }
            // if there was an atomic above and we're now at the function
            // write the func header
            in_atomic = true;

            if re_main.is_match(line) {
                writeln!(output, "{}", line)?;
                output_line_num += 1;
                writeln!(output, "{}", format!(
                    "let mut exec_num = 1;"
                ))?;
                output_line_num +=1;
                // output_line_num += 1;
                // curr_line_num +=1 ;
                // continue;
            } else
            if let Some(caps) = re_fn_no_param.captures(line) {
                let head = &caps[1];
                let tail = &caps[2];

                writeln!(output, "{}", format!(
                    "{}(exec_num: &mut i32){}",
                    head, tail
                ))?;
                output_line_num +=1;
                // output_line_num += 1;
                // curr_line_num +=1 ;
                // continue;
            } else if let Some(caps) = re_fn.captures(line) {
                
                // println!("{:?}", caps);
                let head = &caps[1];
                let params = &caps[2];
                let tail = &caps[3];

                writeln!(output, "{}", format!(
                    "{}({}, exec_num: &mut i32){}",
                    head, params.trim(), tail
                ))?;
                output_line_num +=1;
                // output_line_num += 1;
                // curr_line_num +=1 ;
                // continue;
            }

            // writeln!(output, "{}", line)?;


            // if the `{` is on the same line, inject right away
            if line.contains("{") {
                // println!("{}", line);
                // println!("Hellloooo");

                let to_write = format!(" \n 'label{loop_label}: loop {{");
                writeln!(output, "{}", to_write)?;
                output_line_num += 2;
                
                let mut inject= "".to_string();
                for (var,values) in &data {
                    for num in 0..values.len() {
                        if values[num].is_some() {
                            let exec_num = num + 1;
                            let val = values[num].unwrap().0;
                            if ref_vars.contains(var) {
                                inject = format!("if {exec_var} == {exec_num} {{*{var} =  {val}}} //restored from mem");
                                writeln!(output, "{}", inject)?;
                                output_line_num += 1;
                            } else if defined_vars.contains(var){
                                inject = format!("if {exec_var} == {exec_num} {{ {var} = {val}}} //restored from mem");
                                writeln!(output, "{}", inject)?;
                                output_line_num += 1;
                            }
                            
                        }
                    }
                }
                

                
                pend_atom = false;
               
                brace_depth = 1;

            } else {
                brace_depth = 0;
            }
            // continue reading normally
            curr_line_num+=1;
            // println!("{curr_line_num}");
            continue;
        } else {
            // if it doesn't start with fn
            if re_call.is_match(line) && !line.trim_ascii_start().starts_with("fn"){
                writeln!(output, "{}", rewrite_calls(&line))?;
                output_line_num += 1;
                curr_line_num +=1;
                continue;
            }
        }

        if pend_atom && line.contains("{") {
            // in case the brace is elsewhere
            writeln!(output, "{}", line)?;
            output_line_num += 1;
            let to_write = format!("\n 'label{loop_label}: loop {{");
            writeln!(output, "{}", to_write)?;
            output_line_num += 2;
            
            pend_atom = false;
            curr_line_num+=1;
            continue;
        }

        if in_atomic {
            if brace_depth == 0 && line.contains("{") {
                // detect opening if wasn't on previous line
                brace_depth = 1;
                writeln!(output, "{}", line)?;
                output_line_num += 1;
                let to_write = format!(" \n 'label{loop_label}: loop {{");
                writeln!(output, "{}", to_write)?;
                output_line_num += 2;
                
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
                output_line_num += 2;
                ref_vars = vec![]; // reset refs to be empty
                defined_vars = vec![];
                // now write the closing brace line TODO-- does this need fixing?
                writeln!(output, "{}", line)?;
                output_line_num += 1;

                in_atomic = false;
                brace_depth = 0;
                curr_line_num+=1;
                continue;
            }

            if line_nums.contains(&(curr_line_num)) {
                // println!("MEWO {}", curr_line_num);
                
                writeln!(output, "{}", line)?;
                line_mappings.insert(curr_line_num, output_line_num);
                // this is a mod -- so this "output_line_num" needs to replace "curr_line_num"

                output_line_num += 1;

                let to_write = format!("if ({} == {}) {{
                    {}+=1;
                    continue \'label{};
                }}", exec_var, exec_num, exec_var, loop_label);
                // get values operations -- may need to consult w anjali for this


                exec_num += 1;
                writeln!(output, "{}", to_write)?;
                output_line_num += 4;
                curr_line_num+=1;
                continue;
            } 

            // otherwise, just update depth and forward the line
            brace_depth += opens - closes;
            writeln!(output, "{}", line)?;
            output_line_num += 1;
            curr_line_num+=1;
            continue;
        }

        // if we are at a line number where there should be a crash point
        // println!("{}", curr_line_num);



        writeln!(output, "{}", line)?;
        output_line_num += 1;
        curr_line_num+=1;

    }

    // nid_return.remove_entry("nids");
    // println!("{:?}", nid_return);
    update_struct(&mut data, &line_mappings);
    println!("{:?}", data);
    // println!("A:KJLK {:?}", data);
    println!("{output_line_num}");
    // println!("{:?}", line_mappings);
    Ok(data)
}
