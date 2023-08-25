use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

pub fn input_loop<F>(f: F)
where
    F: Fn(&str),
{
    println!("Press Ctrl-C to exit.");
    let mut rl = DefaultEditor::new().unwrap();
    loop {
        let readline = rl.readline("Word to parse: ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str()).unwrap();
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
