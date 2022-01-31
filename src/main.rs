use std::fs;
use std::env;
use regex::Regex;

struct Unit {
    unit_id: String,
    unit_name: String,
    unit_num: f32,
    grade_num: f32,
    unit_group: String,
    year: String,
}

impl Unit {
    fn print(&self, group: &str, is_comp: bool) {
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
        } else {
            println!("{}/{}: {:3} {:<7} {}({})", group, sub_group, status, self.unit_id, self.unit_name, self.year);
        }
    }
}

struct User {
    units_a: Vec<Unit>,
    units_b: Vec<Unit>,
    units_c: Vec<Unit>,
    units_c0: Vec<Unit>,
    gpa: f32,
    gps: f32,
}

fn strec2unit(strec: csv::StringRecord) -> Unit {
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

fn create_user(csv: String) -> User {
    let mut rdr = csv::Reader::from_reader(csv.as_bytes());
    let mut units: Vec<Unit> = Vec::new();
    let mut units_a: Vec<Unit> = Vec::new();
    let mut units_b: Vec<Unit> = Vec::new();
    let mut units_c: Vec<Unit> = Vec::new();
    let mut units_c0: Vec<Unit> = Vec::new();

    for result in rdr.records() {
        units.push(strec2unit(result.unwrap()));
    }

    let mut gps: f32 = 0.0;
    let mut unum: f32 = 0.0;
    for unit in units {
        if unit.unit_group != "C0" && unit.grade_num >= 0.0  {
            gps += unit.unit_num * unit.grade_num;
            unum += unit.unit_num;
        }
        if unit.unit_group == "A" {
            units_a.push(unit);
        } else if unit.unit_group == "B" {
            units_b.push(unit);
        } else if unit.unit_group == "C" {
            units_c.push(unit);
        } else if unit.unit_group == "C0" {
            units_c0.push(unit);
        }
    }

    User {
        units_a: units_a,
        units_b: units_b,
        units_c: units_c,
        units_c0: units_c0,
        gpa: gps / unum,
        gps: gps,
    }
}

fn make_requirement(req: Vec<&str>) -> Vec<String> {
    let mut reqs: Vec<String> = Vec::new();
    for r in req {
        reqs.push(r.to_string());
    }
    reqs
}

fn check_req(units: Vec<Unit>, reqs: Vec<String>, group: &str) -> Vec<Unit> {
    let mut unitscp: Vec<Unit> = units;
    for req in reqs {
        let mut existance: bool = false;
        for unit in unitscp.iter().filter(|x| x.unit_name == req) {
            unit.print(group, true);
            existance = true;
        }
        if !existance {
            println!("{}/必修: \x1b[31m---\x1b[m         {}", group, req);
        }
        unitscp.retain(|x| x.unit_name != req);
    }
    unitscp
}

fn print_cmp(left: f32, right: f32, label: &str) {
    let fail = "\x1b[31mfail\x1b[m";
    let pass = "\x1b[32mpass\x1b[m";
    println!("{}: {:>4}/{:>2}   {}", if left < right {fail} else {pass}, left, right, label);
}

fn check(user: User) -> i32 {
    let a_req = make_requirement(vec!["主専攻実験A","主専攻実験B","卒業研究A","卒業研究B","専門語学A","専門語学B"]);
    let b_req = make_requirement(vec!["線形代数A","線形代数B","微分積分A","微分積分B","情報数学A","専門英語基礎","プログラミング入門","コンピュータとプログラミング","データ構造とアルゴリズム","データ構造とアルゴリズム実験","論理回路","論理回路実験"]);
    let c_req = make_requirement(vec!["フレッシュマン・セミナー","学問への誘い","English Reading Skills I","English Reading Skills II","English Presentation Skills I","English Presentation Skills II","情報リテラシー(講義)","情報リテラシー(演習)","データサイエンス"]);

    let mut units = check_req(user.units_a, a_req, "専門    ");
    let mut countn0: f32 = 0.0;
    let n0_reg: Regex = Regex::new(r"GB(2|3|4)0\d{3}").unwrap();
    let mut countn: f32 = 0.0;
    let n_reg: Regex = Regex::new(r"(GB(2|3|4)|GA4)\d{4}").unwrap();
    while units.len() > 0 {
        let unit = units.pop().unwrap();
        unit.print("専門    ", false);
        if unit.grade_num > 0.0 {
            if n0_reg.is_match(&unit.unit_id) {
                countn0 += unit.unit_num;
            } else if n_reg.is_match(&unit.unit_id) {
                countn += unit.unit_num;
            }
        }
    }

    units = check_req(user.units_b, b_req, "専門基礎");
    let mut misc: f32 = 0.0;
    let misc_reg: Regex = Regex::new(r"確率論|統計学|数値計算法|論理と形式化|電磁気学|論理システム|論理システム演習").unwrap();
    let mut cseng: f32 = 0.0;
    let cseng_reg: Regex = Regex::new(r"Computer Science in English (A|B)").unwrap();
    let mut gb1: f32 = 0.0;
    let mut ga1: f32 = 0.0;
    while units.len() > 0 {
        let unit = units.pop().unwrap();
        unit.print("専門基礎", false);
        if unit.grade_num > 0.0 {
            if misc_reg.is_match(&unit.unit_name) {
                misc += unit.unit_num;
            } else if &unit.unit_id[..3] == "GB1" {
                gb1 += unit.unit_num;
            } else if &unit.unit_id[..3] == "GA1" {
                ga1 += unit.unit_num;
            } else if cseng_reg.is_match(&unit.unit_name) {
                cseng += unit.unit_num;
            }
        }
    }

    units = check_req(user.units_c, c_req, "共通基礎");

    let mut pe1: f32 = 0.0;
    let mut pe2: f32 = 0.0;
    for unit in units.iter().filter(|x| &x.unit_id[..1] == "2") {
        unit.print("共通基礎", true);
        if unit.grade_num > 0.0 {
            if &unit.unit_id[1..2] == "1" {
                pe1 += unit.unit_num;
            } else if &unit.unit_id[1..2] == "2" {
                pe2 += unit.unit_num;
            }
        }
    }
    units.retain(|x| &x.unit_id[..1] != "2");

    if pe1 < 1.0 {
        println!("共通基礎/必修: \x1b[31m---\x1b[m         基礎体育");
    }
    if pe2 < 1.0 {
        println!("共通基礎/必修: \x1b[31m---\x1b[m         応用体育");
    }

    let mut acfnd: f32 = 0.0;
    let mut arts: f32 = 0.0;
    while units.len() > 0 {
        let unit = units.pop().unwrap();
        unit.print("共通基礎", false);
        if unit.grade_num > 0.0 {
            if &unit.unit_id[..2] == "12" || &unit.unit_id[..2] == "14" {
                acfnd += unit.unit_num;
            } else {
                arts += unit.unit_num;
            }
        }
    }

    units = user.units_c0;
    let mut science: f32 = 0.0;
    let mut not_science: f32 = 0.0;
    while units.len() > 0 {
        let unit = units.pop().unwrap();
        unit.print("関連基礎", false);
        if unit.grade_num > 0.0 {
            if &unit.unit_id[..1] == "E" || &unit.unit_id[..1] == "F" || &unit.unit_id[..2] == "GC" || &unit.unit_id[..2] == "GE" || &unit.unit_id[..1] == "H" {
                science += unit.unit_num;
            } else {
                not_science += unit.unit_num;
            }
        }
    }

    print_cmp(countn0, 18.0,                    "GBn0");
    print_cmp(countn.min(18.0) + countn0, 36.0, "専門選択");
    print_cmp(misc, 10.0,                       "確率論,統計学,数値計算法,論理と形式化,電磁気学,論理システム,論理システム演習");
    print_cmp(cseng, 2.0,                       "Computer Science in English A or B");
    print_cmp(ga1, 8.0,                         "GA1");
    print_cmp(misc + cseng + ga1 + gb1, 24.0,   "専門基礎選択");
    print_cmp(acfnd, 1.0,                       "総合科目 (学士基盤等)");
    let common = acfnd + arts.min(4.0);
    print_cmp(common, 1.0,                      "共通基礎選択");
    print_cmp(not_science, 6.0,                 "文系科目");
    let related = not_science + science.min(4.0);
    print_cmp(related, 6.0,                    "関連基礎選択");
    print_cmp(common.min(5.0) + related.min(10.0), 11.0, "基礎選択");

    println!("GPA: {:>.4}", user.gpa);
    println!("GPΣ: {:>.1}", user.gps);
    0
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args.len() > 2 {
        eprintln!("Usage error: tanici /path/to/file.csv");
        std::process::exit(1);
    } else {
        match fs::read_to_string(&args[1]) {
            Ok(data) => {
                let user = create_user(data);
                println!("start checking your graduation possibility");
                std::process::exit(check(user));
            },
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }
    }
}