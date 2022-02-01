use regex::Regex;
use super::unit::Unit;
use std::collections::HashMap;

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

    pub fn req_id(name: &str, id: &str, matchlen: u8, is_comp: bool) -> Self {
        let id_reg = Regex::new(&format!(r"^{}.{{{}}}$", id, 7 - matchlen)).unwrap();
        let units: Vec<Unit> = Vec::new();
        Requirement {
            name: name.to_string(),
            name_reg: None,
            id_reg: Some(id_reg),
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
    pub fn new(group_name: &str) -> Self {
        let reqs: Vec<Requirement> = Vec::new();
        let sums: HashMap<String, f32> = HashMap::new();
        RequirementGroup {
            group_name: group_name.to_string(),
            reqs,
            sums,
        }
    }

    pub fn comp_reqs(group_name: &str, names: Vec<&str>) -> Self {
        let mut reqs: Vec<Requirement> = Vec::new();
        let mut sums: HashMap<String, f32> = HashMap::new();
        for name in names {
            reqs.push(Requirement::req_name(name, &format!(r"^{}$", name), true));
            sums.insert(name.to_string(), 0.0);
        }
        return RequirementGroup{group_name: group_name.to_string(), reqs, sums};
    }

    pub fn add_req(&mut self, req: Requirement) {
        self.sums.insert(req.name.clone(), 0.0);
        self.reqs.push(req);
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