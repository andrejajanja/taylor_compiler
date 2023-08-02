use std::{io::stdin, process::exit};

// #[macro_export()]
// macro_rules! measure_time {
//     ($code:block) => {{
//         let start = Instant::now();
//         let result = $code;
//         let duration = start.elapsed();

//         println!("Time spent: {:?}", duration);
//         result
//     }};
// }

// fn _integral(a: f64, b: f64, steps: i64, fun: fn(f64) -> f64) -> f64 {
//     if a > b {
//         panic!("a value can't be bigger than b, see --help for instructions");
//     };
//     if a == b {
//         return 0.0;
//     };
//     let mut s: f64 = 0.0;
//     let dx: f64 = (b - a) / (steps as f64);
//     for i in (0..steps).rev() {
//         s += fun(a + (i as f64) * dx) * dx
//     }
//     s
// }

// fn _fun(x: f64) -> f64 {
//     x.cos()
// }

pub fn print_help() {
    println!("
    \t\tIntegral calculator user manual\n\n
    Options:
    --help -> prints this message\n

    Example call:\n
    integral_aproximator <- call an executable

    Input the function in the shape of:  sin(x)*e^(x+7)-tg(x)/ln(x-9)
    Set parameters:  0.0 1.0 1000
                      |   |   |
    range start ------^   |   |
    range end ------------^   |
    number of steps ----------^   

    ^^ This call aproximates an integral on a range from 0.0 to 1.0 with 1000 steps of approximation\n

    range start -> where the range starts (number, integer or a real number)
    range end -> where the range ends (number, integer or a real number)
    number of steps -> integrer of steps on which the intgeral of a function is aproximated\n\t(recommended value is between 10^4 and 10^7)
    
    Made by Andreja Janković; Year 2023; E-mail: me@andreja.dev\n\n");
}

pub fn parse_inputs(function: &mut String, start: &mut f64, end: &mut f64, steps: &mut u64) {
    println!("f(x) = ");
    stdin().read_line(function).unwrap_or_else(|err| {
        println!("\n\nError while taking a function input: {err}\n\n");
        exit(0);
    });
    function.pop(); //remove the newline character

    let mut parameters: String = String::new();
    print!("\nrange start, range end, step count: \n");
    stdin().read_line(&mut parameters).unwrap_or_else(|err| {
        println!("\n\nError while taking a parameter input: {err}\n\n");
        exit(0);
    });

    parameters.pop();
    let provided: Vec<&str> = parameters.split(" ").collect();

    if provided.len() != 3 {
        println!("Parameters entered in insufficient format, please try again.");
        exit(0);
    }

    *start = match provided[0].parse::<f64>() {
        Ok(x) => x,
        Err(e) => {
            println!(
                "\n\nError parsing range start argument: {e}\nthis is the value passed: '{}'\n\n",
                provided[0]
            );
            exit(0);
        }
    };

    *end = match provided[1].parse::<f64>() {
        Ok(x) => x,
        Err(e) => {
            println!(
                "\n\nError parsing range end argument: {e}\nthis is the value passed: '{}'\n\n",
                provided[1]
            );
            exit(0);
        }
    };

    *steps = match provided[2].parse::<u64>() {
        Ok(num) => num,
        Err(e) => {
            println!("\n\nError parsing number of steps argument: {e}\nthis is the value passed: '{}'\n\n", provided[2]);
            exit(0);
        }
    };
}

#[derive(Debug, Copy, Clone)]
pub enum Func {
    Sin,    // sin(f(x))
    Cos,    // cos(f(x))
    Tg,     // tg(f(x))
    Ctg,    // ctg(f(x))
    Ln,     // ln(f(x))
    Exp,    // e^(f(x))
    Pow,    // C^(f(x)) CeR
    Sqrt,   // sqrt(f(x))
    Const,  // C where CeR
    Arctg,  // arctg(f(x))
    Arcctg, // arcctg(f(x))
    Arcsin, // arcsin(f(x))
    Arccos, // arccos(f(x))
    //These are the operation +, -, *, /
    Add,
    Sub,
    Mul,
    Div,
    X, //if node is just x
    None,
}

#[derive(Debug)]
pub struct Node {
    pub first: Option<Box<Node>>,
    pub second: Option<Box<Node>>,
    pub op: Func,
    pub c: Option<f64>, //if type op = Func::Const
}

impl Node {
    pub fn new() -> Node {
        Node {
            first: None,
            second: None,
            op: Func::None,
            c: None,
        }
    }
}

fn split_by_ops(
    function: &String,
    op1: char,
    op2: char,
    mk1: Func,
    mk2: Func,
) -> (Vec<String>, Vec<Func>) {
    let mut tier_chunks = Vec::<String>::new();
    let mut tier_ops = Vec::<Func>::new();

    let mut depth: u8 = 0;
    let mut first: i16 = -1;
    let mut second: usize = 0;
    for (i, c) in function.chars().enumerate() {
        if depth != 0 {
            if c == '(' {
                depth += 1;
            }
            if c == ')' {
                depth -= 1;
            }
            continue;
        } else {
            if c == '(' {
                depth += 1;
                continue;
            }
        }

        if c == op1 || c == op2 {
            if c == op1 {
                tier_ops.push(mk1);
            } else {
                tier_ops.push(mk2);
            }

            if second != 0 {
                first = second as i16;
            }
            second = i;
            tier_chunks.push(function[(first + 1) as usize..second].to_string())
        }
    }
    if second == 0 {
        tier_chunks.push(function.clone());
    } else {
        tier_chunks.push(function[second + 1..].to_string());
    }
    (tier_chunks, tier_ops)
}

fn generate_stairs(chunks: &[String], ops: &[Func]) -> Node {
    let mut node = Node::new();
    if chunks.len() == 1 {
        node.first = Some(Box::new(generate_tree_from_string(&chunks[0])));
    } else {
        node.op = ops[0];
        node.first = Some(Box::new(generate_tree_from_string(&chunks[0])));
        if ops.len() == 1 {
            node.second = Some(Box::new(generate_tree_from_string(&chunks[1])));
        } else {
            node.second = Some(Box::new(generate_stairs(&chunks[1..], &ops[1..])));
        }
    }
    node
}

pub fn generate_tree_from_string(function: &String) -> Node {
    let mut sub_node = Node::new();
    let (first_tier_chunks, first_tier_ops) =
        split_by_ops(function, '+', '-', Func::Add, Func::Sub);

    //if there are some first tier ops
    if first_tier_chunks.len() != 1 {
        //case if there are any first tier operation in the function string
        sub_node.op = first_tier_ops[0];
        sub_node.first = Some(Box::new(generate_tree_from_string(&first_tier_chunks[0])));
        if first_tier_chunks.len() == 2 {
            //there are just 2 elements of 1st tier ops, processing the other one manualy
            sub_node.second = Some(Box::new(generate_tree_from_string(&first_tier_chunks[1])));
        } else {
            //there are more than 2 elements of 1st tier ops, running a tree algorithm
            sub_node.second = Some(Box::new(generate_stairs(
                &first_tier_chunks[1..],
                &first_tier_ops[1..],
            )));
        }
    } else {
        //there aren't any first tier ops
        let (second_tier_chunks, second_tier_ops) =
            split_by_ops(&first_tier_chunks[0], '*', '/', Func::Mul, Func::Div);

        //if there are some second tier ops
        if second_tier_chunks.len() != 1 {
            sub_node.op = second_tier_ops[0];
            sub_node.first = Some(Box::new(generate_tree_from_string(&second_tier_chunks[0])));

            if second_tier_chunks.len() == 2 {
                sub_node.second = Some(Box::new(generate_tree_from_string(&second_tier_chunks[1])));
            } else {
                //there are more than 2 elements of 2nd tier ops, running a tree algorithm
                sub_node.second = Some(Box::new(generate_stairs(
                    &second_tier_chunks[1..],
                    &second_tier_ops[1..],
                )));
            }
        } else {
            //There aren't any 2nd tier ops, checking for the single ops
            sub_node.second = None;
            if function == "x" {
                sub_node.first = None;
                sub_node.op = Func::X;
                sub_node.c = None;
                return sub_node;
            }
            match function.parse::<f64>() {
                Ok(c) => {
                    sub_node.first = None;
                    sub_node.op = Func::Const;
                    sub_node.c = Some(c);
                    return sub_node;
                }
                Err(_c) => {}
            }
            //this chunk isn't x or a number, so it is a complex function
            //extracting a lower level of this chunk
            let mut start: usize = 0;
            let mut end: usize = 0;
            let mut depth: u8 = 0;
            for (i, c) in function.chars().enumerate() {
                if c == '(' {
                    if depth == 0 {
                        start = i;
                    }
                    depth += 1;
                }

                if c == ')' {
                    depth -= 1;
                    if depth == 0 {
                        end = i;
                    }
                }
            }
            let lower_level = &function[start + 1..end].to_string();

            //determening the function type
            sub_node.c = None;
            match &function[0..start] {
                "sin" => {
                    sub_node.op = Func::Sin;
                }
                "cos" => {
                    sub_node.op = Func::Cos;
                }
                "tg" => {
                    sub_node.op = Func::Tg;
                }
                "ctg" => {
                    sub_node.op = Func::Ctg;
                }
                "sqrt" => {
                    sub_node.op = Func::Sqrt;
                }
                "arctg" => {
                    sub_node.op = Func::Arctg;
                }
                "arcctg" => {
                    sub_node.op = Func::Arcctg;
                }
                "arcsin" => {
                    sub_node.op = Func::Arcsin;
                }
                "arccos" => {
                    sub_node.op = Func::Arccos;
                }
                "e^" => {
                    sub_node.op = Func::Exp;
                }
                "ln" => {
                    sub_node.op = Func::Ln;
                }
                _ => {
                    println!("\n\tError parsing the function part. Check for typos!\n\tExact part that caused this error: '{}'\n", first_tier_chunks[0]);
                    exit(0);
                }
            }
            //further processing lower level of this chunk
            sub_node.first = Some(Box::new(generate_tree_from_string(lower_level)));
        }
    }
    sub_node
}

pub fn print_tree(node: &Node, tab: usize, addition: char) {
    match &node.op {
        Func::Const => {
            print!("{}| {:?} |{}", "\t".repeat(tab), node.c, addition);
        }
        _ => {
            print!("{}| {:?} |{}", "\t".repeat(tab), node.op, addition);
        }
    }
    match &node.first {
        Some(no) => {
            match &node.second {
                None => {
                    print!("\n");
                }
                Some(_x) => {}
            }

            print_tree(&no, tab + 1, '\n');
        }
        None => {}
    }
    match &node.second {
        Some(no) => {
            print_tree(&no, tab + 1, '\n');
        }
        None => {}
    }
}

//fundamental functions and their IR code.

const SIN_IR: &str = "SINE IR REP";
const COS_IR: &str = "SINE IR REP";
const TG_IR: &str = "SINE IR REP";
const CTG_IR: &str = "SINE IR REP";


fn compile_tree(node: &Node) -> String {
    let rep: String = String::new(); //IR representation

    rep
}

pub fn generate_ir(node: &Node) -> String {
    let rep: String = compile_tree(node);

    let contents: String = format!(";IR Code generated by Bata compiler
    target triple = 'x86_64-pc-linux-gnu'
    target datalayout = 'e-m:e-i64:64-f80:128-n8:16:32:64-S128'
    {rep}
    
    ");
    contents
}
