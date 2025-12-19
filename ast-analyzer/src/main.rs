use quote::ToTokens;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use syn::spanned::Spanned;
use syn::{visit::Visit, BinOp, Expr, ExprAssign, ExprBinary, File, UnOp};

#[derive(Debug)]

// struct to hold single modification event
struct Modification {
    line: usize,
    operation: String, // function like f_n(x)
}

// Visitor that tracks variables
struct VariableTracker {
    watched_vars: HashSet<String>, // list of NIDs
    history: HashMap<String, Vec<Modification>>, // maps variable name to list of its modifications
}

impl VariableTracker {
    fn new(targets: Vec<String>) -> Self {
        VariableTracker {
            watched_vars: targets.into_iter().collect(),
            history: HashMap::new(),
        }
    }
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

// implement Visit trait
impl<'ast> Visit<'ast> for VariableTracker {
    // handles assignments i.e. x = 2
    fn visit_expr_assign(&mut self, node: &'ast ExprAssign) {
        if let Some(var_name) = get_var_name(&node.left) {
            // filter by NIDs
            if self.watched_vars.contains(&var_name) {
                let rhs = node.right.to_token_stream().to_string();
                let line = node.span().start().line;
                
                let lhs_code = node.left.to_token_stream().to_string();
                let func_repr = format!("{} = {}", lhs_code, rhs);

                self.history.entry(var_name).or_default().push(Modification {
                    line,
                    operation: func_repr,
                });
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
                    let op = node.op.to_token_stream().to_string();
                    let rhs = node.right.to_token_stream().to_string();
                    let line = node.span().start().line;

                    let lhs_code = node.left.to_token_stream().to_string();
                    let math_op = op.trim_end_matches('=');
                    let func_repr = format!("{} = {} {} ({})", lhs_code, lhs_code, math_op, rhs);

                    self.history.entry(var_name).or_default().push(Modification {
                        line,
                        operation: func_repr,
                    });
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
    let syntax_tree: File = syn::parse_str(&code).expect("Unable to parse code");

    // traverse AST with respect to NIDs
    let mut tracker = VariableTracker::new(nids);
    tracker.visit_file(&syntax_tree);

    let mut sorted_keys: Vec<_> = tracker.history.keys().collect();
    sorted_keys.sort();

    if sorted_keys.is_empty() {
        println!("No modifications found for NIDs.");
    }

    for var in sorted_keys {
        let mods = &tracker.history[var];
        println!("Variable: '{}'", var);
        print!("  Chain: x");
        for m in mods {
            print!(" -> f_{}(x)", m.line);
        }
        println!("\n  Details:");
        
        for m in mods {
            println!("    Line {:<3} | Transformation: {}", m.line, m.operation);
        }
        println!();
    }
}