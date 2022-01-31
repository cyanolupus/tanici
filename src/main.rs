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
    fn new(strec: csv::StringRecord) -> Self {
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

impl User {
    fn new(csv: String) -> Self {
        let mut rdr = csv::Reader::from_reader(csv.as_bytes());
        let mut units: Vec<Unit> = Vec::new();
        let mut units_a: Vec<Unit> = Vec::new();
        let mut units_b: Vec<Unit> = Vec::new();
        let mut units_c: Vec<Unit> = Vec::new();
        let mut units_c0: Vec<Unit> = Vec::new();
    
        for result in rdr.records() {
            units.push(Unit::new(result.unwrap()));
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
}

enum RegType {
    Exist(Regex),
    NotExist
}

struct Requirement {
    name: String,
    name_reg: RegType,
    id_reg: RegType,
    sum: f32,
    units: Vec<Unit>,
    is_comp: bool,
}

impl Requirement {
    fn req_name(name: &str, reg: &str, is_comp: bool) -> Self {
        let name_reg = Regex::new(reg).unwrap();
        let units = Vec::new();
        Requirement {
            name: name.to_string(),
            name_reg: RegType::Exist(name_reg),
            id_reg: RegType::NotExist,
            sum: 0.0,
            units,
            is_comp
        }
    }

    fn req_id(name: &str, id: &str, matchlen: u8, is_comp: bool) -> Self {
        let id_reg = Regex::new(&format!(r"^{}.{{{}}}$", id, 7 - matchlen)).unwrap();
        let units = Vec::new();
        Requirement {
            name: name.to_string(),
            name_reg: RegType::NotExist,
            id_reg: RegType::Exist(id_reg),
            sum: 0.0,
            units,
            is_comp
        }
    }

    fn check(&mut self, unit: &Unit) -> bool{
        let name_match = match self.name_reg {
            RegType::Exist(ref reg) => reg.is_match(&unit.unit_name),
            RegType::NotExist => true,
        };
        let id_match = match self.id_reg {
            RegType::Exist(ref reg) => reg.is_match(&unit.unit_id),
            RegType::NotExist => true,
        };
        if name_match && id_match && unit.grade_num > 0.0 {
            self.sum += unit.unit_num;
        }
        return name_match && id_match;
    }

    fn print(&self, group_name: &str) {
        if self.units.len() == 0 && self.is_comp {
            println!("{}/必修: \x1b[31m---\x1b[m         {}", group_name, self.name);
        }
        for unit in self.units.iter() {
            unit.print(group_name, self.is_comp);
        }
    }
}

fn make_reqs_name(names: Vec<&str>) -> Vec<Requirement> {
    let mut reqs: Vec<Requirement> = Vec::new();
    for name in names {
        reqs.push(Requirement::req_name(name, &format!(r"^{}$", name), true));
    }
    return reqs;
}

fn print_cmp(left: f32, right: f32, label: &str) {
    let fail = "\x1b[31mfail\x1b[m";
    let pass = "\x1b[32mpass\x1b[m";
    println!("{}: {:>4}/{:>2}   {}", if left < right {fail} else {pass}, left, right, label);
}

fn check_req(units: Vec<Unit>, reqs: Vec<Requirement>) -> Vec<Requirement> {
    let mut unitscp = units;
    let mut reqscp = reqs;
    'loop0: while unitscp.len() > 0 {
        let unit = unitscp.pop().unwrap();
        for req in reqscp.iter_mut() {
            if req.check(&unit) {
                req.units.push(unit);
                continue 'loop0;
            }
        }
    }
    return reqscp;
}

fn check(user: User) -> i32 {
    let mut a_reqs = make_reqs_name(vec!["主専攻実験A","主専攻実験B","卒業研究A","卒業研究B","専門語学A","専門語学B"]);
    a_reqs.push(Requirement::req_id("n0", "GB(2|3|4)0", 4, false));
    a_reqs.push(Requirement::req_name("n", ".*", false));
    a_reqs = check_req(user.units_a, a_reqs);

    let mut b_reqs = make_reqs_name(vec!["線形代数A","線形代数B","微分積分A","微分積分B","情報数学A","専門英語基礎","プログラミング入門","コンピュータとプログラミング","データ構造とアルゴリズム","データ構造とアルゴリズム実験","論理回路","論理回路実験"]);
    b_reqs.push(Requirement::req_name("misc", r"確率論|統計学|数値計算法|論理と形式化|電磁気学|論理システム|論理システム演習", false));
    b_reqs.push(Requirement::req_name("cseng", r"Computer Science in English (A|B)", false));
    b_reqs.push(Requirement::req_id("ga1", "GA1", 3, false));
    b_reqs.push(Requirement::req_id("gb1", "GB1", 3, false));
    b_reqs = check_req(user.units_b, b_reqs);

    let mut c_reqs = make_reqs_name(vec!["フレッシュマン・セミナー","学問への誘い","English Reading Skills I","English Reading Skills II","English Presentation Skills I","English Presentation Skills II","情報リテラシー\\(講義\\)","情報リテラシー\\(演習\\)","データサイエンス"]);
    c_reqs.push(Requirement::req_id("pe1", "21", 2, true));
    c_reqs.push(Requirement::req_id("pe2", "22", 2, true));
    c_reqs.push(Requirement::req_id("acfnd", "12|14", 2, false));
    c_reqs.push(Requirement::req_name("arts", ".*", false));
    c_reqs = check_req(user.units_c, c_reqs);

    let mut c0_reqs: Vec<Requirement> = Vec::new();
    c0_reqs.push(Requirement::req_id("sci", "(E|F|H).|G(C|E)", 2, false));
    c0_reqs.push(Requirement::req_name("nsci", ".*", false));
    c0_reqs = check_req(user.units_c0, c0_reqs);

    let mut n0sum: f32 = 0.0;
    let mut nsum: f32 = 0.0;
    let mut miscsum: f32 = 0.0;
    let mut csengsum: f32 = 0.0;
    let mut ga1sum: f32 = 0.0;
    let mut gb1sum: f32 = 0.0;
    let mut acfndsum: f32 = 0.0;
    let mut artsum: f32 = 0.0;
    let mut scisum: f32 = 0.0;
    let mut nscisum: f32 = 0.0;

    for req in a_reqs {
        req.print("専門    ");
        match &*req.name {
            "n0" => n0sum = req.sum,
            "n" => nsum = req.sum,
            _ => {}
        }
    }

    for req in b_reqs {
        req.print("専門基礎");
        match &*req.name {
            "misc" => miscsum = req.sum,
            "cseng" => csengsum = req.sum,
            "ga1" => ga1sum = req.sum,
            "gb1" => gb1sum = req.sum,
            _ => {}
        }
    }

    for req in c_reqs {
        req.print("共通基礎");
        match &*req.name {
            "acfnd" => acfndsum = req.sum,
            "arts" => artsum = req.sum,
            _ => {}
        }
    }

    for req in c0_reqs {
        req.print("関連基礎");
        match &*req.name {
            "sci" => scisum = req.sum,
            "nsci" => nscisum = req.sum,
            _ => {}
        }
    }

    let spec = nsum.min(18.0) + n0sum;
    let specf = miscsum + csengsum + ga1sum + gb1sum;
    let common = acfndsum + artsum.min(4.0);
    let related = nscisum + scisum.min(4.0);

    print_cmp(n0sum, 18.0,     "GBn0");
    print_cmp(spec, 36.0,      "専門選択");
    print_cmp(miscsum, 10.0,   "確率論,統計学,数値計算法,論理と形式化,電磁気学,論理システム,論理システム演習");
    print_cmp(csengsum, 2.0,   "Computer Science in English A or B");
    print_cmp(ga1sum, 8.0,     "GA1");
    print_cmp(specf, 24.0,     "専門基礎選択");
    print_cmp(acfndsum, 1.0,   "総合科目 (学士基盤等)");
    print_cmp(common, 1.0,     "共通基礎選択");
    print_cmp(nscisum, 6.0,    "文系科目");
    print_cmp(related, 6.0,    "関連基礎選択");
    print_cmp(common.min(5.0) + related.min(10.0), 11.0, "基礎選択");
    println!("{}", artsum);

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
                let user = User::new(data);
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