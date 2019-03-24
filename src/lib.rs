pub struct Millionaire {
    address: String,
    net_worth: u64,
}

pub struct MillionairesProblem {
    millionaires: Vec<Millionaire>,
}

impl MillionairesProblem {
    pub fn new() -> MillionairesProblem {
        MillionairesProblem {
            millionaires: Vec::new(),
        }
    }
    pub fn add_millionaire(&mut self, address: String, net_worth: u64) {
        let Millionaire = Millionaire {
            address,
            net_worth,
        };
        self.millionaires.push(Millionaire);
    }
    pub fn compute_richest(&self) -> &str {
        match self.millionaires.iter().max_by_key(|m| m.net_worth) {
            Some(millionaire) => &millionaire.address[..],
            None => "None"
        }
    }
}
