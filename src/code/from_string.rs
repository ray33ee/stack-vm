use std::fmt;
use ::{Code, InstructionTable};
use Builder;

impl<'a, T: fmt::Debug + PartialEq + From<& 'a str>> Code<T> {
    fn parse<D>(s: & 'a str, table: &InstructionTable<T, D>) -> Self {
        let mut builder = Builder::new(table);

        for line in s.split('\n') {
            //Get all non-comment content
            let mut line = line.split('#');
            let line = line.next().unwrap();

            if line.contains(':') {

                let mut split = line.split(':');
                let pre = split.next().unwrap();

                let mut label = pre.split_whitespace();
                let fin = label.next().unwrap();

                builder.label(fin);

            } else {
                let tokens: Vec<_> = line.split_whitespace().collect();

                //If the line is just a comment, just whitespace, or completely empty
                if tokens.is_empty() {
                    continue
                }

                let operation = table.by_name(tokens[0]).unwrap();


                let args = {
                    let mut args = Vec::new();
                    for i in 0..operation.arity {
                        args.push(T::from(tokens[i+1]));
                    }
                    args
                };

                builder.push(operation.name.as_str(), args);


            }



        }

        Code::from(builder)
    }
}


mod test {
    use std::fmt::{Debug, Formatter};
    use std::num::ParseIntError;
    use ::{Code, Machine};
    use ::{Instruction, InstructionTable};

    #[derive(PartialEq)]
    enum Operand {
        I(i32),
        S(String),
    }

    impl Debug for Operand {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self {
                Operand::I(i) => {
                    write!(f, "{}", i)
                }
                Operand::S(s) => {
                    write!(f, "{}", s)
                }
            }
        }
    }

    impl From<&str> for Operand {
        fn from(s: &str) -> Self {
            match s.parse::<i32>() {
                Ok(i) => {Operand::I(i)}
                Err(_) => {Operand::S(String::from(s))}
            }
        }
    }

    fn push(machine: &mut Machine<Operand, u32>, args: &[usize]) {
    }

    fn add(machine: &mut Machine<Operand, u32>, _args: &[usize]) {
    }

    fn sub(machine: &mut Machine<Operand, u32>, _args: &[usize]) {
    }

    #[test]
    fn test() {

        let mut table = InstructionTable::new();

        table.insert(Instruction::new(0, "push", 1, push));
        table.insert(Instruction::new(1, "add", 0, push));
        table.insert(Instruction::new(2, "sub", 0, push));

        let s = "
	push 33
	push hello_string
	add
";


        let code = Code::<Operand>::parse(s, &table);

        println!("{:?}", code);
    }
}