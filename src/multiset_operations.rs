pub struct MultiSet {
    pub elements: Vec<usize>,
}

impl MultiSet {
    pub fn from_vec(vec: Vec<usize>) -> Self {
        MultiSet { elements: vec }
    }

    // Get the combination of values between the two multisets.
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

    // Get the minimum count of an element between two multisets.
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

    // Subtract the count of the element in multiset B from multiset A.
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

    // Add the count of the elements between two multisets
    pub fn sum(&self) -> usize {
        self.elements.iter().sum()
    }

    // List each element value and the count of that element.
    pub fn display(&self) {
        for (index, &count) in self.elements.iter().enumerate() {
            println!("Element {index}: Count {count}");
        }
    }
}
