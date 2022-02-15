pub struct Allergies {
    values: Vec<Allergen>,
}

#[derive(Debug, PartialEq)]
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
        println!("Called from Allergies::new({})", &score);
        // unimplemented!("");
        match score {
            scr if scr == 1 => Allergies {
                values: vec![Allergen::Eggs],
            },
            scr if scr >= 256 => Allergies {
                values: vec![Allergen::Eggs],
            },
            scr if scr < 1 => Allergies { values: vec![] },
            scr if scr > 1 && scr < 256 => {
                println!("\t scr > 1 && scr < 256 ");
                /* Algorithm:
                loop until count of allergies
                    if score related to an Allergen
                        - insert related allergen to allergies list

                    else:
                        - if allergen(count) not less than score
                            - insert allergen(count) to allergies
                            - minus allerten(count) score from score

                */
                // for i in Allergen.into_iter().enumerate(){
                // }
                // let allergies_count = 8;
                // for i in allergies_count..0 {
                //     dbg!(i.clone());
                //     //==
                // }

                let result = Allergies { values: vec![] };
                // ===
                result
            }
            _ => Allergies { values: vec![] },
        }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.values.contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let list: Vec<Allergen> = vec![
            Allergen::Eggs,
            Allergen::Peanuts,
            Allergen::Shellfish,
            Allergen::Strawberries,
            Allergen::Tomatoes,
            Allergen::Chocolate,
            Allergen::Pollen,
            Allergen::Cats,
        ];
        list
    }
}
