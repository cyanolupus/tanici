use std::fs;
use std::env;

#[macro_use]
extern crate clap;
use clap::{App, Arg};

extern crate yaml_rust;
use yaml_rust::YamlLoader;

pub mod unit;

pub mod unitgroup;
use unitgroup::UnitGroupMap;

pub mod user;
use user::User;

fn print_cmp(left: f32, right: f32, label: &str) {
    let fail = "\x1b[31mfail\x1b[m";
    let pass = "\x1b[32mpass\x1b[m";
    println!("{}: {:>4}/{:>2}   {}", if left < right {fail} else {pass}, left, right, label);
}

fn main() {
    let app = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(Arg::with_name("csv")
            .help("CSV file path")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("verbose")
            .short("v")
            .long("verbose")
            .help("print all units"))
        .arg(Arg::with_name("requirements")
            .short("i")
            .long("import")
            .help("import requirements from yaml file")
            .takes_value(true)
            .required(true));
    
    let matches = app.get_matches();
    let csv_path = matches.value_of("csv").unwrap();
    let yaml_path = matches.value_of("requirements").unwrap();
    let verbose: bool = matches.is_present("verbose");
    let mut groups_a = UnitGroupMap::new("専門    ");
    let mut groups_b = UnitGroupMap::new("専門基礎");
    let mut groups_c = UnitGroupMap::new("共通基礎");
    let mut groups_c0 = UnitGroupMap::new("関連基礎");
    
    match fs::read_to_string(yaml_path) {
        Ok(data) => {
            match YamlLoader::load_from_str(&data) {
                Ok(yaml) => {
                    groups_a.push_yaml(&yaml, "groups_a");
                    groups_b.push_yaml(&yaml, "groups_b");
                    groups_c.push_yaml(&yaml, "groups_c");
                    groups_c0.push_yaml(&yaml, "groups_c0");
                },
                Err(e) => {
                    eprintln!("{}", e);
                    std::process::exit(1);
                }
            }
        },
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }

    match fs::read_to_string(csv_path) {
        Ok(data) => {
            let user = User::new(data);
            println!("start checking your graduation possibility");
            groups_a.push_units(user.units_a);
            groups_b.push_units(user.units_b);
            groups_c.push_units(user.units_c);
            groups_c0.push_units(user.units_c0);
            groups_a.print(verbose);
            groups_b.print(verbose);
            groups_c.print(verbose);
            groups_c0.print(verbose);
            
            let n0sum: f32 = *groups_a.sums.get("gbn0").unwrap_or(&0.0);
            let nsum: f32 = *groups_a.sums.get("gbn").unwrap_or(&0.0);
            let miscsum: f32 = *groups_b.sums.get("misc").unwrap_or(&0.0);
            let csengsum: f32 = *groups_b.sums.get("cseng").unwrap_or(&0.0);
            let ga1sum: f32 = *groups_b.sums.get("ga1").unwrap_or(&0.0);
            let gb1sum: f32 = *groups_b.sums.get("gb1").unwrap_or(&0.0);
            let acfndsum: f32 = *groups_c.sums.get("acfnd").unwrap_or(&0.0);
            let artsum: f32 = *groups_c.sums.get("arts").unwrap_or(&0.0);
            let scisum: f32 = *groups_c0.sums.get("sci").unwrap_or(&0.0);
            let nscisum: f32 = *groups_c0.sums.get("nonsci").unwrap_or(&0.0);

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