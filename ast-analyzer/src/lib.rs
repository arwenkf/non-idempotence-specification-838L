use quote::ToTokens;
use core::num;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use syn::spanned::Spanned;
use syn::{visit::Visit, BinOp, Expr, ExprAssign, ExprBinary, File, ItemFn, ExprCall};

#[derive(Debug)]

// struct to hold single modification event
struct Modification {
    line: usize,
    operation: String, // function like f_n(x)
}

#[derive(Debug)]
// Visitor that tracks variables
struct VariableTracker {
    watched_vars: HashSet<String>, // list of NIDs
    history: HashMap<String, Vec<Modification>>, // maps variable name to list of its modifications
    functions: HashMap<String, ItemFn>,
}

type Env = HashMap<String, String>; // addition 1 for function extension

struct ScopedTracker<'a> { // addition 2 for function
    parent: &'a mut VariableTracker,
    env: &'a Env,
}

impl ScopedTracker<'_> {
    fn handle_write(&mut self, local: &str, rhs: Expr, span: proc_macro2::Span) {
        if let Some(caller) = self.env.get(local) {
            if self.parent.watched_vars.contains(caller) {
                let line = span.start().line;
                let rhs_str = rhs.to_token_stream().to_string();

                self.parent.history
                    .entry(caller.clone())
                    .or_default()
                    .push(Modification {
                        line,
                        operation: format!("{} = {}", caller, rhs_str),
                    });
            }
        }
    }

    fn handle_compound_write(&mut self, local: &str, node: &ExprBinary) {
        if let Some(caller) = self.env.get(local) {
            if self.parent.watched_vars.contains(caller) {
                let op = node.op.to_token_stream().to_string();
                let rhs = node.right.to_token_stream().to_string();
                let line = node.span().start().line;
                let math_op = op.trim_end_matches('=');

                self.parent.history
                    .entry(caller.clone())
                    .or_default()
                    .push(Modification {
                        line,
                        operation: format!(
                            "{} = {} {} ({})",
                            caller, caller, math_op, rhs
                        ),
                    });
            }
        }
    }
}

struct FunctionCollector { //addition 3 for function extension
    functions: HashMap<String, ItemFn>,
}

impl<'ast> Visit<'ast> for FunctionCollector {
    fn visit_item_fn(&mut self, node: &'ast ItemFn) {
        self.functions.insert(
            node.sig.ident.to_string(),
            node.clone(),
        );
    }
}

impl<'ast> Visit<'ast> for ScopedTracker<'_> {
    fn visit_expr_assign(&mut self, node: &'ast ExprAssign) {
        match &*node.left {
        // Existing: a = ...
            Expr::Path(path) => {
                if let Some(ident) = path.path.get_ident() {
                    self.handle_write(&ident.to_string(), *node.right.clone(), node.span());
                }
            }

            // NEW: *a = ...
            Expr::Unary(unary) if matches!(unary.op, syn::UnOp::Deref(_)) => {
                if let Expr::Path(path) = &*unary.expr {
                    if let Some(ident) = path.path.get_ident() {
                        self.handle_write(&ident.to_string(), *node.right.clone(), node.span());
                    }
                }
            }

            _ => {}
        }

        syn::visit::visit_expr_assign(self, node);
    }

    fn visit_expr_binary(&mut self, node: &'ast ExprBinary) {
        // println!("{:?}", node);
        let is_compound_assignment = match node.op {
            BinOp::AddAssign(_) | BinOp::SubAssign(_) | BinOp::MulAssign(_) | 
            BinOp::DivAssign(_) | BinOp::RemAssign(_) | BinOp::BitXorAssign(_) | 
            BinOp::BitAndAssign(_) | BinOp::BitOrAssign(_) | BinOp::ShlAssign(_) | 
            BinOp::ShrAssign(_) => true,
            _ => false,
        };

        if is_compound_assignment {
            match &*node.left {
            // a += ...
            Expr::Path(path) => {
                if let Some(ident) = path.path.get_ident() {
                    self.handle_compound_write(&ident.to_string(), node);
                }
            }

            // *a += ...
            Expr::Unary(unary) if matches!(unary.op, syn::UnOp::Deref(_)) => {
                if let Expr::Path(path) = &*unary.expr {
                    if let Some(ident) = path.path.get_ident() {
                        self.handle_compound_write(&ident.to_string(), node);
                    }
                }
            }

            _ => {}
            }
            
        }   
        syn::visit::visit_expr_binary(self, node);
    }
}

