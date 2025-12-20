use quote::ToTokens;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use syn::spanned::Spanned;
use syn::{visit::Visit, BinOp, Expr, ExprAssign, ExprBinary, File, UnOp, Local, Pat};

// value, line number
type ValueLineTuple = (i32, i32);

// Visitor that tracks variables
struct VariableTracker {
    watched_vars: HashSet<String>, // list of NIDs
    instantiations: HashMap<String, ValueLineTuple>,
    nid_track: HashMap<String, Vec<ValueLineTuple>>,
}

impl VariableTracker {
    fn new(targets: Vec<String>) -> Self {
        VariableTracker {
            watched_vars: targets.into_iter().collect(),
            instantiations: HashMap::new(),
            nid_track: HashMap::new(),
        }
    }
}

// try to parse values to i32s 
fn parse_value(val_str: &str) -> i32 {
    val_str.trim().parse::<i32>().unwrap_or(0)
}

// helper for variable dereferences
fn get_var_name(expr: &Expr) -> Option<String> {
    match expr {
        // direct variable access
        Expr::Path(path) => {
            path.path.get_ident().map(|id| id.to_string())
        }
        // access via dereference
        Expr::Unary(unary) => {
            if let UnOp::Deref(_) = unary.op {
                get_var_name(&unary.expr)
            } else {
                None
            }
        }
        // access via parens
        Expr::Paren(paren) => {
            get_var_name(&paren.expr)
        }
        _ => None,
    }
}

// crappy pattern parser for lets
fn get_pat_name(pat: &Pat) -> Option<String> {
    if let Pat::Ident(pat_ident) = pat {
        return Some(pat_ident.ident.to_string());
    }
    None
}

// implement Visit trait
impl<'ast> Visit<'ast> for VariableTracker {
    // handles instatiations
    fn visit_local(&mut self, node: &'ast Local) {
        if let Some(var_name) = get_pat_name(&node.pat) {
            if self.watched_vars.contains(&var_name) {
                if let Some(init) = &node.init {
                    let rhs_str = init.expr.to_token_stream().to_string();
                    let rhs_val = parse_value(&rhs_str);
                    let line = node.span().start().line as i32;

                    self.instantiations.insert(var_name, (rhs_val, line));
                }
            }
        }
        syn::visit::visit_local(self, node);
    }
    
    // handles assignments i.e. x = 2
    fn visit_expr_assign(&mut self, node: &'ast ExprAssign) {
        if let Some(var_name) = get_var_name(&node.left) {
            // filter by NIDs
            if self.watched_vars.contains(&var_name) {
                let rhs_str = node.right.to_token_stream().to_string();
                let rhs_val = parse_value(&rhs_str);
                let line = node.span().start().line as i32;
                
                self.nid_track
                    .entry(var_name)
                    .or_default()
                    .push((rhs_val, line));
            }
        }
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
            if let Some(var_name) = get_var_name(&node.left) {
                // only process if current var is NID
                if self.watched_vars.contains(&var_name) {
                    let rhs_str = node.right.to_token_stream().to_string();
                    let rhs_val = parse_value(&rhs_str); 
                    let line = node.span().start().line as i32;

                    self.nid_track
                        .entry(var_name)
                        .or_default()
                        .push((rhs_val, line));
                }
            }
        }
        syn::visit::visit_expr_binary(self, node);
    }
}

// parse NIDs from user-given annotation at the top of a program
fn parse_nids_header(code: &str) -> Vec<String> {
    for line in code.lines() {
        let trimmed = line.trim();
        // check for the specific prefix
        if let Some(content) = trimmed.strip_prefix("#[nids(") {
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

fn main() {
    // get filename 
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    // read the file
    let code = fs::read_to_string(filename).expect("Could not read file");

    // extract NIDS from header annotation
    let nids = parse_nids_header(&code);
    println!("Found NIDs: {:?}", nids);

    if nids.is_empty() {
        println!("No NIDs found");
        return;
    }

    // parse AST
    let ast: File = syn::parse_str(&code).expect("Unable to parse code");

    // traverse AST with respect to NIDs
    let mut tracker = VariableTracker::new(nids);
    tracker.visit_file(&ast);

    let mut all_vars: Vec<_> = tracker.watched_vars.iter().collect();
    all_vars.sort();

    println!("nid_track");
    for var in all_vars {
        let mut all_mods = Vec::new();
        
        if let Some(inst) = tracker.instantiations.get(var) {
            all_mods.push(*inst);
        }

        if let Some(nid_track) = tracker.nid_track.get(var) {
            all_mods.extend(nid_track);
        }

        if !all_mods.is_empty() {
            println!("Nid: '{}'", var);
            print!("  [");
            for (i, (val, line)) in all_mods.iter().enumerate() {
                if i > 0 { print!(", "); }
                print!("({}, {})", val, line);
            }
            println!("]\n");
        }
    }
}