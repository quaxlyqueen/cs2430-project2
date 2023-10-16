pub struct MultiSet {
    pub elements: Vec<usize>,
}

impl MultiSet {
    pub fn from_vec(vec: Vec<usize>) -> Self {
        MultiSet { elements: vec }
    }

    pub fn union(&self, other: &MultiSet) -> MultiSet {
        MultiSet {
            elements: self
                .elements
                .iter()
                .zip(&other.elements)
                .map(|(&x, &y)| x + y)
                .collect(),
        }
    }

    pub fn intersection(&self, other: &MultiSet) -> MultiSet {
        MultiSet {
            elements: self
                .elements
                .iter()
                .zip(&other.elements)
                .map(|(&x, &y)| usize::min(x, y))
                .collect(),
        }
    }

    pub fn difference(&self, other: &MultiSet) -> MultiSet {
        MultiSet {
            elements: self
                .elements
                .iter()
                .zip(&other.elements)
                .map(|(&x, &y)| if x > y { x - y } else { 0 })
                .collect(),
        }
    }

    pub fn sum(&self) -> usize {
        self.elements.iter().sum()
    }

    pub fn display(&self) {
        for (index, &count) in self.elements.iter().enumerate() {
            println!("Element {}: Count {}", index, count);
        }
    }
}
