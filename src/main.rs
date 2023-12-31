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

    let mut msg: HashMap<char, Vec<(char, char)>> = HashMap::new();

    for i in start..=end {
        for j in start..=end {
            let res = i ^ j;
            if want.contains(&res) {
                let vect = msg.get_mut(&(res as char));
                match vect {
                    Some(v) => {
                        v.push((i as char, j as char));
                    }
                    None => {
                        msg.insert(res as char, vec![(i as char, j as char)]);
                    }
                }
                // msg.insert(res as char, format!(r#""{}" ^ "{}""#, i as char, j as char));
            }
        }
    }

    let mut lock = std::io::stdout().lock();
    for c in want.iter() {
        let outvec = msg.get(&(*c as char));
        let output = match outvec {
            Some(v) => {
                let mut output = String::new();
                for (i, j) in v {
                    output.push_str(&format!("\n\t{:?} ^ {:?}", i, j));
                }
                format!("{} : {}", (*c as char), output)
            }
            None => format!("{} : Not found", (*c as char)),
        };

        writeln!(lock, "{:^width$}\n{}", output, seperator, width = seperator.len()).unwrap();
    }
}
