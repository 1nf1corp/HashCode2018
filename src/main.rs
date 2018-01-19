
use std::env;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::str::FromStr;
use std::collections::HashMap;

struct Pizza {
    rows: usize,
    cols: usize,
    min_ingr: usize,
    max_cells: usize,
    matrix: Vec<Vec<Option<char>>>,
    slices: Vec<Slice>,
}

impl Pizza {
    pub fn new(rows: usize, cols: usize, min_ingr: usize, max_cells: usize) -> Pizza {
        Pizza {
            rows: rows,
            cols: cols,
            min_ingr: min_ingr,
            max_cells: max_cells,
            matrix: Vec::with_capacity(rows),
            slices: Vec::new(),
        }
    }

    pub fn add_row(&mut self, r: usize, s: String) {
        if self.matrix.len() <= r {
            self.matrix.push(Vec::with_capacity(self.cols));
        }

        for c in s.chars() {
            self.matrix[r].push(Some(c));
        }
    }

    pub fn cut(&mut self) -> &Pizza {
        for row1 in 0..self.rows {
            for col1 in 0..self.cols {
                //if it's an already used slot let's skip it
                if self.matrix[row1][col1] == None {
                    continue;
                }

                for row2 in row1..self.rows {
                    //slices can't overlap
                    if self.matrix[row2][col1] == None {
                        break;
                    }

                    for col2 in col1..self.cols {
                        //slices can't overlap
                        if self.matrix[row2][col2] == None {
                            break;
                        }

                        if self.is_valid_slice(row1, col1, row2, col2) {
                            self.save_slice(row1, col1, row2, col2);
                        }
                        
                    }
                }
            }
        }

        self
    }

    fn is_valid_slice(&self, row1: usize, col1: usize, row2: usize, col2: usize) -> bool {
        if ((row2 - row1) + 1) * ((col2 - col1) + 1) > self.max_cells {
            return false;
        }

        let mut ingredients = HashMap::new();
        for row in row1..row2 + 1 {
            for col in col1..col2 + 1 {
                match self.matrix[row][col] {
                    Some(s) => {
                        let count = ingredients.entry(s).or_insert(0);
                        *count += 1;
                    },
                    None => return false,
                }
            }
        }

        *ingredients.values().min().unwrap() >= self.min_ingr
    }

    fn save_slice(&mut self, row1: usize, col1: usize, row2: usize, col2: usize) {
        for row in row1..row2 {
            for col in col1..col2 {
                self.matrix[row][col] = None;
            }
        }

        self.slices.push(Slice {
            row1: row1,
            col1: col1,
            row2: row2,
            col2: col2,
        });
    }
}

impl FromStr for Pizza {
    type Err = String;

    fn from_str(d: &str) -> Result<Self, Self::Err> {
        let s = d.to_owned();
        let p: Vec<&str> = s.split(' ').collect();
        if p.len() != 4 {
            Err(format!("Wrong input {}", d))
        }
        else {
            Ok(Pizza::new(p[0].parse().unwrap(), p[0].parse().unwrap(), p[0].parse().unwrap(), p[0].parse().unwrap()))
        }
    }
}

impl ToString for Pizza {
    fn to_string(&self) -> String {
        let mut s = self.slices.len().to_string();

        for n in 0..self.slices.len() {
            s = format!("{}\n{}", s, self.slices[n].to_string());
        }

        s
    }
}

struct Slice {
    row1: usize,
    col1: usize,
    row2: usize,
    col2: usize,
}

impl ToString for Slice {
    fn to_string(&self) -> String {
        format!("{} {} {} {}", self.row1, self.col1, self.row2, self.col2)
    }
}

fn main() {
    let args:Vec<String> = env::args().collect();
    assert!(args.len() == 2, "Usage: <file.in>");

    let f = File::open(args.get(1).unwrap()).unwrap();
    let file = BufReader::new(&f);
    let mut iter = file.lines();

    let mut p:Pizza = iter.next().unwrap().unwrap().parse().unwrap();//orribile
    for i in 0..p.rows {
        p.add_row(i, iter.next().unwrap().unwrap());
    }

    println!("{}", p.cut().to_string());
}
