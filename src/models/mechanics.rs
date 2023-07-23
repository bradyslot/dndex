#![allow(non_snake_case, non_camel_case_types, clippy::similar_names)]

#[derive(PartialEq, Debug)]
pub struct SRDAdvancement {
    pub level: u8,
    pub experience: u32,
    pub proficiency_bonus: u8,
}

#[derive(PartialEq, Debug)]
pub struct SRDAdvancementLevels {
    pub level_1: SRDAdvancement,
    pub level_2: SRDAdvancement,
    pub level_3: SRDAdvancement,
    pub level_4: SRDAdvancement,
    pub level_5: SRDAdvancement,
    pub level_6: SRDAdvancement,
    pub level_7: SRDAdvancement,
    pub level_8: SRDAdvancement,
    pub level_9: SRDAdvancement,
    pub level_10: SRDAdvancement,
    pub level_11: SRDAdvancement,
    pub level_12: SRDAdvancement,
    pub level_13: SRDAdvancement,
    pub level_14: SRDAdvancement,
    pub level_15: SRDAdvancement,
    pub level_16: SRDAdvancement,
    pub level_17: SRDAdvancement,
    pub level_18: SRDAdvancement,
    pub level_19: SRDAdvancement,
    pub level_20: SRDAdvancement,
}
