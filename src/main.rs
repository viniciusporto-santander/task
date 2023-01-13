use std::{env, fs, io};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage: {} <key> <input_file>", args[0]);
        return Ok(());
    }
    let key = args[1].as_bytes();
    let input_filename = &args[2];
    let input = fs::read(input_filename)?;

    let mut output: Vec<u8> = Vec::with_capacity(input.len());

    for (i, b) in input.iter().enumerate() {
        output.push(b ^ key[i % key.len()]);
    }

    let output_filename = format!("{}_encoded", input_filename);
    fs::write(output_filename, output)?;

    Ok(())
}
