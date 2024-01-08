#![allow(dead_code)]

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Func {
    //unary operators
    Sin,    // sin(f(x))
    Cos,    // cos(f(x))
    Tg,     // tg(f(x))
    Ctg,    // ctg(f(x))
    Ln,     // ln(f(x))
    Exp,    // e^(f(x))
    Sqrt,   // sqrt(f(x))
    Atg,  // arctg(f(x))
    Actg, // arcctg(f(x))
    Asin, // arcsin(f(x))
    Acos, // arccos(f(x))

    //These are the binary operation +, -, *, /
    Add,
    Sub,
    Mul,
    Div,
    Pow,    // g(x)^(f(x))

    //brackets
    Ob, //opne bracket
    Cb, //closed bracket

    //auxilary
    X,      //function variable
    Const,  // C, CeR
    None,   // end of the tree
}

impl ToString for Func {
    fn to_string(&self) -> String {
        match self{
            Func::Add => String::from("+"),
            Func::Sub => String::from("-"),
            Func::Mul => String::from("*"),
            Func::Div => String::from("/"),
            Func::Pow => String::from("^"),
            Func::X => String::from("x"),
            Func::None => String::from("None"),
            Func::Sin => String::from("sin"),
            Func::Cos => String::from("cos"),
            Func::Tg => String::from("tg"),
            Func::Ctg => String::from("ctg"),
            Func::Ln => String::from("ln"),
            Func::Exp => String::from("e^"),
            Func::Sqrt => String::from("sqrt"),
            Func::Const => String::from("Const"),
            Func::Atg => String::from("arctg"),
            Func::Asin => String::from("arcsin"),
            Func::Acos => String::from("arccos"),
            Func::Actg => String::from("arcctg"),
            Func::Ob => String::from("("),
            Func::Cb => String::from(")")
        }
    }
}

#[derive(Debug,Clone)]
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

    pub fn new_value(fun: Func, con: Option<f64>) -> Node {
        Node {
            first: None,
            second: None,
            op: fun,
            c: con,
        }
    }

    pub fn print_value(&self){
        if self.op == Func::Const {
            print!("{:?} ", match self.c {
                Some(x) => {x}
                None => {1.0}
            });
        }else{
            print!("{:?} ", self.op);
        }        
    }

    pub fn return_copy(&self) -> Node {
        Node {
            first: self.first.clone(),
            second: self.second.clone(),
            op: self.op,
            c: self.c,
        }
    }

    //IMPLEMENT COPY TRAIT HERE
}

pub struct Subseq {
    code: String, //complete code generated by subsequent branches
    raddr: i16,  //adress on which the result of the branches is on
}

impl Subseq {
    pub fn new(code: String, raddr: i16) -> Subseq{
        Subseq { code: code, raddr: raddr }
    }

    pub fn return_code(self) -> String {
        return self.code;
    }
}
