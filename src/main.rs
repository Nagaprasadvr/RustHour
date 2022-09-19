mod interpreter;
use interpreter::Interpreter::interpret;


type ByteCode = Vec<String>;

fn main() {
    let mut bytecode: ByteCode = ByteCode::new();
    println!("Enter the lines of code :");
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).expect("err");
    let n = buf.trim().parse::<u64>().unwrap();
    println!("Enter the Bytecode without new lines and comments:");
    for _ in 0..n {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).expect("err");
        let b = buf.trim().to_string();
        bytecode.push(b);
    }
    println!("Reading Bytecode...");
    println!("Enter 1 for integer operations or 2 for floating point operations:");
    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice).expect("err");
    let choice = choice.trim().parse::<u8>().unwrap();
    if choice == 1 {
        interpret::<i64>(&bytecode).expect("err");
    } else {
        interpret::<f64>(&bytecode).expect("err");
    }

    // println!("ByteCode :{:?}", bytecode);
}
