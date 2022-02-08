use std::collections::HashMap;

extern crate yaml_rust;
use yaml_rust::Yaml;

pub struct Req{
    desc: String,
    subreqs: Option<Vec<Req>>,
    groups: HashMap<String, f32>,
    min: f32,
    max: f32,
}

impl Req {
    fn recu_load(reqs_yaml: &Vec<Yaml>) -> Option<Vec<Req>> {
        let mut reqs: Vec<Req> = Vec::new();
        for req_yaml in reqs_yaml.iter() {
            let desc = req_yaml["desc"].as_str().unwrap();
            let min = req_yaml["min"].as_f64().unwrap();
            let max = match req_yaml["max"].as_f64() {
                Some(v) => v,
                None => -1.0,
            };
            let subreqs = match &req_yaml["subreqs"].as_vec() {
                Some(subreqs) => Req::recu_load(subreqs),
                None => None,
            };
            let mut groups = HashMap::new();
            match req_yaml["groups"].as_vec() {
                Some(groups_yaml) => {
                    for group_yaml in groups_yaml.iter() {
                        let name = group_yaml["name"].as_str().unwrap();
                        let max = group_yaml["max"].as_f64().unwrap();
                        groups.insert(name.to_string(), max as f32);
                    }
                },
                None => {},
            }
            reqs.push(Req{
                desc: desc.to_string(),
                subreqs,
                groups,
                min: min as f32,
                max: max as f32,
            });
        }
        return Some(reqs);
    }

    pub fn reqs_yaml(yaml: &Vec<Yaml>) -> Option<Vec<Req>> {
        match &yaml[0]["reqs"].as_vec() {
            Some(reqs_yaml) => Req::recu_load(reqs_yaml),
            None => None,
        }
    }

    fn print_cmp(left: f32, right: f32, label: &str) {
        let fail = "\x1b[31mfail\x1b[m";
        let pass = "\x1b[32mpass\x1b[m";
        println!("{}: {:>4}/{:>2}   {}", if left < right {fail} else {pass}, left, right, label);
    }

    pub fn check_req(&self, sums: &HashMap<String, f32>, verbose: bool) -> f32 {
        let mut result: f32 = 0.0;
        match &self.subreqs {
            Some(reqs) => {
                for req in reqs.iter() {
                    result += req.check_req(&sums, verbose);
                }
            },
            None => {},
        }
        for (group, max) in self.groups.iter() {
            let group_sum = sums.get(group).unwrap_or(&0.0);
            if *max < 0.0 {
                result += *group_sum;
            } else {
                result += group_sum.min(*max);
            }
        }
        if verbose {
            Req::print_cmp(result, self.min, self.desc.as_str());
        }
        if self.max < 0.0 {
            return result;
        } else {
            return result.min(self.max);
        }
    }
}