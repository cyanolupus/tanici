use regex::Regex;
use super::unit::Unit;
use std::collections::HashMap;

extern crate yaml_rust;
use yaml_rust::Yaml;

pub struct UnitGroup {
    name: String,
    name_reg: Option<Regex>,
    id_reg: Option<Regex>,
    pub units: Vec<Unit>,
    is_comp: bool,
}

impl UnitGroup {
    fn group_name(name: &str, reg: &str, is_comp: bool) -> Self {
        let name_reg = Regex::new(reg).unwrap();
        let units: Vec<Unit> = Vec::new();
        UnitGroup {
            name: name.to_string(),
            name_reg: Some(name_reg),
            id_reg: None,
            units,
            is_comp
        }
    }

    fn group_id(name: &str, id: &str, is_comp: bool) -> Self {
        let id_reg = Regex::new(id).unwrap();
        let units: Vec<Unit> = Vec::new();
        UnitGroup {
            name: name.to_string(),
            name_reg: None,
            id_reg: Some(id_reg),
            units,
            is_comp
        }
    }

    fn group_none(name: &str, is_comp: bool) -> Self {
        let units: Vec<Unit> = Vec::new();
        UnitGroup {
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
        if self.name == "pe1" && self.units.len() < 2 && self.is_comp {
            println!("{}/必修: \x1b[31m---\x1b[m         基礎体育", group_name);
        } else if self.name == "pe2" && self.units.len() < 2 && self.is_comp {
            println!("{}/必修: \x1b[31m---\x1b[m         応用体育", group_name);
        } else if self.name == "pe3" && self.units.len() < 2 && self.is_comp {
            println!("{}/必修: \x1b[31m---\x1b[m         発展体育", group_name);
        } else if self.units.len() == 0 && self.is_comp {
            println!("{}/必修: \x1b[31m---\x1b[m         {}", group_name, self.name);
        }
        
        for unit in self.units.iter() {
            unit.print(group_name, self.is_comp, verbose);
        }
    }
}

pub struct UnitGroupMap {
    group_name: String,
    groups: Vec<UnitGroup>,
    pub sums: HashMap<String, f32>,
}

impl UnitGroupMap {
    pub fn new(group_name: &str) -> Self {
        let groups: Vec<UnitGroup> = Vec::new();
        let sums: HashMap<String, f32> = HashMap::new();
        UnitGroupMap {
            group_name: group_name.to_string(),
            groups,
            sums,
        }
    }

    pub fn push_yaml(&mut self, yaml: &Vec<Yaml>, groups_name: &str) {
        match &yaml[0][groups_name].as_vec() {
            Some(groups) => {
                for group in groups.iter() {
                    let name = group["name"].as_str().unwrap();
                    let is_comp = match group["isCp"].as_bool() {
                        Some(is_comp) => is_comp,
                        None => false,
                    };
                    match group["regtype"].as_str() {
                        Some("name") => {
                            let reg = match group["reg"].as_str() {
                                Some(reg) => reg.to_string(),
                                None => format!("^{}$", name),
                            };
                            let req = UnitGroup::group_name(name, &reg, is_comp);
                            self.groups.push(req);
                        }
                        Some("id") => {
                            let reg = group["reg"].as_str().unwrap();
                            let req = UnitGroup::group_id(name, reg, is_comp);
                            self.groups.push(req);
                        }
                        Some("none") => {
                            let req = UnitGroup::group_none(name, is_comp);
                            self.groups.push(req);
                        }
                        _ => {
                            eprintln!("error");
                            std::process::exit(1);
                        }
                    }
                }
            }
            None => {}
        }
    }

    pub fn push_units(&mut self, units: Vec<Unit>) {
        let mut unitscp = units;
        while unitscp.len() > 0 {
            let unit = unitscp.pop().unwrap();
            match self.groups.iter_mut().find(|req| req.check(&unit)) {
                Some(req) => {
                    let sum: &mut f32 = self.sums.entry(req.name.clone()).or_insert(0.0);
                    if unit.grade_num > 0.0 {
                        *sum += unit.unit_num;
                    }
                    req.units.push(unit);
                },
                None => {
                    println!{"Not found in UnitGroup"};
                    unit.print(&self.group_name, false, false);
                },
            }
        }
    }

    pub fn print(&self, verbose: bool) {
        for req in self.groups.iter() {
            req.print_units(&self.group_name, verbose);
        }
    }
}