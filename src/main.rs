/// There has to be at least one file (block) in the mem folder -> 0.txt with "{}" inside (without quotes)

use blake2::{Blake2b, Digest};
use std::{fs, io, path::Path};
use std::io::{Write, Read};
use json::object;
use std::env;

fn get_registry_files(path: &Path) -> Result<Vec<u8>, io::Error> {
    if !path.exists() {
        fs::create_dir(path)?;
    }
    let mut v = path
        .read_dir()?
        .filter_map(|e| {
            e.ok().and_then(|el| {
                el.file_name()
                    .to_str()
                    .and_then(|e| e.split('.').next().and_then(|e| e.parse::<u8>().ok()))
            })
        })
        .collect::<Vec<u8>>();
    v.sort();
    Ok(v)
}

fn get_data_from_registry(path: &Path, index: u8) -> io::Result<json::JsonValue> {
    let p = path.join(Path::new(&format!("{}.txt", index)));
    let mut f = fs::File::open(p)?;
    let mut b = String::new();
    f.read_to_string(&mut b)?;

    match json::parse(&b) {
        Ok(e) => Ok(e),
        Err(e) => Err(io::Error::new(io::ErrorKind::InvalidInput, e.to_string()))
    }
}

fn get_hash_from_registry(path: &Path, index: u8) -> io::Result<String> {
    let p = path.join(Path::new(&format!("{}.txt", index)));
    let mut f = fs::File::open(p)?;
    let mut hasher = Blake2b::new();

    io::copy(&mut f, &mut hasher)?;

    let hash = hasher.result();
    Ok(format!("{:x}", hash))
}

fn save_to_registry(path: &Path, index: u8, data: &str) -> io::Result<()> {
    let p = path.join(Path::new(&format!("{}.txt", index)));
    let mut f = fs::File::create(p)?;
    f.write_all(data.as_bytes())?;
    Ok(())
}

fn get_and_save_to_registry(path: &Path, registry: &mut Vec<u8>) -> io::Result<()> {
    let mut stdout = io::stdout();
    let mut stdin = io::stdin();
    let mut resp = String::new();

    print!("Write message to save: ");
    stdout.flush();
    stdin.read_line(&mut resp)?;

    let prev_hash = get_hash_from_registry(path, registry[registry.len() - 1])?;

    resp.pop();

    let mut data = object! {
        "message" => resp,
        "prev_hash" => prev_hash
    };

    save_to_registry(path, registry[registry.len() - 1] + 1, &data.dump())?;

    registry.push(registry[registry.len() - 1] + 1);

    Ok(())
}

fn check_registry(path: &Path, index: u8) -> io::Result<bool> {
    Ok(get_data_from_registry(path, index + 1)?["prev_hash"] == get_hash_from_registry(path, index)?)
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let p = Path::new("mem/");

    if let Ok(mut registry) = get_registry_files(p) {
        if args.len() > 1 && args[1] == "check" {
            println!("{} -> tail", registry[registry.len()-1]);  //- Tail is always OK
            for i in registry.iter().rev().skip(1) {
                let state = check_registry(p, *i).expect("Is not able to check the registry!");
                println!("{} -> {:?}", i, state);
                if !state {
                    break;
                }
            }
        } else {
            loop {
                match get_and_save_to_registry(p, &mut registry) {
                    Ok(_) => println!("Saved."),
                    Err(_) => println!("Got error when trying to save to the registry."),
                }
            }
        }
    } else {
        println!("Could not get the registry files");
    }
}
