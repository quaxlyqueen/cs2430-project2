pub struct Set {
    pub elements: Vec<bool>,
}

impl Set {
    pub fn from_vec(vec: Vec<bool>) -> Self {
        Self { elements: vec }
    }

    // Combine sets A and B.
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

    // Get the shared values between sets A and B.
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

    // Determine the set where set A has no ocommon elements with set B.
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

    // Calculate the union of A diff B and B diff A between two sets.
    pub fn symmetric_difference(&self, other: &Set) -> Set {
        let a_minus_b = self.difference(other);
        let b_minus_a = other.difference(self);
        a_minus_b.union(&b_minus_a)
    }

    // Format and display the set.
    pub fn display(&self) {
        for &element in &self.elements {
            print!("{}", if element { 1 } else { 0 })
        }
        println!();
    }
}
