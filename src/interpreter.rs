type ByteCode = Vec<String>;

pub enum OpCode {
    LOAD_VAL,
    WRITE_VAR,
    READ_VAR,
    ADD,
    SUB,
    MUL,
    DIV,
    LOOP,
    NULL,
}

impl OpCode {
    pub fn new(a: &String) -> Self {
        match a.as_str() {
            "LOAD_VAL" => OpCode::LOAD_VAL,
            "WRITE_VAR" => OpCode::WRITE_VAR,
            "READ_VAR" => OpCode::READ_VAR,
            "ADD" => OpCode::ADD,
            "SUB" => OpCode::SUB,
            "MUL" => OpCode::MUL,
            "DIV" => OpCode::DIV,
            "LOOP" => OpCode::LOOP,
            _ => OpCode::NULL,
        }
    }
}

pub mod Interpreter {
    use super::{ByteCode, OpCode};
    use std::collections::HashMap;

    pub fn interpret<
        T: std::str::FromStr
            + std::fmt::Debug
            + Clone
            + Copy
            + std::ops::Add<Output = T>
            + std::ops::Sub<Output = T>
            + std::ops::Div<Output = T>
            + std::ops::Mul<Output = T>,
    >(
        bytecode: &ByteCode,
    ) -> Result<(), String>
    where
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        let mut var: HashMap<&str, T> = HashMap::new();
        let mut varnames: Vec<&str> = Vec::new();
        let mut loaded: Vec<T> = Vec::new();

        for code in bytecode {
            // println!("Elements:{:?}", var);
            let tmp: Vec<&str> = code.split(" ").collect();

            match OpCode::new(&tmp[0].to_string()) {
                OpCode::LOAD_VAL => loaded.push(tmp[1].parse().unwrap()),
                OpCode::WRITE_VAR => {
                    varnames.push(&tmp[1][1..2]);

                    var.insert(varnames.pop().unwrap(), loaded.pop().unwrap());
                }
                OpCode::READ_VAR => {
                    loaded.push(var[&tmp[1][1..2]]);
                }
                OpCode::ADD => {
                    println!("Performing Addition...");
                    let op2 = loaded.pop().unwrap();
                    let op1 = loaded.pop().unwrap();
                    let temp = op1 + op2;
                    loaded.push(temp);
                    println!("Result:{:?}", temp);
                }
                OpCode::SUB => {
                    println!("Performing Subtraction...");
                    let op2 = loaded.pop().unwrap();
                    let op1 = loaded.pop().unwrap();
                    let temp = op1 - op2;
                    loaded.push(temp);
                    println!("Result:{:?}", temp);
                }
                OpCode::DIV => {
                    println!("Performing Division...");
                    let op2 = loaded.pop().unwrap();
                    let op1 = loaded.pop().unwrap();
                    let temp = op1 / op2;
                    loaded.push(temp);
                    println!("Result:{:?}", temp);
                }
                OpCode::MUL => {
                    println!("Performing Multiplication...");
                    let op2 = loaded.pop().unwrap();
                    let op1 = loaded.pop().unwrap();
                    let temp = op1 * op2;
                    loaded.push(temp);
                    println!("Result:{:?}", temp);
                }
                OpCode::NULL => {
                    println!("Error in Bytecode");
                    return Err("Invalid bytecode".to_string());
                }
                OpCode::LOOP => {
                    println!("Entering a Loop..");
                }
            }
        }

        Ok(())
    }
}
