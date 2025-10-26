pub struct Allergies(Vec<Allergen>);

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

impl TryFrom<u8> for Allergen {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Eggs),
            2 => Ok(Self::Peanuts),
            4 => Ok(Self::Shellfish),
            8 => Ok(Self::Strawberries),
            16 => Ok(Self::Tomatoes),
            32 => Ok(Self::Chocolate),
            64 => Ok(Self::Pollen),
            128 => Ok(Self::Cats),
            _ => Err("{value} is not a valid allergen.".into()),
        }
    }
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies(
            (0_u8..8)
                .filter_map(|shift| Allergen::try_from(score as u8 & (1 << shift)).ok())
                .collect(),
        )
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.0.contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.0.clone()
    }
}
/*
























// fn try_from(val: u8) -> Result<Allergen, String> {
    //     match val {
    //         1_u8 => Ok(Allergen::Eggs),
    //         2_u8 => Ok(Allergen::Peanuts),
    //         4_u8 => Ok(Allergen::Shellfish),
    //         8_u8 => Ok(Allergen::Strawberries),
    //         16_u8 => Ok(Allergen::Tomatoes),
    //         32_u8 => Ok(Allergen::Chocolate),
    //         64_u8 => Ok(Allergen::Pollen),
    //         128_u8 => Ok(Allergen::Cats),
    //         _ => Err(format!("{val} is not a valid allergen.")),
    //     }
    // }



























    // pub fn new(score: u32) -> Self {
    //     Allergies { score: score as u8 }
    // }
    //
    // pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
    //     self.score & (*allergen as u8) > 0
    // }
    //
    // pub fn allergies(&self) -> Vec<Allergen> {
    //     (0_u8..8)
    //         .filter_map(|n| Allergen::try_from(1 << n).ok())
    //         .filter(|a| self.is_allergic_to(a))
    //         .collect::<Vec<Allergen>>()
    // }
 */
