use rand::seq::SliceRandom;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;

struct Channel {
    fname: String, // folder name
    link: String,  // link to Youtube channel
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("channels.json")?;
    let mut contents = String::new();

    // parses file contents to vector of Channel structs
    file.read_to_string(&mut contents)?;
    let mut channels: Vec<Channel> = parse_string(contents);

    // Randomly shuffle vector containing channels
    let mut rng = rand::thread_rng();
    channels.shuffle(&mut rng);

    // Loop over once to print all the channels in order
    for channel in channels.iter() {
        println!("{} -> {}", channel.fname, channel.link);
    }

    // Loop over second time to start the processes sequentially.
    // Second youtube-dl command starts after first one is done, etc
    for channel in channels {
        new_process(channel);
    }

    Ok(())
}

fn parse_string(contents: String) -> Vec<Channel> {
    let mut channels: Vec<Channel> = vec![];

    // Split contents by newline
    for s in contents.split("\n") {
        let parsed = json::parse(&s);
        match parsed {
            Ok(parsed) => {
                let ch = Channel {
                    fname: parsed["fn"].to_string(),
                    link: parsed["link"].to_string(),
                };

                // Add to channels vector
                channels.push(ch);
            }
            Err(error) => {
                // If error is NOT Unexpected end of JSON then print the error
                if error.to_string() != "Unexpected end of JSON" {
                    println!("{:?}", error);
                }
            }
        };
    }

    channels
}

fn new_process(channel: Channel) {
    println!("Starting youtube-dl for: {}", channel.fname);
    let mut child = Command::new("youtube-dl")
        .arg("-f")
        .arg("best")
        .arg("-ciw")
        .arg("-o")
        .arg(format!("{}/%(title)s.%(ext)s", channel.fname))
        .arg("-v")
        .arg(channel.link)
        .spawn()
        .expect("failed to execute!");

    child.wait().expect("failed to wait on child");
    println!("Child has finished its execution!");
}
