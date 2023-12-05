#![allow(dead_code)]
use std::process::exit;
use crate::parts::object_type_definitions::*;

use super::tree_to_ir::print_tree;

fn try_parsing(chunk: &str) -> Option<Node> {
    match chunk.len() {
        1 => {
            match &chunk[..]{
                "*" => {
                    return Some(Node::new_value(Func::Mul, None));
                }

                "/" => {
                    return Some(Node::new_value(Func::Div, None));
                }

                "+" => {
                    return Some(Node::new_value(Func::Add, None));
                }

                "-" => {
                    return Some(Node::new_value(Func::Sub, None));
                }

                "^" => {
                    return Some(Node::new_value(Func::Pow, None));
                }

                "x" => {
                    return Some(Node::new_value(Func::X, None));
                }

                "(" => {
                    return Some(Node::new_value(Func::Ob, None));
                }

                ")" => {
                    return Some(Node::new_value(Func::Cb, None));
                }

                _ => {
                    return None;
                }
            }
        }
        2 => {
            match &chunk[..]{
                "ln" => {
                    return Some(Node::new_value(Func::Ln, None));
                }

                "e^" => {
                    return Some(Node::new_value(Func::Exp, None));
                }

                "tg" => {
                    return Some(Node::new_value(Func::Tg, None));
                }

                _ => {
                    return None;
                }
            }
        }
        3 => {
            match &chunk[..]{
                "sin" => {
                    return Some(Node::new_value(Func::Sin, None));
                }

                "cos" => {
                    return Some(Node::new_value(Func::Cos, None));
                }

                "ctg" => {
                    return Some(Node::new_value(Func::Ctg, None));
                }

                "atg" => {
                    return Some(Node::new_value(Func::Atg, None));
                }
                
                _ => {
                    return None;
                }
            }
        }
        4 => {
            match &chunk[..]{
                "sqrt" => {
                    return Some(Node::new_value(Func::Sqrt, None));
                }

                "asin" => {
                    return Some(Node::new_value(Func::Asin, None));
                }

                "acos" => {
                    return Some(Node::new_value(Func::Acos, None));
                }

                "actg" => {
                    return Some(Node::new_value(Func::Actg, None));
                }

                _ => {
                    return None;
                }
            }
        }
        _ => {
           println!("ERROR\nCouldn't parse part of the function string: {}\nLenght: {}\nCheck if you made a typo.", chunk,chunk.len());
           exit(0);
        }
    }
}

pub fn string_to_vec_of_node(function: &str) -> Vec<Node> {
    let mut list: Vec<Node> = Vec::<Node>::new();

    let mut i: usize = 0;
    let mut buffer: usize = 1;

    while i+buffer<function.len()+1{
        let mut temp = i;

        while let Some(ch) = function.chars().nth(temp) {
            if ch.is_digit(10) || ch == '.' {
                buffer+=1;
                temp+=1;
                continue;
            }
            break;
        }

        let inter_node: Option<Node>;

        if i == temp{
            inter_node = try_parsing(&function[i..i+buffer]);
        }else {
            buffer-=1;
            inter_node = Some(Node::new_value(Func::Const, Some(
                match function[i..i + buffer].parse::<f64>() {
                    Ok(c) => {c}
                    Err(_c) => {
                        println!("\nFailed to parse a string to number, string in question:\n{}", &function[i..i + buffer]);
                        exit(0);
                    }
                }
            )));
        }

        match inter_node {
            Some(list_node) => {
                i += buffer;
                buffer = 1;
                list.push(list_node);
            }
            None => {
                buffer+=1;
            }
        }
    }

    list
}

fn in_op_priority(op: &Node) -> u8{
    match op.op {
        Func::Add | Func::Sub=> {
            return 2;
        }

        Func::Mul | Func::Div => {
            return 3;
        }
        
        Func::Pow => {
            return 5;
        }

        Func::Ob => {
            return 9;
        }

        Func::Cb => {
            return 1;
        }

        Func::Const | Func::X=> {
            return 11;
        }

        //all other functions that behave as unary operators in this stack coversion
        _ => {
            return 8;
        }
    }
}

