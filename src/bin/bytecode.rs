#![feature(test)]
extern crate test;

use rusthour::Interpreter::interpret;
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

#[cfg(test)]
mod tests {
    use rusthour::Interpreter::interpret;
    use test::Bencher;

    #[test]
    fn test_add() {
        use super::ByteCode;
        let mut bytcd = ByteCode::new();
        bytcd.push("LOAD_VAL 5".to_string());
        bytcd.push("LOAD_VAL 5".to_string());
        bytcd.push("ADD".to_string());
        assert_eq!(interpret::<i64>(&bytcd).unwrap(), 10);
    }
    #[test]
    fn test_mul() {
        use super::ByteCode;
        let mut bytcd = ByteCode::new();
        bytcd.push("LOAD_VAL 5".to_string());
        bytcd.push("LOAD_VAL 7".to_string());
        bytcd.push("WRITE_VAR 'x'".to_string());
        bytcd.push("READ_VAR 'x".to_string());
        bytcd.push("MUL".to_string());
        bytcd.push("WRITE_VAR 'r'".to_string());
        bytcd.push("LOAD_VAL 7".to_string());
        bytcd.push("WRITE_VAR 'y'".to_string());
        bytcd.push("READ_VAR 'r".to_string());
        bytcd.push("READ_VAR 'y'".to_string());
        bytcd.push("MUL".to_string());
        assert_eq!(interpret::<i64>(&bytcd).unwrap(), 35 * 7);
    }
    #[test]
    fn test_sub() {
        use super::ByteCode;
        let mut bytcd = ByteCode::new();
        bytcd.push("LOAD_VAL 5".to_string());
        bytcd.push("WRITE_VAR 'x'".to_string());
        bytcd.push("LOAD_VAL 5".to_string());
        bytcd.push("WRITE_VAR 'y'".to_string());
        bytcd.push("READ_VAR 'x'".to_string());
        bytcd.push("READ_VAR 'y'".to_string());
        bytcd.push("SUB".to_string());
        assert_eq!(interpret::<i64>(&bytcd).unwrap(), 0);
    }
    #[test]
    fn test_div1() {
        use super::ByteCode;
        let mut bytcd = ByteCode::new();
        bytcd.push("LOAD_VAL 5".to_string());
        bytcd.push("WRITE_VAR 'x'".to_string());
        bytcd.push("LOAD_VAL 5".to_string());
        bytcd.push("WRITE_VAR 'y'".to_string());
        bytcd.push("READ_VAR 'x'".to_string());
        bytcd.push("READ_VAR 'y'".to_string());
        bytcd.push("DIV".to_string());
        assert_eq!(interpret::<i64>(&bytcd).unwrap(), 1);
    }
    #[test]
    fn test_div2() {
        use super::ByteCode;
        let mut bytcd = ByteCode::new();
        bytcd.push("LOAD_VAL 5".to_string());
        bytcd.push("WRITE_VAR 'x'".to_string());
        bytcd.push("LOAD_VAL 0".to_string());
        bytcd.push("WRITE_VAR 'y'".to_string());
        bytcd.push("READ_VAR 'x'".to_string());
        bytcd.push("READ_VAR 'y'".to_string());
        bytcd.push("DIV".to_string());
        assert_eq!(
            interpret::<i64>(&bytcd),
            Err("Division by zero error!".to_owned())
        );
    }
    #[test]
    fn test_return() {
        use super::ByteCode;
        let mut bytcd = ByteCode::new();
        bytcd.push("LOAD_VAL 5".to_string());
        bytcd.push("WRITE_VAR 'x'".to_string());
        bytcd.push("RETURN_VAL 'x'".to_string());
        assert_eq!(interpret::<i64>(&bytcd), Ok(5));
    }

    #[bench]

    fn bench_return(b: &mut Bencher) {
        use super::ByteCode;
        let mut bytcd = ByteCode::new();
        bytcd.push("LOAD_VAL 5".to_string());
        bytcd.push("WRITE_VAR 'x'".to_string());
        bytcd.push("RETURN_VAL 'x'".to_string());
        b.iter(|| interpret::<i64>(&bytcd))
    }
    #[bench]
    fn bench_add(b: &mut Bencher) {
        use super::ByteCode;
        let mut bytcd = ByteCode::new();
        bytcd.push("LOAD_VAL 5".to_string());
        bytcd.push("WRITE_VAR 'x'".to_string());
        bytcd.push("LOAD_VAL 0".to_string());
        bytcd.push("WRITE_VAR 'y'".to_string());
        bytcd.push("READ_VAR 'x'".to_string());
        bytcd.push("READ_VAR 'y'".to_string());
        bytcd.push("ADD".to_string());
        b.iter(|| interpret::<i64>(&bytcd))
    }
    #[bench]
    fn bench_mul(b: &mut Bencher) {
        use super::ByteCode;
        let mut bytcd = ByteCode::new();
        bytcd.push("LOAD_VAL 5".to_string());
        bytcd.push("LOAD_VAL 7".to_string());
        bytcd.push("WRITE_VAR 'x'".to_string());
        bytcd.push("READ_VAR 'x".to_string());
        bytcd.push("MUL".to_string());
        bytcd.push("LOAD_VAL 5".to_string());
        bytcd.push("LOAD_VAL 7".to_string());
        bytcd.push("WRITE_VAR 'x'".to_string());
        bytcd.push("READ_VAR 'x".to_string());
        bytcd.push("MUL".to_string());
        b.iter(|| interpret::<i64>(&bytcd))
    }
    #[bench]
    fn bench_div(b: &mut Bencher) {
        use super::ByteCode;
        let mut bytcd = ByteCode::new();
        bytcd.push("LOAD_VAL 5".to_string());
        bytcd.push("WRITE_VAR 'x'".to_string());
        bytcd.push("LOAD_VAL 5".to_string());
        bytcd.push("WRITE_VAR 'y'".to_string());
        bytcd.push("READ_VAR 'x'".to_string());
        bytcd.push("READ_VAR 'y'".to_string());
        bytcd.push("DIV".to_string());
        b.iter(|| interpret::<i64>(&bytcd))
    }
}
