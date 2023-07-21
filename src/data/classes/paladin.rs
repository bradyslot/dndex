#![allow(unused, non_upper_case_globals)]
use crate::models::classes::*;
use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
  pub static ref cleric: SRDClass<SRDPaladinAttributes> = SRDClass::<SRDPaladinAttributes> {
    hit_points: SRDClassHitPoints {
      hit_dice: 10,
      static_option: 6,
      desc: "**Hit Dice:** 1d10 per Paladin level\n**Hit Points at 1st Level:** 10 + your Constitution modifier\n**Hit Points at Higher Levels:** 1d10 (or 6) + your Constitution modifier per paladin level after 1st"
    },
    proficiencies: SRDClassProficiencies {
      armor: vec![
        SRDEquipment::Open5eCategory(SRDItem { key: "Armor", source: "armor", qty: 1 }),
        SRDEquipment::Open5eCategory(SRDItem { key: "Shield", source: "armor", qty: 1 }),
      ],
      weapons: vec![
        SRDEquipment::Open5eCategory(SRDItem { key: "Simple", source: "weapons", qty: 1 }),
        SRDEquipment::Open5eCategory(SRDItem { key: "Martial", source: "weapons", qty: 1 }),
      ],
      tools: vec![],
      saving_throws: vec![ "wisdom", "charisma" ],
      skills: SRDClassProficientSkills {
        choices: 2,
        options: vec![ "athletics", "insight", "intimidation", "medicine", "persuasion", "religion" ]
      },
      desc: "**Armor:** All armor, shields\n**Weapons:** Simple weapons, martial weapons\n**Tools:** None\n**Saving Throws:** Wisdom, Charisma\n**Skills:** Choose two from Athletics, Insight, Intimidation, Medicine, Persuasion, and Religion"
    },
    starting_equipment: SRDClassStartingEquipment {
      choice_1: vec![
        vec![
          SRDEquipment::Open5eCategory(SRDItem { key: "Martial", source: "weapons", qty: 1 }),
          SRDEquipment::Open5eItem(SRDItem { key: "shield", source: "armor", qty: 1 }),
        ],
        vec![
          SRDEquipment::Open5eCategory(SRDItem { key: "Martial", source: "weapons", qty: 2 }),
        ]
      ],
      choice_2: vec![
        vec![
          SRDEquipment::Open5eItem(SRDItem { key: "javelin", source: "weapons", qty: 5 })
        ],
        vec![
          SRDEquipment::Open5eCategory(SRDItem { key: "Simple Melee", source: "weapons", qty: 1 }),
        ]
      ],
      choice_3: vec![
        vec![
          SRDEquipment::DnDexItem(SRDItem { key: "priests_pack", source: "equipment_packs", qty: 1 }),
        ],
        vec![
          SRDEquipment::DnDexItem(SRDItem { key: "explorers_pack", source: "equipment_packs", qty: 1 }),
        ]
      ],
      choice_4: vec![],
      defaults: vec![
        SRDEquipment::Open5eItem(SRDItem { key: "chain-mail", source: "armor", qty: 1 }),
        SRDEquipment::DnDexItem(SRDItem { key: "holy_symbol", source: "adventuring_gear", qty: 1 }),
      ],
      desc: "You start with the following equipment, in addition to the equipment granted by your background:\n- *(a)* a martial weapon and a shield or *(b)* two martial weapons\n- *(a)* five javelins or *(b)* any simple melee weapon\n- *(a)* a priest's pack or *(b)* an explorer's pack\n- Chain mail and a holy symbol"
    },
    spellcasting: Some(SRDClassSpellcasting {
      ability: "charisma",
      desc: "By 2nd level, you have learned to draw on divine magic through meditation and prayer to cast spells as a cleric does.\n## Preparing and Casting Spells The Paladin table shows how many spell slots you have to cast your spells. To cast one of your paladin spells of 1st level or higher, you must expend a slot of the spell's level or higher. You regain all expended spell slots when you finish a long rest.\nYou prepare the list of paladin spells that are available for you to cast, choosing from the paladin spell list. When you do so, choose a number of paladin spells equal to your Charisma modifier + half your paladin level, rounded down (minimum of one spell). The spells must be of a level for which you have spell slots.\nFor example, if you are a 5th-level paladin, you have four 1st-level and two 2nd-level spell slots. With a Charisma of 14, your list of prepared spells can include four spells of 1st or 2nd level, in any combination. If you prepare the 1st-level spell cure wounds, you can cast it using a 1st-level or a 2nd- level slot. Casting the spell doesn't remove it from your list of prepared spells.\nYou can change your list of prepared spells when you finish a long rest. Preparing a new list of paladin spells requires time spent in prayer and meditation: at least 1 minute per spell level for each spell on your list.\n## Spellcasting Ability\nCharisma is your spellcasting ability for your paladin spells, since their power derives from the strength of your convictions. You use your Charisma whenever a spell refers to your spellcasting ability. In addition, you use your Charisma modifier when setting the saving throw DC for a paladin spell you cast and when making an attack roll with one.\n**Spell save DC** = 8 + your proficiency bonus + your Charisma modifier\n**Spell attack modifier** = your proficiency bonus + your Charisma modifier\n## Spellcasting Focus\nYou can use a holy symbol as a spellcasting focus for your paladin spells.",
      at_level: 2,
    }),
    levels: SRDClassLevels::<SRDPaladinAttributes> {
      level_1: SRDPaladinAttributes {
        level: 1,
        features: vec![
          SRDClassLevelFeature { key: "divine_sense", name: None },
          SRDClassLevelFeature { key: "lay_on_hands", name: None }
        ],
        spells_known: 4,
        spell_slots: [ 0, 0, 0, 0, 0, 0, 0, 0, 0 ]
      },
      level_2: SRDPaladinAttributes {
        level: 2,
        features: vec![
          SRDClassLevelFeature { key: "divine_smite", name: None },
          SRDClassLevelFeature { key: "fighting_style", name: None },
        ],
        spells_known: 2,
        spell_slots: [ 2, 0, 0, 0, 0, 0, 0, 0, 0 ]
      },
      level_3: SRDPaladinAttributes {
        level: 3,
        features: vec![
          SRDClassLevelFeature { key: "divine_health", name: None },
          SRDClassLevelFeature { key: "sacred_oath", name: None }
        ],
        spells_known: 3,
        spell_slots: [ 3, 0, 0, 0, 0, 0, 0, 0, 0 ]
      },
      level_4: SRDPaladinAttributes {
        level: 4,
        features: vec![
          SRDClassLevelFeature { key: "ability_score", name: None }
        ],
        spells_known: 3,
        spell_slots: [ 3, 0, 0, 0, 0, 0, 0, 0, 0 ]
      },
      level_5: SRDPaladinAttributes {
        level: 5,
        features: vec![
          SRDClassLevelFeature { key: "extra_attack", name: None }
        ],
        spells_known: 4,
        spell_slots: [ 4, 2, 0, 0, 0, 0, 0, 0, 0 ]
      },
      level_6: SRDPaladinAttributes {
        level: 6,
        features: vec![
          SRDClassLevelFeature { key: "aura_of_protection", name: None }
        ],
        spells_known: 4,
        spell_slots: [ 4, 2, 0, 0, 0, 0, 0, 0, 0 ]
      },
      level_7: SRDPaladinAttributes {
        level: 7,
        features: vec![
          SRDClassLevelFeature { key: "sacred_oath", name: Some("Sacred Oath Feature") }
        ],
        spells_known: 5,
        spell_slots: [ 4, 3, 0, 0, 0, 0, 0, 0, 0 ]
      },
      level_8: SRDPaladinAttributes {
        level: 8,
        features: vec![
          SRDClassLevelFeature { key: "ability_score", name: None }
        ],
        spells_known: 5,
        spell_slots: [ 4, 3, 0, 0, 0, 0, 0, 0, 0 ]
      },
      level_9: SRDPaladinAttributes {
        level: 9,
        features: vec![],
        spells_known: 6,
        spell_slots: [ 4, 3, 2, 0, 0, 0, 0, 0, 0 ]
      },
      level_10: SRDPaladinAttributes {
        level: 10,
        features: vec![
          SRDClassLevelFeature { key: "aura_of_courage", name: None }
        ],
        spells_known: 6,
        spell_slots: [ 4, 3, 2, 0, 0, 0, 0, 0, 0 ]
      },
      level_11: SRDPaladinAttributes {
        level: 11,
        features: vec![
          SRDClassLevelFeature { key: "improved_divine_smite", name: Some("Improved Divine Smite") }
        ],
        spells_known: 7,
        spell_slots: [ 4, 3, 3, 0, 0, 0, 0, 0, 0 ]
      },
      level_12: SRDPaladinAttributes {
        level: 12,
        features: vec![
          SRDClassLevelFeature { key: "ability_score", name: None }
        ],
        spells_known: 7,
        spell_slots: [ 4, 3, 3, 0, 0, 0, 0, 0, 0 ]
      },
      level_13: SRDPaladinAttributes {
        level: 13,
        features: vec![],
        spells_known: 8,
        spell_slots: [ 4, 3, 3, 1, 0, 0, 0, 0, 0 ]
      },
      level_14: SRDPaladinAttributes {
        level: 14,
        features: vec![
          SRDClassLevelFeature { key: "cleansing_touch", name: None }
        ],
        spells_known: 8,
        spell_slots: [ 4, 3, 3, 1, 0, 0, 0, 0, 0 ]
      },
      level_15: SRDPaladinAttributes {
        level: 15,
        features: vec![
          SRDClassLevelFeature { key: "sacred_oath", name: Some("Sacred Oath Feature") }
        ],
        spells_known: 9,
        spell_slots: [ 4, 3, 3, 2, 0, 0, 0, 0, 0 ]
      },
      level_16: SRDPaladinAttributes {
        level: 16,
        features: vec![
          SRDClassLevelFeature { key: "ability_score", name: None }
        ],
        spells_known: 9,
        spell_slots: [ 4, 3, 3, 2, 0, 0, 0, 0, 0 ]
      },
      level_17: SRDPaladinAttributes {
        level: 17,
        features: vec![],
        spells_known: 10,
        spell_slots: [ 4, 3, 3, 3, 1, 0, 0, 0, 0 ]
      },
      level_18: SRDPaladinAttributes {
        level: 18,
        features: vec![
          SRDClassLevelFeature { key: "aura_of_protection", name: Some("Improved Aura of Protection") },
          SRDClassLevelFeature { key: "aura_of_courage", name: Some("Improved Aura of Courage") }
        ],
        spells_known: 10,
        spell_slots: [ 4, 3, 3, 3, 1, 0, 0, 0, 0 ]
      },
      level_19: SRDPaladinAttributes {
        level: 19,
        features: vec![
          SRDClassLevelFeature { key: "ability_score", name: None }
        ],
        spells_known: 11,
        spell_slots: [ 4, 3, 3, 3, 2, 0, 0, 0, 0 ]
      },
      level_20: SRDPaladinAttributes {
        level: 20,
        features: vec![
          SRDClassLevelFeature { key: "sacred_oath", name: Some("Sacred Oath Feature") }
        ],
        spells_known: 11,
        spell_slots: [ 4, 3, 3, 3, 2, 0, 0, 0, 0 ]
      }
    },
    features: HashMap::from([
      (
        "ability_score",
        SRDClassFeature {
          name: "Ability Score Improvement",
          desc: "When you reach 4th level, and again at 8th, 12th, 16th, and 19th level, you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1. As normal, you can't increase an ability score above 20 using this feature."
        },
      ),
      (
        "divine_sense",
        SRDClassFeature {
          name: "Divine Sense",
          desc: "The presence of strong evil registers on your senses like a noxious odor, and powerful good rings like heavenly music in your ears. As an action, you can open your awareness to detect such forces. Until the end of your next turn, you know the location of any celestial, fiend, or undead within 60 feet of you that is not behind total cover. You know the type (celestial, fiend, or undead) of any being whose presence you sense, but not its identity (the vampire Count Strahd von Zarovich, for instance). Within the same radius, you also detect the presence of any place or object that has been consecrated or desecrated, as with the *hallow* spell.\nYou can use this feature a number of times equal to 1 + your Charisma modifier. When you finish a long rest, you regain all expended uses."
        },
      ),
      (
        "lay_on_hands",
        SRDClassFeature {
          name: "Lay on Hands",
          desc: "Your blessed touch can heal wounds. You have a pool of healing power that replenishes when you take a long rest. With that pool, you can restore a total number of hit points equal to your paladin level × 5.\nAs an action, you can touch a creature and draw power from the pool to restore a number of hit points to that creature, up to the maximum amount remaining in your pool.\nAlternatively, you can expend 5 hit points from your pool of healing to cure the target of one disease or neutralize one poison affecting it. You can cure multiple diseases and neutralize multiple poisons with a single use of Lay on Hands, expending hit points separately for each one.\nThis feature has no effect on undead and constructs."
        },
      ),
      (
        "paladin_fighting_style",
        SRDClassFeature {
          name: "Fighting Style",
          desc: "At 2nd level, you adopt a style of fighting as your specialty. Choose one of the following options. You can't take a Fighting Style option more than once, even if you later get to choose again. \n ## Defense \n While you are wearing armor, you gain a +1 bonus to AC. \n ## Dueling \n When you are wielding a melee weapon in one hand and no other weapons, you gain a +2 bonus to damage rolls with that weapon. \n ## Great Weapon Fighting \n When you roll a 1 or 2 on a damage die for an attack you make with a melee weapon that you are wielding with two hands, you can reroll the die and must use the new roll. The weapon must have the two_handed or versatile property for you to gain this benefit. \n ## Protection \n When a creature you can see attacks a target other than you that is within 5 feet of you, you can use your reaction to impose disadvantage on the attack roll. You must be wielding a shield."
        },
      ),
      (
        "divine_smite",
        SRDClassFeature {
          name: "Divine Smite",
          desc: "Starting at 2nd level, when you hit a creature with a melee weapon attack, you can expend one spell slot to deal radiant damage to the target, in addition to the weapon's damage. The extra damage is 2d8 for a 1st_level spell slot, plus 1d8 for each spell level higher than 1st, to a maximum of 5d8. The damage increases by 1d8 if the target is an undead or a fiend.\nBy 11th level, you are so suffused with righteous might that all your melee weapon strikes carry divine power with them. Whenever you hit a creature with a melee weapon, the creature takes an extra 1d8 radiant damage. If you also use your Divine Smite with an attack, you add this damage to the extra damage of your Divine Smite."
        },
      ),
      (
        "divine_health",
        SRDClassFeature {
          name: "Divine Health",
          desc: "By 3rd level, the divine magic flowing through you makes you immune to disease."
        },
      ),
      (
        "sacred_oath",
        SRDClassFeature {
          name: "Sacred Oath",
          desc: "When you reach 3rd level, you swear the oath that binds you as a paladin forever. Up to this time you have been in a preparatory stage, committed to the path but not yet sworn to it. Now you choose the Oath of Devotion, the Oath of the Ancients, or the Oath of Vengeance, all detailed at the end of the class description.\nYour choice grants you features at 3rd level and again at 7th, 15th, and 20th level. Those features include oath spells and the Channel Divinity feature.\n## Oath Spells\nEach oath has a list of associated spells. You gain access to these spells at the levels specified in the oath description. Once you gain access to an oath spell, you always have it prepared. Oath spells don't count against the number of spells you can prepare each day.\nIf you gain an oath spell that doesn't appear on the paladin spell list, the spell is nonetheless a paladin spell for you.\n## Channel Divinity\nYour oath allows you to channel divine energy to fuel magical effects. Each Channel Divinity option provided by your oath explains how to use it.\nWhen you use your Channel Divinity, you choose which option to use. You must then finish a short or long rest to use your Channel Divinity again.\nSome Channel Divinity effects require saving throws. When you use such an effect from this class, the DC equals your paladin spell save DC.\n\n> ## Breaking Your Oath\n> A paladin tries to hold to the highest standards of conduct, but even the most virtuous paladin is fallible. Sometimes the right path proves too demanding, sometimes a situation calls for the lesser of two evils, and sometimes the heat of emotion causes a paladin to transgress his or her oath.\n> A paladin who has broken a vow typically seeks absolution from a cleric who shares his or her faith or from another paladin of the same order. The paladin might spend an all_ night vigil in prayer as a sign of penitence, or undertake a fast or similar act of self_denial. After a rite of confession and forgiveness, the paladin starts fresh.\n> If a paladin willfully violates his or her oath and shows no sign of repentance, the consequences can be more serious. At the GM's discretion, an impenitent paladin might be forced to abandon this class and adopt another.\n\nBecoming a paladin involves taking vows that commit the paladin to the cause of righteousness, an active path of fighting wickedness. The final oath, taken when he or she reaches 3rd level, is the culmination of all the paladin's training. Some characters with this class don't consider themselves true paladins until they have reached 3rd level and made this oath. For others, the actual swearing of the oath is a formality, an official stamp on what has always been true in the paladin's heart."
        },
      ),
      (
        "paladin_extra_attack",
        SRDClassFeature {
          name: "Extra Attack",
          desc: "Beginning at 5th level, you can attack twice, instead of once, whenever you take the Attack action on your turn."
        }
      ),
    ])
  };
}
