use std::fs;
use std::env;
use std::collections::HashMap;

#[macro_use]
extern crate clap;
use clap::{App, Arg};

extern crate yaml_rust;
use yaml_rust::YamlLoader;

pub mod unit;

pub mod unitgroup;
use unitgroup::UnitGroupMap;

pub mod require;
use require::Req;

pub mod user;
use user::User;

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
    let reqs: Vec<Req>;
    let mut sums: HashMap<String, f32> = HashMap::new();
    
    match fs::read_to_string(yaml_path) {
        Ok(data) => {
            match YamlLoader::load_from_str(&data) {
                Ok(yaml) => {
                    groups_a.push_yaml(&yaml, "groups_a");
                    groups_b.push_yaml(&yaml, "groups_b");
                    groups_c.push_yaml(&yaml, "groups_c");
                    groups_c0.push_yaml(&yaml, "groups_c0");
                    reqs = match Req::reqs_yaml(&yaml) {
                        Some(reqs) => reqs,
                        None => Vec::new(),
                    }
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
            sums.extend(groups_a.sums);
            sums.extend(groups_b.sums);
            sums.extend(groups_c.sums);
            sums.extend(groups_c0.sums);

            for req in reqs {
                req.check_req(&sums, verbose);
            }

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