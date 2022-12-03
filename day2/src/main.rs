use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::BTreeMap;

fn main()  -> io::Result<()> {
    let file = File::open("input.txt")?;

    let mut scores = BTreeMap::new();
    scores.insert("X", 1);
    scores.insert("Y", 2);
    scores.insert("Z", 3);

    let mut wins = BTreeMap::new();
    wins.insert("Y", "A");
    wins.insert("Z", "B");
    wins.insert("X", "C");

    let mut draws = BTreeMap::new();
    draws.insert("X","A");
    draws.insert("Y","B");
    draws.insert("Z","C");

    let mut losesinv: BTreeMap<&str, &str> = BTreeMap::new();
    losesinv.insert("C", "Y");
    losesinv.insert("A", "Z");
    losesinv.insert("B", "X");

    let mut winsinv: BTreeMap<&str, &str> = BTreeMap::new();
    winsinv.insert("A", "Y" );
    winsinv.insert("B", "Z" );
    winsinv.insert("C", "X" );

    let mut drawsinv: BTreeMap<&str, &str> = BTreeMap::new();
    drawsinv.insert("A", "X");
    drawsinv.insert("B", "Y");
    drawsinv.insert("C", "Z");


   
    let reader = BufReader::new(file);

    let mut score = 0;
    for line in reader.lines() {
        if let Ok(l) = line {
            let la: Vec<&str> = l.split(" ").collect();
            //println!("Abc {}", la[1]);
            score += scores[la[1]];
            if la[0] == wins[la[1]] {
                score += 6;
            } else if la[0] == draws[la[1]] {
                score += 3;
            }
        }
    }
    println!("Score part 1 {score}");

    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    score = 0;
    for line in reader.lines() {
        if let Ok(l) = line {
            let la: Vec<&str> = l.split(" ").collect();
            let mut selectedValue = "X";
            if la[1] == "X" {
                selectedValue = losesinv[la[0]];
            } else if la[1] == "Y" {
                selectedValue = drawsinv[la[0]];
                score += 3;
            } else {
                selectedValue = winsinv[la[0]];
                score += 6;
            }
            score += scores[selectedValue];
        }
    }
    println!("Score part 2 {score}");
    Ok(())
}
