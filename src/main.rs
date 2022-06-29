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
    for (a, b) in questions {
        println!("{} + {} = ", a, b);
    }
}