impl VariableTracker {
    fn new(targets: Vec<String>, collector: FunctionCollector ) -> Self {
        VariableTracker {
            watched_vars: targets.into_iter().collect(),
            history: HashMap::new(),
            functions: collector.functions,
        }
    }

    fn process_function_call(&mut self, func: &ItemFn, call: &ExprCall) {
        let mut env = HashMap::new();

        for (param, arg) in func.sig.inputs.iter().zip(call.args.iter()) {
            let param_ident = match param {
                syn::FnArg::Typed(pat_ty) => {
                    if let syn::Pat::Ident(pat_ident) = &*pat_ty.pat {
                        pat_ident.ident.to_string()
                    } else {
                        continue;
                    }
                }
                _ => continue,
            };

            match arg {
                // foo(x)
                Expr::Path(path) => {
                    if let Some(arg_ident) = path.path.get_ident() {
                        env.insert(param_ident, arg_ident.to_string());
                    }
                }

                // foo(&mut x)
                Expr::Reference(expr_ref) if expr_ref.mutability.is_some() => {
                    if let Expr::Path(path) = &*expr_ref.expr {
                        if let Some(arg_ident) = path.path.get_ident() {
                            env.insert(param_ident, arg_ident.to_string());
                        }
                    }
                }

                _ => {}
            }
        }

        self.visit_function_body_with_env(func, &env);
    }

    fn visit_function_body_with_env(&mut self, func: &ItemFn, env: &HashMap<String, String>) {
        let mut scoped = ScopedTracker {
            parent: self,
            env,
        };

        scoped.visit_block(&func.block);
    }
}


// implement Visit trait
impl<'ast> Visit<'ast> for VariableTracker {
    // handles assignments i.e. x = 2
    fn visit_expr_assign(&mut self, node: &'ast ExprAssign) {
        if let Expr::Path(expr_path) = &*node.left {
            if let Some(ident) = expr_path.path.get_ident() {
                let var_name = ident.to_string();
                
                // filter so that the only vars we actually process are NIDs
                if self.watched_vars.contains(&var_name) {
                    let rhs = node.right.to_token_stream().to_string();
                    let line = node.span().start().line;
                    let func_repr = format!("{} = {}", var_name, rhs);

                    self.history.entry(var_name).or_default().push(Modification {
                        line,
                        operation: func_repr,
                    });
                }
            }
        }
        // traverse down children
        syn::visit::visit_expr_assign(self, node);
    }

    // handles operators i.e. x += 1
    fn visit_expr_binary(&mut self, node: &'ast ExprBinary) {
        let is_compound_assignment = match node.op {
            BinOp::AddAssign(_) | BinOp::SubAssign(_) | BinOp::MulAssign(_) | 
            BinOp::DivAssign(_) | BinOp::RemAssign(_) | BinOp::BitXorAssign(_) | 
            BinOp::BitAndAssign(_) | BinOp::BitOrAssign(_) | BinOp::ShlAssign(_) | 
            BinOp::ShrAssign(_) => true,
            _ => false,
        };

        if is_compound_assignment {
            if let Expr::Path(expr_path) = &*node.left {
                if let Some(ident) = expr_path.path.get_ident() {
                    let var_name = ident.to_string();

                    // only process if var is an NID
                    if self.watched_vars.contains(&var_name) {
                        let op = node.op.to_token_stream().to_string();
                        let rhs = node.right.to_token_stream().to_string();
                        let line = node.span().start().line;

                        // only keep the op (so just the '+' in a '+=')
                        let math_op = op.trim_end_matches('=');
                        let func_repr = format!("{} = {} {} ({})", var_name, var_name, math_op, rhs);

                        self.history.entry(var_name).or_default().push(Modification {
                            line,
                            operation: func_repr,
                        });
                    }
                }
            }
        }
        syn::visit::visit_expr_binary(self, node);
    }

    // handling function calls and following traces

    

    fn visit_item_fn(&mut self, node: &'ast ItemFn) {
        let name = node.sig.ident.to_string();
        self.functions.insert(name, node.clone());

        syn::visit::visit_item_fn(self, node);
    }

    fn visit_expr_call(&mut self, node: &'ast ExprCall) {
         
        if let Expr::Path(path) = &*node.func {
            if let Some(func_ident) = path.path.get_ident() {
                let func_name = func_ident.to_string();

                if let Some(func_def) = self.functions.get(&func_name).cloned() {
                    self.process_function_call(&func_def, node);
                }
            }
        }

        syn::visit::visit_expr_call(self, node);
    }

