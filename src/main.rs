use std::io;
// use rand::Rng;
use std::time::Duration;
use std::thread;

const CORRELATIVE_STARTS:[&'static str; 5] = ["ki","ti","i","neni","ĉi"];
const CORRELATIVE_ENDS: [&'static str; 9] = ["a","al","am","om","el","es","o","u","e"];
const UI_DELAY: u64 = 20;

fn ui_delay(m: u64) {
    thread::sleep(Duration::from_millis(UI_DELAY * m))
}

fn main() {
    ui_delay(50);

    let mut start_input = String::new();

    println!("Welcome to the correlative quiz game!");
    ui_delay(10);
    println!("Bonvenon al la ludo de tabelvortoj!");
    ui_delay(10);
    println!("");
    ui_delay(30);
    println!("Type \"start\" to begin.");
    ui_delay(10);
    println!("Tajpu \"ek\" por komenci.");
    io::stdin().read_line(&mut start_input).expect("Could not read input");

    let mut correlatives = Vec::new();

    for c_start in CORRELATIVE_STARTS.iter() {
        for c_end in CORRELATIVE_ENDS.iter() {

            let correlative = format!("{}{}", c_start, c_end);
            println!("{}", correlative);

            correlatives.push(correlative);
        }
    }

    println!("{:?}", correlatives)

}
