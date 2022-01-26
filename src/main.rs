use std::fs;
use std::env;

#[derive(Clone)]
struct Unit {
    student_id: String,
    student_name: String,
    unit_id: String,
    unit_name: String,
    unit_num: f32,
    spring_grade: String,
    autumn_grade: String,
    grade: String,
    grade_num: f32,
    unit_group: String,
    year: String,
    group: String,
}

struct User {
    units_a: Vec<Unit>,
    units_b: Vec<Unit>,
    units_c: Vec<Unit>,
    units_c0: Vec<Unit>,
    gpa: f32,
    gps: f32,
    units_num: f32,
}

fn strec2unit(strec: csv::StringRecord) -> Unit {
    let student_id = strec[0].to_string();
    let student_name = strec[1].to_string();
    let unit_id = strec[2].to_string();
    let unit_name = strec[3].to_string();
    let unit_num = strec[4][1..].parse::<f32>().unwrap();
    let spring_grade = strec[5].to_string();
    let autumn_grade = strec[6].to_string();
    let grade = strec[7].to_string();
    let unit_group = strec[8].to_string();
    let year = strec[9].to_string();
    let group = strec[10].to_string();
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
        student_id,
        student_name,
        unit_id,
        unit_name,
        unit_num,
        spring_grade,
        autumn_grade,
        grade,
        grade_num,
        unit_group,
        year,
        group,
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
        units_num: unum,
    }
}

fn make_requirement(req: Vec<&str>) -> Vec<String> {
    let mut reqs: Vec<String> = Vec::new();
    for r in req {
        reqs.push(r.to_string());
    }
    reqs
}

fn colorize(s: &str, color: u8) -> String {
    return format!("\x1b[{}m{}\x1b[m", color, s);
}

fn check_req(units: Vec<Unit>, reqs: Vec<String>, group: &str) -> Vec<Unit> {
    let mut unitscp: Vec<Unit> = units;
    for req in reqs {
        if unitscp.iter().any(|x| x.unit_name == req && x.grade_num > -2.0) {
            let unit = unitscp.iter().find(|x| x.unit_name == req).unwrap();
            println!("{}: \x1b[32m{:>2.1}\x1b[m {:<7} {}", group, unit.unit_num, unit.unit_id, req);
            unitscp.retain(|x| x.unit_name != req);
        } else {
            println!("{}:  {}         {}", group, colorize("--", 31), req);
        }
    }
    unitscp
}

