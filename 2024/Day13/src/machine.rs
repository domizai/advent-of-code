use regex::Regex;

#[derive(Debug)]
pub struct Button { 
    pub x: i64, 
    pub y: i64 
}

impl Button {
    pub fn from_string(s: &str) -> Button {
        let re = Regex::new(r"X(?<x>(?:\+|-)\d+), Y(?<y>(?:\+|-)\d+)").unwrap();
        let cap = re.captures(s).unwrap();
        let x = cap.name("x").unwrap().as_str().parse::<i64>().unwrap();
        let y = cap.name("y").unwrap().as_str().parse::<i64>().unwrap();
        Button { x, y }
    }
}

#[derive(Debug)]
pub struct Machine {
    pub a: Button,
    pub b: Button,
    pub price_x: i64,
    pub price_y: i64,
}

impl Machine {
    pub const COST_A: i64 = 3;
    pub const COST_B: i64 = 1;

    pub fn from_string(s: &str) -> Machine {
        let lines = s.lines().map(|l| l.trim()).collect::<Vec<&str>>();
        let re = Regex::new(r"X=(?<price_x>\d+), Y=(?<price_y>\d+)").unwrap();
        let cap = re.captures(lines[2]).unwrap();

        Machine { 
            a: Button::from_string(lines[0]),
            b: Button::from_string(lines[1]),
            price_x: cap.name("price_x").unwrap().as_str().parse::<i64>().unwrap(),
            price_y: cap.name("price_y").unwrap().as_str().parse::<i64>().unwrap(),
        }
    }
}