    fn visit_local(&mut self, node: &'ast syn::Local) {
        if let Some(local_init) = &node.init {
            if let syn::Pat::Ident(pat_ident) = &node.pat {
                let var_name = pat_ident.ident.to_string();

                if self.watched_vars.contains(&var_name) {
                    let rhs = local_init.expr.to_token_stream().to_string();
                    let line = node.span().start().line;
                
                    let func_repr = format!("{} = {}", var_name, rhs);

                    self.history.entry(var_name).or_default().push(Modification {
                        line,
                        operation: func_repr,
                    });
                }
            }
        }å
        syn::visit::visit_local(self, node);
    }
}

// parse NIDs from user-given annotation at the top of a program
fn parse_nids_header(code: &str) -> Vec<String> {
    for line in code.lines() {
        let trimmed = line.trim();
        // check for the specific prefix
        if let Some(content) = trimmed.strip_prefix("// #[nids(") {
            // check for the suffix
            if let Some(inner) = content.strip_suffix(")]") {
                return inner
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
            }
        }
    }
    vec![] // return empty if no annotation for NIDs found
}

fn count_lines(file_name: &String) -> usize {
    let file = fs::File::open(file_name).expect("Input file could not be opened");
    let reader = BufReader::new(file);
    // return number of lines
    reader.lines().count()
}

fn build_value_structure(tracker: &VariableTracker, num_of_reboots: usize, reboot_lines: Vec<usize>)-> HashMap<String, Vec<(Option<i32>, usize)>> {
    let mut to_ret: HashMap<String, Vec<(Option<i32>, usize)>> = HashMap::new();
    let nids = &tracker.watched_vars;
    // which should equal num of mods by default, otherwise num of lines
    
    for var in &tracker.watched_vars {
        // initialize to num of reboots and None at each
        to_ret.insert(var.clone(), vec![]);
    }

    let mut sorted_keys: Vec<_> = tracker.history.keys().collect();
    sorted_keys.sort();

    // for line in reboot_lines {
    //     for var in sorted_keys.clone() {
    //         for m in &tracker.history[var] {
    //             if m.line = line
    //         }
    //     }
    // }

    // let mut sorted_keys: Vec<_> = tracker.history.keys().collect();
    // sorted_keys.sort();

    // if sorted_keys.is_empty() {
    //     return to_ret;
    // }

    // for var in sorted_keys {
    //     let mods = &tracker.history[var];
        

    //     // for m in mods {
    //     //     to_ret.get_mut(var).unwrap().push(value);
    //     //     //.((Some(), m.line));
            
    //     // }
    // }

    to_ret
}
// HashMap<String, Vec<(Option<i32>, usize)>>
pub fn run(filename: &String, every_line: bool) -> Vec<usize> {
    // get filename 
    // let args: Vec<String> = env::args().collect();
    // let filename = &args[1];

    // read the file
    let code = fs::read_to_string(filename).expect("Could not read file");

    // extract NIDS from header annotation
    let nids = parse_nids_header(&code);
    println!("Found NIDs: {:?}", nids);

    if nids.is_empty() {
        println!("No NIDs found");
        return vec![];
    }

    // parse AST
    let syntax_tree: File = syn::parse_str(&code).expect("Unable to parse code");

    let mut collector = FunctionCollector {
        functions: HashMap::new(),
    };
    collector.visit_file(&syntax_tree);

    // traverse AST with respect to NIDs
    let mut tracker = VariableTracker::new(nids, collector);



    tracker.visit_file(&syntax_tree);
    // println!("{:?}", tracker);

    let mut sorted_keys: Vec<_> = tracker.history.keys().collect();
    sorted_keys.sort();

    if sorted_keys.is_empty() {
        println!("No modifications found for NIDs.");
    }

    let mut num_of_reboots = 0;
    let mut reboot_lines = vec![];

    for var in sorted_keys {
        let mods = &tracker.history[var];
        println!("Variable: '{}'", var);
        print!("  Chain: x");
        for m in mods {
            print!(" -> f_{}(x)", m.line);
            reboot_lines.push(m.line);
        }

        println!("\n  Details:");
        
        for m in mods {
            println!("    Line {:<3} | Transformation: {}", m.line, m.operation);
        }
        println!();
        num_of_reboots += mods.len(); // updating number of reboots using mod chain
    }

    // get the number of reboots that need to be tracked
    if every_line {
        num_of_reboots = count_lines(filename); // if flag is set, overwrite
        //TODO: update this to be per each atomic region... oh my
    } 
    reboot_lines
    // build_value_structure(&tracker, num_of_reboots, reboot_lines)
}