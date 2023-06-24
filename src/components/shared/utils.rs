use fastrand::Rng;

pub fn calc_proficiency_bonus(level: u8) -> i8 {
    2 + (level as i8 - 1) / 4
}

pub fn calc_base_modifier(ability: u8) -> i8 {
    (ability as i8 - 10) / 2
}

pub fn is_yes_or_no(value: String) -> bool {
    value == "yes" || value == "no"
}

pub fn random_alpha_string(length: u8) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                             abcdefghijklmnopqrstuvwxyz\
                             0123456789";

    let mut rng = Rng::new();
    let random_string: String = (0..length)
        .map(|_| CHARSET[rng.usize(..CHARSET.len())] as char)
        .collect();

    random_string
}
