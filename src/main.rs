use std::fs;
use std::env;

mod unit;
use unit::Unit;

mod requirement;
use requirement::Requirement;
use requirement::RequirementGroup;

mod user;
use user::User;

fn print_cmp(left: f32, right: f32, label: &str) {
    let fail = "\x1b[31mfail\x1b[m";
    let pass = "\x1b[32mpass\x1b[m";
    println!("{}: {:>4}/{:>2}   {}", if left < right {fail} else {pass}, left, right, label);
}

fn check(units_a: Vec<Unit>, units_b: Vec<Unit>, units_c: Vec<Unit>, units_c0: Vec<Unit>,) -> (RequirementGroup, RequirementGroup, RequirementGroup, RequirementGroup) {
    let mut a_reqs = RequirementGroup::comp_reqs("専門    ", vec!["主専攻実験A","主専攻実験B","卒業研究A","卒業研究B","専門語学A","専門語学B"]);
    a_reqs.add_req(Requirement::req_id("n0", "GB(2|3|4)0", 4, false));
    a_reqs.add_req(Requirement::req_name("n", ".*", false));
    a_reqs.push_units(units_a);

    let mut b_reqs = RequirementGroup::comp_reqs("専門基礎", vec!["線形代数A","線形代数B","微分積分A","微分積分B","情報数学A","専門英語基礎","プログラミング入門","コンピュータとプログラミング","データ構造とアルゴリズム","データ構造とアルゴリズム実験","論理回路","論理回路実験"]);
    b_reqs.add_req(Requirement::req_name("misc", r"確率論|統計学|数値計算法|論理と形式化|電磁気学|論理システム|論理システム演習", false));
    b_reqs.add_req(Requirement::req_name("cseng", r"Computer Science in English (A|B)", false));
    b_reqs.add_req(Requirement::req_id("ga1", "GA1", 3, false));
    b_reqs.add_req(Requirement::req_id("gb1", "GB1", 3, false));
    b_reqs.push_units(units_b);

    let mut c_reqs = RequirementGroup::comp_reqs("共通基礎", vec!["フレッシュマン・セミナー","学問への誘い","English Reading Skills I","English Reading Skills II","English Presentation Skills I","English Presentation Skills II","情報リテラシー\\(講義\\)","情報リテラシー\\(演習\\)","データサイエンス"]);
    c_reqs.add_req(Requirement::req_id("pe1", "21", 2, true));
    c_reqs.add_req(Requirement::req_id("pe2", "22", 2, true));
    c_reqs.add_req(Requirement::req_id("acfnd", "12|14", 2, false));
    c_reqs.add_req(Requirement::req_name("arts", ".*", false));
    c_reqs.push_units(units_c);

    let mut c0_reqs = RequirementGroup::new("関連基礎");
    c0_reqs.add_req(Requirement::req_id("sci", "(E|F|H).|G(C|E)", 2, false));
    c0_reqs.add_req(Requirement::req_name("nsci", ".*", false));
    c0_reqs.push_units(units_c0);

    return (a_reqs, b_reqs, c_reqs, c0_reqs);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &str;
    let mut verbose: bool = false;
    match args.len() {
        2 => filename = &args[1],
        3 => match args[1].as_str() {
            "--verbose" => {
                verbose = true; 
                filename = &args[2];
            },
            _ => {
                eprintln!("Usage error: tanici --option /path/to/file.csv");
                std::process::exit(1);
            }
        },
        _ => {
            eprintln!("Usage error: tanici /path/to/file.csv");
            std::process::exit(1);
        }
    }
    match fs::read_to_string(filename) {
        Ok(data) => {
            let user = User::new(data);
            println!("start checking your graduation possibility");
            let (a_reqs, b_reqs, c_reqs, c0_reqs) = check(user.units_a, user.units_b, user.units_c, user.units_c0);
            a_reqs.print(verbose);
            b_reqs.print(verbose);
            c_reqs.print(verbose);
            c0_reqs.print(verbose);
            
            let n0sum: f32 = *a_reqs.sums.get("n0").unwrap_or(&0.0);
            let nsum: f32 = *a_reqs.sums.get("n").unwrap_or(&0.0);
            let miscsum: f32 = *b_reqs.sums.get("misc").unwrap_or(&0.0);
            let csengsum: f32 = *b_reqs.sums.get("cseng").unwrap_or(&0.0);
            let ga1sum: f32 = *b_reqs.sums.get("ga1").unwrap_or(&0.0);
            let gb1sum: f32 = *b_reqs.sums.get("gb1").unwrap_or(&0.0);
            let acfndsum: f32 = *c_reqs.sums.get("acfnd").unwrap_or(&0.0);
            let artsum: f32 = *c_reqs.sums.get("arts").unwrap_or(&0.0);
            let scisum: f32 = *c0_reqs.sums.get("sci").unwrap_or(&0.0);
            let nscisum: f32 = *c0_reqs.sums.get("nsci").unwrap_or(&0.0);

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

            println!("GPA: {:>.4}", user.gpa);
            println!("GPΣ: {:>.1}", user.gps);
            std::process::exit(0);
        },
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}