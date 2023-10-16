pub struct Set {
    pub elements: Vec<bool>,
}

impl Set {
    pub fn from_vec(vec: Vec<bool>) -> Self {
        Self { elements: vec }
    }

    pub fn complement(&self) -> Self {
        Set {
            elements: self.elements.iter().map(|&x| !x).collect(),
        }
    }

    pub fn union(&self, other: &Set) -> Set {
        Set {
            elements: self
                .elements
                .iter()
                .zip(&other.elements)
                .map(|(&x, &y)| x || y)
                .collect(),
        }
    }

    pub fn intersection(&self, other: &Set) -> Set {
        Set {
            elements: self
                .elements
                .iter()
                .zip(&other.elements)
                .map(|(&x, &y)| x && y)
                .collect(),
        }
    }

    pub fn difference(&self, other: &Set) -> Set {
        Set {
            elements: self
                .elements
                .iter()
                .zip(&other.elements)
                .map(|(&x, &y)| x && !y)
                .collect(),
        }
    }

    pub fn symmetric_difference(&self, other: &Set) -> Set {
        let a_minus_b = self.difference(other);
        let b_minus_a = other.difference(self);
        a_minus_b.union(&b_minus_a)
    }

    pub fn display(&self) {
        for &element in &self.elements {
            print!("{}", if element { 1 } else { 0 })
        }
        println!();
    }
}
