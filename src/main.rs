mod auto;


fn input(prompt: &str) -> String {
    use std::io::Write;
    print!("{}", prompt);
    std::io::stdout().flush().unwrap();
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.pop();
    buf.pop();
    buf
}


fn main() {
    loop {
        let root = auto::VerbRoot::from_inf(&input("pick a verb any verb> "));
        println!("present simple:        {:?}", root.aorist());
        println!("present continuous:    {:?}", root.continuous());
        println!("past definite:         {:?}", root.definite_past());
        println!("past reported:         {:?}", root.reported_past());
        println!("future:                {:?}", root.future());
        println!();
        println!("present simple /n:     {:?}", root.neg_aorist());
        println!("present continuous /n: {:?}", root.neg_continuous());
        println!("past definite /n:      {:?}", root.neg_definite_past());
        println!("past reported /n:      {:?}", root.neg_reported_past());
        println!("future /n:             {:?}", root.neg_future());
    }
}

