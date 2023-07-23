#![allow(unused, non_upper_case_globals)]
use crate::models::mechanics::*;
use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
  pub static ref advancement: SRDAdvancementLevels = SRDAdvancementLevels {
    level_1: SRDAdvancement { level: 1,  experience: 0,      proficiency_bonus: 2 },
    level_2: SRDAdvancement { level: 2,  experience: 300,    proficiency_bonus: 2 },
    level_3: SRDAdvancement { level: 3,  experience: 900,    proficiency_bonus: 2 },
    level_4: SRDAdvancement { level: 4,  experience: 2700,   proficiency_bonus: 2 },
    level_5: SRDAdvancement { level: 5,  experience: 6500,   proficiency_bonus: 3 },
    level_6: SRDAdvancement { level: 6,  experience: 14000,  proficiency_bonus: 3 },
    level_7: SRDAdvancement { level: 7,  experience: 23000,  proficiency_bonus: 3 },
    level_8: SRDAdvancement { level: 8,  experience: 14000,  proficiency_bonus: 3 },
    level_9: SRDAdvancement { level: 9,  experience: 23000,  proficiency_bonus: 4 },
    level_10: SRDAdvancement { level: 10, experience: 34000,  proficiency_bonus: 4 },
    level_11: SRDAdvancement { level: 11, experience: 48000,  proficiency_bonus: 4 },
    level_12: SRDAdvancement { level: 12, experience: 85000,  proficiency_bonus: 4 },
    level_13: SRDAdvancement { level: 13, experience: 100000, proficiency_bonus: 5 },
    level_14: SRDAdvancement { level: 14, experience: 120000, proficiency_bonus: 5 },
    level_15: SRDAdvancement { level: 15, experience: 140000, proficiency_bonus: 5 },
    level_16: SRDAdvancement { level: 16, experience: 165000, proficiency_bonus: 5 },
    level_17: SRDAdvancement { level: 17, experience: 225000, proficiency_bonus: 6 },
    level_18: SRDAdvancement { level: 18, experience: 265000, proficiency_bonus: 6 },
    level_19: SRDAdvancement { level: 19, experience: 305000, proficiency_bonus: 6 },
    level_20: SRDAdvancement { level: 20, experience: 355000, proficiency_bonus: 6 }
  };
}
