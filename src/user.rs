use super::unit::Unit;

pub struct User {
    pub units_a: Vec<Unit>,
    pub units_b: Vec<Unit>,
    pub units_c: Vec<Unit>,
    pub units_c0: Vec<Unit>,
    pub gpa: f32,
    pub gps: f32,
}

impl User {
    pub fn new(csv: String) -> Self {
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