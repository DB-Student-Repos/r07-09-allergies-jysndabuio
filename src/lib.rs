pub struct Allergies {
    score_allergies: u32
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
// Added Copy and Clone. If no COPY bitwise operation failed. 
// no implementation for `u32 & &Allergen`
// cannot move out of `*allergen` which is behind a shared reference

pub enum Allergen {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies { score_allergies: score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        // Use Bitwise AND operator 0 = allergic 
        self.score_allergies & *allergen as u32 != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        // Iterate over each allergen and check if the person is allergic
        let list_allergies = vec![
            Allergen::Eggs, Allergen::Peanuts, Allergen::Shellfish, Allergen::Strawberries,
            Allergen::Tomatoes, Allergen::Chocolate, Allergen::Pollen, Allergen::Cats
        ];
        list_allergies.iter().filter(|x| self.is_allergic_to(x)).cloned().collect()
    }
}