fn st_op_priority(op: &Node) -> u8{
    match op.op {
        Func::Add | Func::Sub=> {
            return 2;
        }

        Func::Mul | Func::Div => {
            return 3;
        }
        
        Func::Pow => {
            return 4;
        }

        Func::Ob => {
            return 0;
        }

        Func::Cb => {
            return 0;
        }

        _ => {
            return 7;
        }
    }
}

pub fn vec_node_to_string(ve: &Vec<Node>) -> String{
    let mut helper_string = String::new();

    for x in ve {
        helper_string += &(x.op.to_string() + " ");
    }
    helper_string += "\n";
    helper_string
}

//Finnih this function, rest it easy https://view.officeapps.live.com/op/view.aspx?src=https%3A%2F%2Frti.etf.bg.ac.rs%2Frti%2Fri3sp%2Fmaterijali%2Fir2asp%2F04_StekRedovi.ppt&wdOrigin=BROWSELINK
pub fn vec_infix_to_postfix(infix: Vec<Node>) -> Vec<Node>{
    let mut postfix: Vec<Node> = Vec::<Node>::new();
    let mut stack: Vec<Node> = Vec::<Node>::new();

    let mut i: usize = 0;

    while i < infix.len() {
        if in_op_priority(&infix[i]) == 11 {
            postfix.push(infix[i].return_copy());
            i+=1;
            continue;
        }

        let mut stack_top: Option<Node> = match stack.pop() {
            Some(x) => {
                Some(x.return_copy())
            }
            None => {
                None
            }
        };

        loop {
            match stack_top {
                Some(x) => {
                    if in_op_priority(&infix[i]) < st_op_priority(&x) {                                                                                            
                        postfix.push(x.return_copy());
                        stack_top = stack.pop();          
                    }else{
                        stack.push(x.return_copy());
                        break;
                    }
                }
                None => {
                    break;
                }
            }
        }

        if infix[i].op != Func::Cb {
            stack.push(infix[i].return_copy());
        }else{            
            stack.pop();
        }        
        
        i+=1;
    }

    loop {
        let last_on_stack: Node = match stack.pop() {
            Some(nod) => {
                nod
            }

            None => {
                break;
            }
        };

        postfix.push(last_on_stack);
    }
    postfix
}

fn postfix_to_tree(list: &mut Vec<Node>) -> Node {
    //check if it's more efficient to format this differently
    let binary_ops = vec![Func::Add, Func::Sub, Func::Mul, Func::Div, Func::Pow];
    let unary_ops = vec![Func::Sin,Func::Cos,Func::Tg,Func::Ctg,Func::Ln,Func::Exp,Func::Sqrt,Func::Atg,Func::Actg,Func::Asin,Func::Acos];
    match list.len() {
        0 => {
            panic!("Tree can't be generated due to list having no elements");
        }

        1 => {
            return list[0].return_copy();
        }

        2 => {
            //check if this thing works proprely
            list[1].first = Some(Box::new(list[0].return_copy()));
            list.remove(0);
        }

        _ => {
            let mut i = 2;
            let mut prev1 = 1; 
            let mut prev2 = 0;

            if in_op_priority(&list[1]) == 8 {
                list[1].first = Some(Box::new(list[0].return_copy()));
                list.remove(0);
            }

            //check if this list can be folded between prev1 and prev2

            while list.len() != 1 && i>list.len(){
                if unary_ops.contains(&list[prev1].op){
                    list[prev1].first = Some(Box::new(list[prev2].return_copy()));
                    list.remove(prev2);

                    if prev2 != 0 {
                        prev2 -= 1;
                    }
                    continue;
                }

                if binary_ops.contains(&list[i].op) {
                    list[i].first = Some(Box::new(list[prev1].return_copy()));
                    list[i].second = Some(Box::new(list[prev2].return_copy()));
                    list.remove(prev1);
                    list.remove(prev2);
                }


                i+= 1;
                prev1+=1;
                prev2+=1;
            }
        }
    }

    print_tree(&list[0], 0, ' ');

    list[0].return_copy()
}

pub fn str_to_tree_iter(function: &str) -> Node{
    let mut list: Vec<Node> = vec_infix_to_postfix(string_to_vec_of_node(function));    
    let root = postfix_to_tree(&mut list);
    root
}