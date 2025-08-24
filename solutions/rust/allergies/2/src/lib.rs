pub struct Allergies {
    score: u8,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
        Allergies { score: score as u8 }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        match allergen {
            Allergen::Eggs => 1 & self.score > 0,
            Allergen::Peanuts => 2 & self.score > 0,
            Allergen::Shellfish => 4 & self.score > 0,
            Allergen::Strawberries => 8 & self.score > 0,
            Allergen::Tomatoes => 16 & self.score > 0,
            Allergen::Chocolate => 32 & self.score > 0,
            Allergen::Pollen => 64 & self.score > 0,
            Allergen::Cats => 128 & self.score > 0,
        }
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        use Allergen::*;

        let allergens: Vec<Allergen> = vec![
            Eggs,
            Peanuts,
            Shellfish,
            Strawberries,
            Tomatoes,
            Chocolate,
            Pollen,
            Cats,
        ];

        allergens
            .iter()
            .filter_map(|allergen| {
                if self.is_allergic_to(allergen) {
                    Some(*allergen)
                } else {
                    None
                }
            })
            .collect()
    }
}
