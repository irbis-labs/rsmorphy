use rustyline::{error::ReadlineError, Editor};

pub fn input_loop<F>(f: F)
where
    F: Fn(&str),
{
    println!("Press Ctrl-C to exit.");
    let mut rl = Editor::<()>::new();
    loop {
        let readline = rl.readline("Word to parse: ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                f(line.trim());
            }
            Err(ReadlineError::Interrupted) => {
                println!("Ctrl-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("Ctrl-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}
