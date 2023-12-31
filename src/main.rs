use std::{collections::HashMap, io::Write};

fn main() {
    let seperator = "-------------------------------------------------";
    let mut want = String::new();
    print!("Input: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut want).unwrap();

    let want = want.trim().chars().map(|c| c as u8).collect::<Vec<_>>();

    for i in 0..=255u8 {
        let mut msg = HashMap::new();
        for j in 0..=255u8 {
            let res = i ^ j;
            if want.contains(&res) {
                msg.insert(res as char, format!(r#""{}" ^ "{}""#, i as char, j as char));
            }
        }
        for c in want.iter() {
            let msg = if msg.contains_key(&(*c as char)) {
                format!("  {} :   {}", (*c as char), msg[&(*c as char)])
            } else {
                format!("  {} :   {}", (*c as char), "Not found")
            };

            println!("{:^width$}", msg, width = seperator.len());
        }
        println!("{}", seperator);
    }
}
