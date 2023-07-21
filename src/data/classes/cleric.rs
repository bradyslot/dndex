#![allow(unused, non_upper_case_globals)]
use crate::models::classes::*;
use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
  pub static ref cleric: SRDClass<SRDClericAttributes> = SRDClass::<SRDClericAttributes> {
    hit_points: SRDClassHitPoints {
      hit_dice: 8,
      static_option: 5,
      desc: "**Hit Dice:** 1d8 per cleric level\n**Hit Points at 1st Level:** 8 + your Constitution modifier\n**Hit Points at Higher Levels:** 1d8 (or 5) + your Constitution modifier per cleric level after 1st"
    },
    proficiencies: SRDClassProficiencies {
      armor: vec![
        SRDEquipment::Open5eCategory(SRDItem { key: "Light", source: "armor", qty: 0 }),
        SRDEquipment::Open5eCategory(SRDItem { key: "Medium", source: "armor", qty: 0 }),
        SRDEquipment::Open5eCategory(SRDItem { key: "Shield", source: "armor", qty: 0 }),
      ],
      weapons: vec![
        SRDEquipment::Open5eCategory(SRDItem { key: "Simple", source: "weapons", qty: 0 }),
      ],
      tools: vec![],
      saving_throws: vec![ "wisdom", "charisma" ],
      skills: SRDClassProficientSkills {
        choices: 2,
        options: vec![ "history", "insight", "medicine", "persuasion", "religion" ]
      },
      desc: "**Armor:** Light armor, medium armor, shields\n**Weapons:** Simple weapons\n**Tools:** None\n**Saving Throws:** Wisdom, Charisma\n**Skills:** Choose two from History, Insight, Medicine, Persuasion, and Religion"
    },
    starting_equipment: SRDClassStartingEquipment {
      choice_1: vec![
        vec![
          SRDEquipment::Open5eItem(SRDItem { key: "mace", source: "weapons", qty: 1 }),
        ],
        vec![
          SRDEquipment::Open5eItem(SRDItem { key: "warhammer", source: "weapons", qty: 1 }),
        ]
      ],
      choice_2: vec![
        vec![
          SRDEquipment::Open5eItem(SRDItem { key: "crossbow-light", source: "weapons", qty: 1 }),
          SRDEquipment::CustomItem(SRDCustomItem { name: "Bolts", qty: 20 }),
        ],
        vec![
          SRDEquipment::Open5eCategory(SRDItem { key: "Simple", source: "weapons", qty: 1 }),
        ]
      ],
      choice_3: vec![
        vec![
          SRDEquipment::Open5eItem(SRDItem { key: "scale-mail", source: "amor", qty: 1 }),
        ],
        vec![
          SRDEquipment::Open5eItem(SRDItem { key: "leather", source: "amor", qty: 1 }),
        ],
        vec![
          SRDEquipment::Open5eItem(SRDItem { key: "chain-mail", source: "amor", qty: 1 }),
        ]
      ],
      choice_4: vec![
        vec![
          SRDEquipment::DnDexItem(SRDItem { key: "priests_pack", source: "equipment_packs", qty: 1 }),
        ],
        vec![
          SRDEquipment::DnDexItem(SRDItem { key: "explorers_pack", source: "equipment_packs", qty: 1 }),
        ]
      ],
      defaults: vec![
        SRDEquipment::Open5eItem(SRDItem { key: "shield", source: "amor", qty: 1 }),
        SRDEquipment::DnDexItem(SRDItem { key: "holy_symbol", source: "adventuring_gear", qty: 1 }),
      ],
      desc: "You start with the following equipment, in addition to the equipment granted by your background:\n- *(a)* a mace or *(b)* a warhammer (if proficient)\n- *(a)* scale mail, *(b)* leather armor, or (c) chain mail (if proficient)\n- *(a)* a light crossbow and 20 bolts or *(b)* any simple weapon\n- *(a)* a priest's pack or *(b)* an explorer's pack\nA shield and a holy symbol"
    },
    spellcasting: Some(SRDClassSpellcasting {
      ability: "wisdom",
      desc: "As a conduit for divine power, you can cast cleric spells.\n## Cantrips At 1st level, you know three cantrips of your choice from the cleric spell list. You learn additional cleric cantrips of your choice at higher levels, as shown in the Cantrips Known column of the Cleric table.\n## Preparing and Casting Spells The Cleric table shows how many spell slots you have to cast your spells of 1st level and higher. To cast one of these spells, you must expend a slot of the spell's level or higher. You regain all expended spell slots when you finish a long rest.\nYou prepare the list of cleric spells that are available for you to cast, choosing from the cleric spell list. When you do so, choose a number of cleric spells equal to your Wisdom modifier + your cleric level (minimum of one spell). The spells must be of a level for which you have spell slots.\nFor example, if you are a 3rd-level cleric, you have four 1st-level and two 2nd-level spell slots. With a Wisdom of 16, your list of prepared spells can include six spells of 1st or 2nd level, in any combination. If you prepare the 1st-level spell cure wounds, you can cast it using a 1st-level or 2nd-level slot. Casting the spell doesn't remove it from your list of prepared spells.\nYou can change your list of prepared spells when you finish a long rest. Preparing a new list of cleric spells requires time spent in prayer and meditation: at least 1 minute per spell level for each spell on your list.\n## Spellcasting Ability Wisdom is your spellcasting ability for your cleric spells. The power of your spells comes from your devotion to your deity. You use your Wisdom whenever a cleric spell refers to your spellcasting ability. In addition, you use your Wisdom modifier when setting the saving throw DC for a cleric spell you cast and when making an attack roll with one.\n**Spell save DC** = 8 + your proficiency bonus + your Wisdom modifier\n**Spell attack modifier** = your proficiency bonus + your Wisdom modifier\n## Ritual Casting You can cast a cleric spell as a ritual if that spell has the ritual tag and you have the spell prepared.\n## Spellcasting Focus You can use a holy symbol (see [Adventuring Gear]({{ base_url }}/equipment/adventuring-gear)) as a spellcasting focus for your cleric spells.",
      at_level: 1,
    }),
    levels: SRDClassLevels::<SRDClericAttributes> {
      level_1: SRDClericAttributes {
        level: 1,
        features: vec![
          SRDClassLevelFeature { key: "spellcasting", name: None },
          SRDClassLevelFeature { key: "divine_domain", name: None },
        ],
        cantrips_known: 3,
        spell_slots: [ 2, 0, 0, 0, 0, 0, 0, 0, 0 ]
      },
      level_2: SRDClericAttributes {
        level: 2,
        features: vec![
          SRDClassLevelFeature { key: "channel_divinity", name: Some("Channel Divinity (1/rest)") },
          SRDClassLevelFeature { key: "divine_domain", name: Some("Divine Domain Feature") }
        ],
        cantrips_known: 3,
        spell_slots: [ 3, 0, 0, 0, 0, 0, 0, 0, 0 ]
      },
      level_3: SRDClericAttributes {
        level: 3,
        features: vec![],
        cantrips_known: 3,
        spell_slots: [ 4, 2, 0, 0, 0, 0, 0, 0, 0 ]
      },
      level_4: SRDClericAttributes {
        level: 4,
        features: vec![
          SRDClassLevelFeature { key: "ability_score", name: None }
        ],
        cantrips_known: 4,
        spell_slots: [ 4, 3, 0, 0, 0, 0, 0, 0, 0 ]
      },
      level_5: SRDClericAttributes {
        level: 5,
        features: vec![
          SRDClassLevelFeature { key: "destroy_undead", name: Some("Destroy Undead (CR 1/2)") }
        ],
        cantrips_known: 4,
        spell_slots: [ 4, 3, 2, 0, 0, 0, 0, 0, 0 ]
      },
      level_6: SRDClericAttributes {
        level: 6,
        features: vec![
          SRDClassLevelFeature { key: "channel_divinity", name: Some("Channel Divinity (2/rest)") },
          SRDClassLevelFeature { key: "divine_domain", name: Some("Divine Domain Feature") }
        ],
        cantrips_known: 4,
        spell_slots: [ 4, 3, 3, 0, 0, 0, 0, 0, 0 ]
      },
      level_7: SRDClericAttributes {
        level: 7,
        features: vec![],
        cantrips_known: 4,
        spell_slots: [ 4, 3, 3, 1, 0, 0, 0, 0, 0 ]
      },
      level_8: SRDClericAttributes {
        level: 8,
        features: vec![
          SRDClassLevelFeature { key: "ability_score", name: None },
          SRDClassLevelFeature { key: "destroy_undead", name: Some("Destroy Undead (CR 1)") },
          SRDClassLevelFeature { key: "divine_domain", name: Some("Divine Domain Feature") }
        ],
        cantrips_known: 4,
        spell_slots: [ 4, 3, 3, 2, 0, 0, 0, 0, 0 ]
      },
      level_9: SRDClericAttributes {
        level: 9,
        features: vec![],
        cantrips_known: 4,
        spell_slots: [ 4, 3, 3, 3, 1, 0, 0, 0, 0 ]
      },
      level_10: SRDClericAttributes {
        level: 10,
        features: vec![
          SRDClassLevelFeature { key: "divine_intervention", name: None }
        ],
        cantrips_known: 5,
        spell_slots: [ 4, 3, 3, 3, 2, 0, 0, 0, 0 ]
      },
      level_11: SRDClericAttributes {
        level: 11,
        features: vec![
          SRDClassLevelFeature { key: "destroy_undead", name: Some("Destroy Undead (CR 2)") }
        ],
        cantrips_known: 5,
        spell_slots: [ 4, 3, 3, 3, 2, 1, 0, 0, 0 ]
      },
      level_12: SRDClericAttributes {
        level: 12,
        features: vec![
          SRDClassLevelFeature { key: "ability_score", name: None }
        ],
        cantrips_known: 5,
        spell_slots: [ 4, 3, 3, 3, 2, 1, 0, 0, 0 ]
      },
      level_13: SRDClericAttributes {
        level: 13,
        features: vec![],
        cantrips_known: 5,
        spell_slots: [ 4, 3, 3, 3, 2, 1, 1, 0, 0 ]
      },
      level_14: SRDClericAttributes {
        level: 14,
        features: vec![
          SRDClassLevelFeature { key: "destroy_undead", name: Some("Destroy Undead (CR 3)") }
        ],
        cantrips_known: 5,
        spell_slots: [ 4, 3, 3, 3, 2, 1, 1, 0, 0 ]
      },
      level_15: SRDClericAttributes {
        level: 15,
        features: vec![],
        cantrips_known: 5,
        spell_slots: [ 4, 3, 3, 3, 2, 1, 1, 1, 0 ]
      },
      level_16: SRDClericAttributes {
        level: 16,
        features: vec![
          SRDClassLevelFeature { key: "ability_score", name: None }
        ],
        cantrips_known: 5,
        spell_slots: [ 4, 3, 3, 3, 2, 1, 1, 1, 0 ]
      },
      level_17: SRDClericAttributes {
        level: 17,
        features: vec![
          SRDClassLevelFeature { key: "destroy_undead", name: Some("Destroy Undead (CR 4)") },
          SRDClassLevelFeature { key: "divine_domain", name: Some("Divine Domain Feature") }
        ],
        cantrips_known: 5,
        spell_slots: [ 4, 3, 3, 3, 2, 1, 1, 1, 1 ]
      },
      level_18: SRDClericAttributes {
        level: 18,
        features: vec![
          SRDClassLevelFeature { key: "channel_divinity", name: Some("Channel Divinity (3/rest)") }
        ],
        cantrips_known: 5,
        spell_slots: [ 4, 3, 3, 3, 3, 1, 1, 1, 1 ]
      },
      level_19: SRDClericAttributes {
        level: 19,
        features: vec![
          SRDClassLevelFeature { key: "ability_score", name: None }
        ],
        cantrips_known: 5,
        spell_slots: [ 4, 3, 3, 3, 3, 2, 1, 1, 1 ]
      },
      level_20: SRDClericAttributes {
        level: 20,
        features: vec![
          SRDClassLevelFeature { key: "divine_intervention", name: None }
        ],
        cantrips_known: 5,
        spell_slots: [ 4, 3, 3, 3, 3, 2, 2, 1, 1 ]
      },
    },
    features: HashMap::from([
      (
        "ability_score",
        SRDClassFeature {
          name: "Ability Score Improvement",
          desc: "When you reach 4th level, and again at 8th, 12th, 16th, and 19th level, you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1. As normal, you can't increase an ability score above 20 using this feature."
        }
      ),
      (
        "divine_domain",
        SRDClassFeature {
          name: "Devine Domain",
          desc: "Choose one domain related to your deity: Knowledge, Life, Light, Nature, Tempest, Trickery, or War. Each domain is detailed at the end of the class description, and each one provides examples of gods associated with it. Your choice grants you domain spells and other features when you choose it at 1st level. It also grants you additional ways to use Channel Divinity when you gain that feature at 2nd level, and additional benefits at 6th, 8th, and 17th levels.\n## Domain Spells Each domain has a list of spells that you gain at the cleric levels noted in the domain description. Once you gain a domain spell, you always have it prepared, and it doesn't count against the number of spells you can prepare each day.\nIf you have a domain spell that doesn't appear on the cleric spell list, the spell is nonetheless a cleric spell for you."
        },
      ),
      (
        "channel_divinity",
        SRDClassFeature {
          name: "Channel Divinity",
          desc: "At 2nd level, you gain the ability to channel divine energy directly from your deity, using that energy to fuel magical effects. You start with two such effects: Turn Undead and an effect determined by your domain. Some domains grant you additional effects as you advance in levels, as noted in the domain description.\nWhen you use your Channel Divinity, you choose which effect to create. You must then finish a short or long rest to use your Channel Divinity again.\nSome Channel Divinity effects require saving throws. When you use such an effect from this class, the DC equals your cleric spell save DC.\nBeginning at 6th level, you can use your Channel\nDivinity twice between rests, and beginning at 18th level, you can use it three times between rests. When you finish a short or long rest, you regain your expended uses.\n## Channel Divinity: Turn Undead As an action, you present your holy symbol and speak a prayer censuring the undead. Each undead that can see or hear you within 30 feet of you must make a Wisdom saving throw. If the creature fails its saving throw, it is turned for 1 minute or until it takes any damage.\nA turned creature must spend its turns trying to move as far away from you as it can, and it can't willingly move to a space within 30 feet of you. It also can't take reactions. For its action, it can use only the Dash action or try to escape from an effect that prevents it from moving. If there's nowhere to move, the creature can use the Dodge action."
        },
      ),
      (
        "destroy_undead",
        SRDClassFeature {
          name: "Destroy Undead",
          desc: "Starting at 5th level, when an undead fails its saving throw against your Turn Undead feature, the creature is instantly destroyed if its challenge rating is at or below a certain threshold, as shown in the Destroy Undead table.\n**Destroy Undeada (table)**\n| Cleric Level | Destroys Undead of CR… |\n|_|-|\n| 5th | 1/2 or lower |\n| 8th | 1 or lower |\n| 11th | 2 or lower |\n| 14th | 3 or lower |\n| 17th | 4 or lower |\n"
        },
      ),
      (
        "divine_intervention",
        SRDClassFeature {
          name: "Divine Intervention",
          desc: "Beginning at 10th level, you can call on your deity to intervene on your behalf when your need is great.\nImploring your deity's aid requires you to use your action. Describe the assistance you seek, and roll percentile dice. If you roll a number equal to or lower than your cleric level, your deity intervenes. The GM chooses the nature of the intervention; the effect of any cleric spell or cleric domain spell would be appropriate.\nIf your deity intervenes, you can't use this feature again for 7 days. Otherwise, you can use it again after you finish a long rest.\nAt 20th level, your call for intervention succeeds automatically, no roll required. "
        }
      )
    ])
  };
}
