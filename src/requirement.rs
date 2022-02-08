use regex::Regex;
use super::unit::Unit;
use std::collections::HashMap;

extern crate yaml_rust;
use yaml_rust::{YamlLoader, YamlEmitter};

pub struct Requirement {
    name: String,
    name_reg: Option<Regex>,
    id_reg: Option<Regex>,
    pub units: Vec<Unit>,
    is_comp: bool,
}

impl Requirement {
    pub fn req_name(name: &str, reg: &str, is_comp: bool) -> Self {
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

    pub fn req_id(name: &str, id: &str, is_comp: bool) -> Self {
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

    pub fn req_none(name: &str, is_comp: bool) -> Self {
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

    fn add_req(&mut self, req: Requirement) {
        self.sums.insert(req.name.clone(), 0.0);
        self.reqs.push(req);
    }

    pub fn yaml2reqs(yaml: &str) -> (Self, Self, Self, Self) {
        match YamlLoader::load_from_str(yaml) {
            Ok(yaml) => {
                let mut yaml_vec = &yaml[0]["a_reqs"];
                let mut a_reqs: RequirementGroup = RequirementGroup::new("専門    ");
                for req in yaml_vec.as_vec().unwrap() {
                    let name = req["name"].as_str().unwrap();
                    let is_comp = req["isCp"].as_bool().unwrap();
                    let reg = req["reg"].as_str().unwrap();
                    match req["regtype"].as_str() {
                        Some("name") => {
                            let req2 = Requirement::req_name(name, reg, is_comp);
                            a_reqs.reqs.push(req2);
                        }
                        Some("id") => {
                            let req2 = Requirement::req_id(name, reg, is_comp);
                            a_reqs.reqs.push(req2);
                        }
                        Some("none") => {
                            let req2 = Requirement::req_none(name, is_comp);
                            a_reqs.reqs.push(req2);
                        }
                        _ => {
                            println!("error");
                        }
                    }
                }

                let mut yaml_vec = &yaml[0]["b_reqs"];
                let mut b_reqs: RequirementGroup = RequirementGroup::new("専門基礎");
                for req in yaml_vec["b_reqs"] {
                    let name = req["name"].as_str().unwrap();
                    let is_comp = req["isCp"].as_bool().unwrap();
                    let reg = req["reg"].as_str().unwrap();
                    match req["regtype"].as_str() {
                        Some("name") => {
                            let req2 = Requirement::req_name(name, reg, is_comp);
                            b_reqs.reqs.push(req2);
                        }
                        Some("id") => {
                            let req2 = Requirement::req_id(name, reg, is_comp);
                            b_reqs.reqs.push(req2);
                        }
                        Some("none") => {
                            let req2 = Requirement::req_none(name, is_comp);
                            b_reqs.reqs.push(req2);
                        }
                        _ => {
                            println!("error");
                        }
                    }
                }
                
                let mut yaml_vec = &yaml[0]["c_reqs"];
                let mut c_reqs: RequirementGroup = RequirementGroup::new("共通基礎");
                for req in yaml_vec["c_reqs"] {
                    let name = req["name"].as_str().unwrap();
                    let is_comp = req["isCp"].as_bool().unwrap();
                    let reg = req["reg"].as_str().unwrap();
                    match req["regtype"].as_str() {
                        Some("name") => {
                            let req2 = Requirement::req_name(name, reg, is_comp);
                            c_reqs.reqs.push(req2);
                        }
                        Some("id") => {
                            let req2 = Requirement::req_id(name, reg, is_comp);
                            c_reqs.reqs.push(req2);
                        }
                        Some("none") => {
                            let req2 = Requirement::req_none(name, is_comp);
                            c_reqs.reqs.push(req2);
                        }
                        _ => {
                            println!("error");
                        }
                    }
                }
                
                let mut yaml_vec = &yaml[0]["c0_reqs"];
                let mut c0_reqs: RequirementGroup = RequirementGroup::new("関連基礎");
                for req in yaml_vec["c0_reqs"] {
                    let name = req["name"].as_str().unwrap();
                    let is_comp = req["isCp"].as_bool().unwrap();
                    let reg = req["reg"].as_str().unwrap();
                    match req["regtype"].as_str() {
                        Some("name") => {
                            let req2 = Requirement::req_name(name, reg, is_comp);
                            c0_reqs.reqs.push(req2);
                        }
                        Some("id") => {
                            let req2 = Requirement::req_id(name, reg, is_comp);
                            c0_reqs.reqs.push(req2);
                        }
                        Some("none") => {
                            let req2 = Requirement::req_none(name, is_comp);
                            c0_reqs.reqs.push(req2);
                        }
                        _ => {
                            println!("error");
                        }
                    }
                }
                
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