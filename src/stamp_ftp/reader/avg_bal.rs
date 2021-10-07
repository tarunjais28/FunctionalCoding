use super::*;

#[derive(Debug, PartialEq)]
pub struct Balances {
    pub bal: f64,
    pub rate: f64,
}

#[derive(Debug)]
pub struct AverageBalance {
    pub avg_bal: AvgBalMap,
}

pub type AvgBalMap = HashMap<String, Balances>;
impl AverageBalance {
    pub fn new(file_path: &str) -> Self {
        let mut avg_bal: AvgBalMap = HashMap::new();
        let reader = read_file(file_path);
        for (line_num, lines) in reader.lines().enumerate().skip(1) {
            let line = extract_lines(line_num, lines, file_path);
            let fields: Vec<&str> = line.split('|').collect();
            if fields.len() == 3 {
                avg_bal.insert(
                    fields[0].to_string(),
                    Balances {
                        bal: parse_f64(fields[1]),
                        rate: parse_f64(fields[2]),
                    },
                );
            }
        }
        Self { avg_bal }
    }
}
