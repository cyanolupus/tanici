use regex::Regex;
use super::unit::Unit;
use std::collections::HashMap;

extern crate yaml_rust;
use yaml_rust::{YamlLoader, Yaml};

pub struct Requirement {
    name: String,
    name_reg: Option<Regex>,
    id_reg: Option<Regex>,
    pub units: Vec<Unit>,
    is_comp: bool,
}

impl Requirement {
    fn req_name(name: &str, reg: &str, is_comp: bool) -> Self {
        let name_reg = Regex::new(reg).unwrap();
        let units: Vec<Unit> = Vec::new();
        Requirement {
            name: name.to_string(),
            name_reg: Some(name_reg),
            id_reg: None,
            units,
            is_comp
        }
    }

    fn req_id(name: &str, id: &str, is_comp: bool) -> Self {
        let id_reg = Regex::new(id).unwrap();
        let units: Vec<Unit> = Vec::new();
        Requirement {
            name: name.to_string(),
            name_reg: None,
            id_reg: Some(id_reg),
            units,
            is_comp
        }
    }

    fn req_none(name: &str, is_comp: bool) -> Self {
        let units: Vec<Unit> = Vec::new();
        Requirement {
            name: name.to_string(),
            name_reg: None,
            id_reg: None,
            units,
            is_comp
        }
    }

    fn check(&self, unit: &Unit) -> bool{
        let name_match = match self.name_reg {
            Some(ref reg) => reg.is_match(&unit.unit_name),
            None => true,
        };
        let id_match = match self.id_reg {
            Some(ref reg) => reg.is_match(&unit.unit_id),
            None => true,
        };
        return name_match && id_match;
    }

    fn print_units(&self, group_name: &str, verbose: bool) {
        if self.units.len() == 0 && self.is_comp {
            println!("{}/必修: \x1b[31m---\x1b[m         {}", group_name, self.name);
        } else if self.name == "pe1" && self.units.len() < 2 {
            println!("{}/必修: \x1b[31m---\x1b[m         基礎体育", group_name);
        } else if self.name == "pe2" && self.units.len() < 2 {
            println!("{}/必修: \x1b[31m---\x1b[m         応用体育", group_name);
        }
        for unit in self.units.iter() {
            unit.print(group_name, self.is_comp, verbose);
        }
    }
}

pub struct RequirementGroup {
    group_name: String,
    reqs: Vec<Requirement>,
    pub sums: HashMap<String, f32>,
}

impl RequirementGroup {
    fn new(group_name: &str) -> Self {
        let reqs: Vec<Requirement> = Vec::new();
        let sums: HashMap<String, f32> = HashMap::new();
        RequirementGroup {
            group_name: group_name.to_string(),
            reqs,
            sums,
        }
    }

    fn push_yaml(&mut self, yamlvec: &Vec<Yaml>) {
        for yaml in yamlvec.iter() {
            let name = yaml["name"].as_str().unwrap();
            let is_comp = yaml["isCp"].as_bool().unwrap();
            match yaml["regtype"].as_str() {
                Some("name") => {
                    let reg = yaml["reg"].as_str().unwrap();
                    let req = Requirement::req_name(name, reg, is_comp);
                    self.reqs.push(req);
                }
                Some("id") => {
                    let reg = yaml["reg"].as_str().unwrap();
                    let req = Requirement::req_id(name, reg, is_comp);
                    self.reqs.push(req);
                }
                Some("none") => {
                    let req = Requirement::req_none(name, is_comp);
                    self.reqs.push(req);
                }
                _ => {
                    println!("error");
                }
            }
        }
    }

    pub fn new_yaml(yaml: &str) -> (Self, Self, Self, Self) {
        match YamlLoader::load_from_str(yaml) {
            Ok(yaml) => {
                let mut a_reqs: RequirementGroup = RequirementGroup::new("専門    ");
                a_reqs.push_yaml(&yaml[0]["a_reqs"].as_vec().unwrap());
                let mut b_reqs: RequirementGroup = RequirementGroup::new("専門基礎");
                b_reqs.push_yaml(&yaml[0]["b_reqs"].as_vec().unwrap());
                let mut c_reqs: RequirementGroup = RequirementGroup::new("共通基礎");
                c_reqs.push_yaml(&yaml[0]["c_reqs"].as_vec().unwrap());
                let mut c0_reqs: RequirementGroup = RequirementGroup::new("関連基礎");
                c0_reqs.push_yaml(&yaml[0]["c0_reqs"].as_vec().unwrap());
                return (a_reqs, b_reqs, c_reqs, c0_reqs);
            }
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }
    }

    pub fn push_units(&mut self, units: Vec<Unit>) {
        let mut unitscp = units;
        while unitscp.len() > 0 {
            let unit = unitscp.pop().unwrap();
            match self.reqs.iter_mut().find(|req| req.check(&unit)) {
                Some(req) => {
                    let sum: &mut f32 = self.sums.entry(req.name.clone()).or_insert(0.0);
                    if unit.grade_num > 0.0 {
                        *sum += unit.unit_num;
                    }
                    req.units.push(unit);
                },
                None => {
                    println!{"Not found in Requirement"};
                    unit.print(&self.group_name, false, false);
                },
            }
        }
    }

    pub fn print(&self, verbose: bool) {
        for req in self.reqs.iter() {
            req.print_units(&self.group_name, verbose);
        }
    }
}