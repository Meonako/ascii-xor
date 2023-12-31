use std::{collections::HashMap, io::Write};

fn main() {
    let seperator = "-------------------------------------------------";
    let mut want = String::new();
    print!("Input: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut want).unwrap();

    let want = want.trim().chars().map(|c| c as u8).collect::<Vec<_>>();
    
    let start = u8::MIN;
    let end = u8::MAX;

    let mut msg: HashMap<char, Vec<String>> = HashMap::new();

    for i in start..=end {
        for j in start..=end {
            let res = i ^ j;
            if want.contains(&res) {
                let vect = msg.get_mut(&(res as char));
                match vect {
                    Some(v) => {
                        v.push(format!("{:?} ^ {:?}", i as char, j as char));
                    }
                    None => {
                        msg.insert(res as char, vec![format!("{:?} ^ {:?}", i as char, j as char)]);
                    }
                }
            }
        }
    }

    let mut lock = std::io::stdout().lock();
    for c in want.iter() {
        let outvec = msg.get(&(*c as char));
        match outvec {
            Some(v) => {
                writeln!(lock, "{sep}\n{:^width$}\n{sep}", format!("Output to: {}", (*c as char)), sep = seperator, width = seperator.len()).unwrap();
                for s in v {
                    writeln!(lock, "{:^width$}", s, width = seperator.len()).unwrap();
                }
            }
            None => {
                writeln!(lock, "{:^width$} : {}", (*c as char), "Not found", width = seperator.len()).unwrap();
            },
        };
    }
}
