use std::collections::HashMap;
use std::io;

type Db = HashMap<String, Vec<String>>;

enum Command<'a> {
    Add {
        name: String,
        department: String,
        db: &'a mut Db,
    },
    ListAll(&'a Db),
    List {
        dep: String,
        db: &'a Db,
    },
    Unknown,
}

impl Command<'_> {
    fn execute(self) {
        match self {
            Self::Add {
                name,
                department,
                db,
            } => {
                let dep = db.entry(department).or_insert(vec![]);
                dep.push(name);
                println!("executed");
            }
            Self::ListAll(db) => println!("{:?}", db),
            Self::List { dep, db } => match db.get(&dep) {
                Some(names) => println!("{:?}", names),
                _ => println!("Cant find this department"),
            },
            Self::Unknown => println!("Command unknown!"),
        }
    }
}

fn main() {
    let mut db: Db = HashMap::new();
    loop {
        println!("Enter a command");
        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read the line");
        //create a command from user unput
        let user_input_lc = user_input.to_lowercase();
        let keywords: Vec<_> = user_input_lc.trim().split_whitespace().collect();
        if keywords.len() < 1 {
            continue;
        }

        let command: Command;
        match keywords[0] {
            "add" => {
                if keywords.len() == 4 {
                    command = Command::Add {
                        name: keywords[1].to_string(),
                        department: keywords[3].to_string(),
                        db: &mut db,
                    }
                } else {
                    command = Command::Unknown;
                }
            }
            "list" => {
                if keywords.len() < 2 {
                    command = Command::ListAll(&db);
                } else {
                    command = Command::List {
                        dep: keywords[1].to_string(),
                        db: &db,
                    };
                }
            }
            "exit" => break,
            _ => command = Command::Unknown,
        }
        //function execute command
        command.execute();
    }
}
