use regex::Regex;
use std::fmt;
use std::path::Path;
use ::{Code, InstructionTable};
use Builder;
use lazy_static::lazy_static;

impl<'a, T: fmt::Debug + PartialEq + From<& 'a str>> Code<T> {
    pub fn parse<D>(s: & 'a str, table: &InstructionTable<T, D>) -> Self {

        lazy_static!{

            static ref RE: Regex = Regex::new("[^\\s]*\"[^\"]*\"[^\\s]*").unwrap();

        }


        let mut builder = Builder::new(table);



        for line in s.split('\n') {
            //Get all non-comment content
            let mut line = line.split('#');
            let line = line.next().unwrap();


            let tokens: Vec<&str> = {
                let mut last = 0;

                let mut res = Vec::new();

                for m in RE.find_iter(line) {

                    let pre= &line[last..m.start()];

                    for it in pre.split_whitespace() {
                        res.push(it);
                    }

                    res.push(m.as_str());
                    last = m.end();
                }

                for it in (&line[last..]).split_whitespace() {
                    res.push(it);
                }

                res
            };

            let tokens: Vec<_> = tokens.iter().filter(|str| !str.is_empty()).collect();

            //If the line is just a comment, just whitespace, or completely empty
            if tokens.is_empty() {
                continue
            }

            if tokens[0].as_bytes()[0] == '.' as u8 {
                let mut split = line.split(|c| c == ':' || c == '.').filter(|c| !c.is_empty());

                let pre = split.next().unwrap();

                let mut label = pre.split_whitespace();
                let fin = label.next().unwrap();

                builder.label(fin);
            } else {
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
.main:
	push 33
	push hello_world
	add
";


        let code = Code::<Operand>::parse(s, &table);

        println!("{:?}", code);

        assert_eq!(s, format!("{:?}", code))
    }
}