use rand::Rng;

fn main() {
    let mut questions: Vec<(u16, u16)> = Vec::new();
    let mut rng = rand::thread_rng();
    while questions.len() < 22 {
        let a: u16 = rng.gen_range(1..=9);
        let b: u16 = rng.gen_range(1..=9);
        if a + b <= 10 {
            let mut is_new = true;
            for &(ea, eb) in &questions {
                if ea == a && eb == b {
                    is_new = false;
                }
            }
            if is_new {
                questions.push((a, b));
            }
        }
    }
    println!("Easy Questions:");
    for (index, &(a, b)) in questions.iter().enumerate() {
        if (index + 1) % 4 == 0 {
            println!("{} + {} = ", a, b)
        } else {
            print!("{} + {} = \t\t", a, b)
        }
    }

    let mut difficult_questions: Vec<(u16, u16)> = Vec::new();
    let mut rng = rand::thread_rng();
    while difficult_questions.len() < 22 {
        let a: u16 = rng.gen_range(2..=9);
        let b: u16 = rng.gen_range(2..=9);
        if a + b <= 10 {
            let mut is_new = true;
            for &(ea, eb) in &difficult_questions {
                if ea == a && eb == b {
                    is_new = false;
                }
            }
            if is_new {
                difficult_questions.push((a, b));
            }
        }
    }
    println!("\n\nDifficult Questions:");
    for (index, (a, b)) in difficult_questions.iter().enumerate() {
        if (index + 1) % 4 == 0 {
            println!("{} + {} = ", a, b);
        } else {
            print!("{} + {} = \t\t", a, b);
        }
    }
    println!("");
}
