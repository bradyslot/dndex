pub fn calc_proficiency_bonus(level: u8) -> i8 {
    2 + (level as i8 - 1) / 4
}

pub fn calc_base_modifier(ability: u8) -> i8 {
    (ability as i8 - 10) / 2
}
