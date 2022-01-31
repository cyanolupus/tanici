pub struct Unit {
    pub unit_id: String,
    pub unit_name: String,
    pub unit_num: f32,
    pub grade_num: f32,
    pub unit_group: String,
    year: String,
}

impl Unit {
    pub fn new(strec: csv::StringRecord) -> Self {
        if strec.len() != 11 {
            eprintln!("Format error");
            std::process::exit(1);
        }
        let unit_id = strec[2].to_string();
        let unit_name = strec[3].to_string();
        let unit_num = strec[4][1..].parse::<f32>().unwrap();
        let unit_group = strec[8].to_string();
        let year = strec[9].to_string();
        let grade_num;

        if &strec[7] == "A+" {
            grade_num = 4.3;
        } else if &strec[7] == "A" {
            grade_num = 4.0;
        } else if &strec[7] == "B" {
            grade_num = 3.0;
        } else if &strec[7] == "C" {
            grade_num = 2.0;
        } else if &strec[7] == "D" {
            grade_num = 0.0;
        } else if &strec[7] == "履修中" {
            grade_num = -2.0;
        } else {
            grade_num = -1.0;
        }

        Unit {
            unit_id,
            unit_name,
            unit_num,
            grade_num,
            unit_group,
            year,
        }
    }

    pub fn print(&self, group: &str, is_comp: bool, verbose: bool) {
        let status: String;
        let sub_group = if is_comp {"必修"} else {"選択"};

        if self.grade_num < -1.0 {
            status = "\x1b[33mWIP\x1b[m".to_string();
        } else if self.grade_num == 0.0 {
            status = "\x1b[31m-d-\x1b[m".to_string();
        } else {
            status = format!("{:1.1}", self.unit_num);
        }

        if is_comp {
            println!("{}/{}: \x1b[32m{:3}\x1b[m {:<7} {}({})", group, sub_group, status, self.unit_id, self.unit_name, self.year);
        } else if verbose {
            println!("{}/{}: {:3} {:<7} {}({})", group, sub_group, status, self.unit_id, self.unit_name, self.year);
        }
    }
}