fn check(user: User) -> i32 {
    println!("start checking {}'s graduation possibility", user.units_a[0].student_name);

    let a_req = make_requirement(vec!["主専攻実験A","主専攻実験B","卒業研究A","卒業研究B","専門語学A","専門語学B"]);
    let b_req = make_requirement(vec!["線形代数A","線形代数B","微分積分A","微分積分B","情報数学A","専門英語基礎","プログラミング入門","コンピュータとプログラミング","データ構造とアルゴリズム","データ構造とアルゴリズム実験","論理回路","論理回路実験"]);
    let c_req = make_requirement(vec!["フレッシュマン・セミナー","学問への誘い","English Reading Skills I","English Reading Skills II","English Presentation Skills I","English Presentation Skills II","情報リテラシー(講義)","情報リテラシー(演習)","データサイエンス"]);

    let mut units = check_req(user.units_a, a_req, "専門    科目");
    let mut countn0: f32 = 0.0;
    let mut countn: f32 = 0.0;
    while units.len() > 0 {
        let unit = units.pop().unwrap();
        if unit.grade_num > -2.0 {
            println!("専門    科目: {:>2.1} {:<7} {}", unit.unit_num, unit.unit_id, unit.unit_name);
            if &unit.unit_id[..4] == "GB40" || &unit.unit_id[..4] == "GB30" || &unit.unit_id[..4] == "GB20" {
                countn0 += unit.unit_num;
            } else {
                countn += unit.unit_num;
            }
        } else {
            println!("専門    科目: {}         {}", colorize("WIP", 33), unit.unit_name);
        }
    }

    if countn.min(18.0) + countn0 < 36.0 {
        println!("{} GBn + GBn0 = {} + {}{}", colorize("fail", 31), countn, countn0, colorize(" < 36", 31));
    } else {
        println!("GBn + GBn0 = {} + {}{}", countn, countn0, colorize(" >= 36", 42));
    }

    units = check_req(user.units_b, b_req, "専門基礎科目");
    let mut misc: f32 = 0.0;
    let mut cseng: f32 = 0.0;
    let mut gb1: f32 = 0.0;
    let mut ga1: f32 = 0.0;
    while units.len() > 0 {
        let unit = units.pop().unwrap();
        if unit.grade_num > -2.0 {
            println!("専門基礎科目: {:>2.1} {} {}", unit.unit_num, unit.unit_id, unit.unit_name);
            if unit.unit_name == "確率論" || unit.unit_name == "統計学" || unit.unit_name == "数値計算法" || unit.unit_name == "論理と形式化" || unit.unit_name == "電磁気学" || unit.unit_name == "論理システム" || unit.unit_name == "論理システム演習" {
                misc += unit.unit_num;
            }else if &unit.unit_id[..3] == "GB1" {
                gb1 += unit.unit_num;
            } else if &unit.unit_id[..3] == "GA1" {
                ga1 += unit.unit_num;
            } else if &unit.unit_name == "Computer Science in English A" || &unit.unit_name == "Computer Science in English B" {
                cseng += unit.unit_num;
            }
        } else {
            println!("専門基礎科目: {}         {}", colorize("WIP", 33), unit.unit_name);
        }
    }

    if misc < 10.0 {
        println!("{} misc = {}{}", colorize("fail", 31), misc, colorize(" < 10", 31));
    } else if cseng < 2.0 {
        println!("{} cseng = {}{}", colorize("fail", 31), cseng, colorize(" < 2", 31));
    } else if ga1 < 8.0 {
        println!("{} ga1 = {}{}", colorize("fail", 31), ga1, colorize(" < 8", 31));
    } else if misc + cseng + ga1 < 24.0 {
        println!("{} misc + cseng + gb1 + ga1 = {}{}", colorize("fail", 31), misc + cseng + gb1 + ga1, colorize(" < 24", 31));
    } else {
        println!("{} misc:{}, CSEng:{}, GB1:{}, GA1{}", colorize("pass", 42), misc, cseng, gb1, ga1);
    }

    units = check_req(user.units_c, c_req, "共通基礎科目");

    let mut pe: f32 = 0.0;
    while units.iter().any(|x| x.unit_name.len() > 6 && x.unit_group == "C" && &x.unit_name[6..12] == "体育" && x.grade_num > -2.0) {
        let unitsbkt2: Vec<Unit> = units.clone();
        let unit = unitsbkt2.iter().find(|x| x.unit_name.len() > 6 && x.unit_group == "C" && &x.unit_name[6..12] == "体育" && x.grade_num > -2.0).unwrap();
        println!("共通基礎科目: \x1b[32m{:>2.1}\x1b[m {} {}", unit.unit_num, unit.unit_id, unit.unit_name);
        pe += unit.unit_num;
        units.retain(|x| x.unit_id != unit.unit_id);
    }

    if pe < 2.0 {
        println!("共通基礎科目:  {}         体育", colorize("NY", 31));
    }

    let mut acfnd: f32 = 0.0;
    let mut arts: f32 = 0.0;
    while units.len() > 0 {
        let unit = units.pop().unwrap();
        if unit.grade_num > -2.0 {
            println!("共通基礎科目: {:>2.1} {} {}", unit.unit_num, unit.unit_id, unit.unit_name);
            if &unit.unit_id[..2] == "12" || &unit.unit_id[..2] == "14" {
                acfnd += unit.unit_num;
            } else {
                arts += unit.unit_num;
            }
        } else {
            println!("共通基礎科目: {}         {}", colorize("WIP", 33), unit.unit_name);
        }
    }

    if acfnd < 1.0 {
        println!("{} acfnd = {}{}", colorize("fail", 31), acfnd, colorize(" < 1", 31));
    } else {
        println!("{} acfnd:{}", colorize("pass", 32), acfnd);
    }

    units = user.units_c0;
    let mut science: f32 = 0.0;
    let mut not_science: f32 = 0.0;
    while units.len() > 0 {
        let unit = units.pop().unwrap();
        if unit.grade_num > -2.0 {
            println!("関連基礎科目: {:>2.1} {} {}", unit.unit_num, unit.unit_id, unit.unit_name);
            if &unit.unit_id[..1] == "E" || &unit.unit_id[..1] == "F" || &unit.unit_id[..2] == "GC" || &unit.unit_id[..2] == "GE" || &unit.unit_id[..1] == "H" {
                science += unit.unit_num;
            } else {
                not_science += unit.unit_num;
            }
        } else {
            println!("関連基礎科目: {}         {}", colorize("WIP", 33), unit.unit_name);
        }
    }

    if not_science < 6.0 {
        println!("{} not_science = {}{}", colorize("fail", 31), not_science, colorize(" < 6", 31));
    } else {
        println!("{} not_science:{}", colorize("pass", 32), not_science);
    }

    if not_science + science.min(4.0) + acfnd + arts.min(4.0) < 11.0 {
        println!("{} acfnd + arts + not_science + science = {}{}", colorize("fail", 31), not_science + science + acfnd + arts, colorize(" < 11", 31));
    } else {
        println!("{} acfnd:{}, arts:{}, not_science:{}, science:{}", colorize("pass", 32), acfnd, arts, not_science, science);
    }

    println!("gpa: {:>.4}\ngps: {:>.1}\nunits_num: {:>.0}", user.gpa, user.gps, user.units_num);
    0
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args.len() > 2 {
        eprintln!("Usage error: tanici /path/to/file.csv");
        std::process::exit(1);
    } else {
        let cont: String = fs::read_to_string(&args[1]).unwrap();
        let user = create_user(cont);
        std::process::exit(check(user));
    }
}