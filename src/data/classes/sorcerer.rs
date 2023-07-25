#![allow(unused, non_upper_case_globals)]
use crate::models::classes::*;
use crate::models::equipment::*;
use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
  pub static ref sorcerer: SRDClass<SRDSorcererAttributes> = SRDClass::<SRDSorcererAttributes> {
    hit_points: SRDClassHitPoints {
      hit_dice: 6,
      static_option: 4,
      desc: "**Hit Dice:** 1d6 per Sorcerer level\n**Hit Points at 1st Level:** 6 + your Constitution modifier\n**Hit Points at Higher Levels:** 1d6 (or 4) + your Constitution modifier per sorcerer level after 1st"
    },
    proficiencies: SRDClassProficiencies {
      armor: vec![],
      weapons: vec![
        SRDEquipment::Open5eItem(SRDItem { key: "dagger", source: "weapons", qty: 0 }),
        SRDEquipment::Open5eItem(SRDItem { key: "dart", source: "weapons", qty: 0 }),
        SRDEquipment::Open5eItem(SRDItem { key: "sling", source: "weapons", qty: 0 }),
        SRDEquipment::Open5eItem(SRDItem { key: "quarterstaff", source: "weapons", qty: 0 }),
        SRDEquipment::Open5eItem(SRDItem { key: "crossbow-light", source: "weapons", qty: 0 }),
      ],
      tools: vec![],
      saving_throws: vec![ "constitution", "charisma" ],
      skills: SRDClassProficientSkills {
        choices: 2,
        options: vec![ "arcana", "deception", "insight", "intimidation", "persuasion", "religion" ]
      },
      desc: "**Armor:** None\n**Weapons:** Daggers, darts, slings, quarterstaffs, light crossbows\n**Tools:** None\n**Saving Throws:** Constitution, Charisma\n**Skills:** Choose two from Arcana, Deception, Insight, Intimidation, Persuasion, and Religion"
    },
    starting_equipment: SRDClassStartingEquipment {
      choice_1: vec![
        vec![
          SRDEquipment::Open5eItem(SRDItem { key: "crossbow-light", source: "weapons", qty: 1 }),
          SRDEquipment::CustomItem(SRDCustomItem { name: "Bolts", qty: 20 })
        ],
        vec![
          SRDEquipment::Open5eCategory(SRDItem { key: "Simple", source: "weapons", qty: 1 }),
        ]
      ],
      choice_2: vec![
        vec![
          SRDEquipment::DnDexItem(SRDItem { key: "component_pouch", source: "adventuring_gear", qty: 1 }),
        ],
        vec![
          SRDEquipment::DnDexItem(SRDItem { key: "arcane_focus", source: "adventuring_gear", qty: 1 }),
        ]
      ],
      choice_3: vec![
        vec![
          SRDEquipment::DnDexItem(SRDItem { key: "dungeoneers_pack", source: "equipment_packs", qty: 1 }),
        ],
        vec![
          SRDEquipment::DnDexItem(SRDItem { key: "explorers_pack", source: "equipment_packs", qty: 1 }),
        ]
      ],
      choice_4: vec![],
      defaults: vec![
        SRDEquipment::Open5eItem(SRDItem { key: "dagger", source: "weapons", qty: 2 }),
      ],
      desc: "You start with the following equipment, in addition to the equipment granted by your background:\n- *(a)* a light crossbow and 20 bolts or *(b)* any simple weapon\n- *(a)* a component pouch or *(b)* an arcane focus\n- *(a)* a dungeoneer's pack or *(b)* an explorer's pack\n- Two daggers"
    },
    spellcasting: Some(SRDClassSpellcasting {
      name: "Spellcasting",
      ability: "charisma",
      desc: "An event in your past, or in the life of a parent or ancestor, left an indelible mark on you, infusing you with arcane magic. This font of magic, whatever its origin, fuels your spells.\n## Cantrips\nAt 1st level, you know four cantrips of your choice from the sorcerer spell list. You learn additional sorcerer cantrips of your choice at higher levels, as shown in the Cantrips Known column of the Sorcerer table.\n## Spell Slots\nThe Sorcerer table shows how many spell slots you have to cast your spells of 1st level and higher. To cast one of these sorcerer spells, you must expend a slot of the spell's level or higher. You regain all expended spell slots when you finish a long rest.\nFor example, if you know the 1st-level spell *burning hands* and have a 1st-level and a 2nd-level spell slot available, you can cast *burning hands* using either slot.\n## Spells Known of 1st Level and Higher\nYou know two 1st-level spells of your choice from the sorcerer spell list.\nThe Spells Known column of the Sorcerer table shows when you learn more sorcerer spells of your choice. Each of these spells must be of a level for which you have spell slots. For instance, when you reach 3rd level in this class, you can learn one new spell of 1st or 2nd level.\nAdditionally, when you gain a level in this class, you can choose one of the sorcerer spells you know and replace it with another spell from the sorcerer spell list, which also must be of a level for which you have spell slots.\n## Spellcasting Ability\nCharisma is your spellcasting ability for your sorcerer spells, since the power of your magic relies on your ability to project your will into the world. You use your Charisma whenever a spell refers to your spellcasting ability. In addition, you use your Charisma modifier when setting the saving throw DC for a sorcerer spell you cast and when making an attack roll with one.\n**Spell save DC** = 8 + your proficiency bonus + your Charisma modifier\n**Spell attack modifier** = your proficiency bonus + your Charisma modifier\n## Spellcasting Focus\nYou can use an arcane focus as a spellcasting focus for your sorcerer spells.",
      at_level: 1
    }),
    levels: SRDClassLevels::<SRDSorcererAttributes> {
      level_1: SRDSorcererAttributes {
        level: 1,
        features: vec![
          SRDClassLevelFeature { key: "sorcerous_origin", name: None }
        ],
        sorcery_points: 0,
        cantrips_known: 4,
        spells_known: 2,
        spell_slots: [ 2, 0, 0, 0, 0, 0, 0, 0, 0 ]
      },
      level_2: SRDSorcererAttributes {
        level: 2,
        features: vec![
          SRDClassLevelFeature { key: "font_of_magic", name: None }
        ],
        sorcery_points: 2,
        cantrips_known: 4,
        spells_known: 3,
        spell_slots: [ 3, 0, 0, 0, 0, 0, 0, 0, 0 ]
      },
      level_3: SRDSorcererAttributes {
        level: 3,
        features: vec![
          SRDClassLevelFeature { key: "metamagic", name: None }
        ],
        sorcery_points: 3,
        cantrips_known: 4,
        spells_known: 4,
        spell_slots: [ 4, 2, 0, 0, 0, 0, 0, 0, 0 ]
      },
      level_4: SRDSorcererAttributes {
        level: 4,
        features: vec![
          SRDClassLevelFeature { key: "ability_score", name: None }
        ],
        sorcery_points: 4,
        cantrips_known: 5,
        spells_known: 5,
        spell_slots: [ 4, 3, 0, 0, 0, 0, 0, 0, 0 ]
      },
      level_5: SRDSorcererAttributes {
        level: 5,
        features: vec![],
        sorcery_points: 5,
        cantrips_known: 5,
        spells_known: 6,
        spell_slots: [ 4, 3, 2, 0, 0, 0, 0, 0, 0 ]
      },
      level_6: SRDSorcererAttributes {
        level: 6,
        features: vec![
          SRDClassLevelFeature { key: "sorcerous_origin", name: Some("Sorcerous Origin Feature") }
        ],
        sorcery_points: 6,
        cantrips_known: 5,
        spells_known: 7,
        spell_slots: [ 4, 3, 3, 0, 0, 0, 0, 0, 0 ]
      },
      level_7: SRDSorcererAttributes {
        level: 7,
        features: vec![],
        sorcery_points: 7,
        cantrips_known: 5,
        spells_known: 8,
        spell_slots: [ 4, 3, 3, 1, 0, 0, 0, 0, 0 ]
      },
      level_8: SRDSorcererAttributes {
        level: 8,
        features: vec![
          SRDClassLevelFeature { key: "ability_score", name: None }
        ],
        sorcery_points: 8,
        cantrips_known: 5,
        spells_known: 9,
        spell_slots: [ 4, 3, 3, 2, 0, 0, 0, 0, 0 ]
      },
      level_9: SRDSorcererAttributes {
        level: 9,
        features: vec![],
        sorcery_points: 9,
        cantrips_known: 5,
        spells_known: 10,
        spell_slots: [ 4, 3, 3, 3, 1, 0, 0, 0, 0 ]
      },
      level_10: SRDSorcererAttributes {
        level: 10,
        features: vec![
          SRDClassLevelFeature { key: "metamagic", name: Some("Metamagic Option") }
        ],
        sorcery_points: 10,
        cantrips_known: 6,
        spells_known: 11,
        spell_slots: [ 4, 3, 3, 3, 2, 0, 0, 0, 0 ]
      },
      level_11: SRDSorcererAttributes {
        level: 11,
        features: vec![],
        sorcery_points: 11,
        cantrips_known: 6,
        spells_known: 12,
        spell_slots: [ 4, 3, 3, 3, 2, 1, 0, 0, 0 ]
      },
      level_12: SRDSorcererAttributes {
        level: 12,
        features: vec![
          SRDClassLevelFeature { key: "ability_score", name: None }
        ],
        sorcery_points: 12,
        cantrips_known: 6,
        spells_known: 12,
        spell_slots: [ 4, 3, 3, 3, 2, 1, 0, 0, 0 ]
      },
      level_13: SRDSorcererAttributes {
        level: 13,
        features: vec![],
        sorcery_points: 13,
        cantrips_known: 6,
        spells_known: 13,
        spell_slots: [ 4, 3, 3, 3, 2, 1, 1, 0, 0 ]
      },
      level_14: SRDSorcererAttributes {
        level: 14,
        features: vec![
          SRDClassLevelFeature { key: "sorcerous_origin", name: Some("Sorcerous Origin Feature") }
        ],
        sorcery_points: 14,
        cantrips_known: 6,
        spells_known: 13,
        spell_slots: [ 4, 3, 3, 3, 2, 1, 1, 0, 0 ]
      },
      level_15: SRDSorcererAttributes {
        level: 15,
        features: vec![],
        sorcery_points: 15,
        cantrips_known: 6,
        spells_known: 14,
        spell_slots: [ 4, 3, 3, 3, 2, 1, 1, 1, 0 ]
      },
      level_16: SRDSorcererAttributes {
        level: 16,
        features: vec![
          SRDClassLevelFeature { key: "ability_score", name: None }
        ],
        sorcery_points: 16,
        cantrips_known: 6,
        spells_known: 14,
        spell_slots: [ 4, 3, 3, 3, 2, 1, 1, 1, 0 ]
      },
      level_17: SRDSorcererAttributes {
        level: 17,
        features: vec![
          SRDClassLevelFeature { key: "metamagic", name: Some("Metamagic Option") }
        ],
        sorcery_points: 17,
        cantrips_known: 6,
        spells_known: 15,
        spell_slots: [ 4, 3, 3, 3, 2, 1, 1, 1, 1 ]
      },
      level_18: SRDSorcererAttributes {
        level: 18,
        features: vec![
          SRDClassLevelFeature { key: "sorcerous_origin", name: Some("Sorcerous Origin Feature") }
        ],
        sorcery_points: 18,
        cantrips_known: 6,
        spells_known: 15,
        spell_slots: [ 4, 3, 3, 3, 3, 1, 1, 1, 1 ]
      },
      level_19: SRDSorcererAttributes {
        level: 19,
        features: vec![
          SRDClassLevelFeature { key: "ability_score", name: None }
        ],
        sorcery_points: 19,
        cantrips_known: 6,
        spells_known: 15,
        spell_slots: [ 4, 3, 3, 3, 3, 2, 1, 1, 1 ]
      },
      level_20: SRDSorcererAttributes {
        level: 20,
        features: vec![
          SRDClassLevelFeature { key: "sorcerous_restoration", name: None }
        ],
        sorcery_points: 20,
        cantrips_known: 6,
        spells_known: 15,
        spell_slots: [ 4, 3, 3, 3, 3, 2, 2, 1, 1 ]
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
        "aura_of_protection",
        SRDClassFeature {
          name: "Aura of Protection",
          desc: "Starting at 6th level, whenever you or a friendly creature within 10 feet of you must make a saving throw, the creature gains a bonus to the saving throw equal to your Charisma modifier (with a minimum bonus of +1). You must be conscious to grant this bonus.\nAt 18th level, the range of this aura increases to 30 feet."
        },
      ),
      (
        "aura_of_courage",
        SRDClassFeature {
          name: "Aura of Courage",
          desc: "Starting at 10th level, you and friendly creatures within 10 feet of you can't be frightened while you are conscious.\nAt 18th level, the range of this aura increases to 30 feet."
        },
      ),
      (
        "cleansing_touch",
        SRDClassFeature {
          name: "Cleansing Touch",
          desc: "Beginning at 14th level, you can use your action to end one spell on yourself or on one willing creature that you touch.\nYou can use this feature a number of times equal to your Charisma modifier (a minimum of once). You regain expended uses when you finish a long rest."
        },
      ),
      (
        "sorcerous_origin",
        SRDClassFeature {
          name: "Sorcerous Origin",
          desc: "Choose a sorcerous origin, which describes the location of your innate magical power: Draconic Bloodline or Wild Magic, both detailed at the end of the class description.\nYour choice grants you features when you choose it at 1st level and again at 6th, 14th, and 18th level.\nDifferent sorcerers claim different origins for their innate magic. Although many variations exist, most of these origins fall into two categories: a draconic bloodline and wild magic. "
        },
      ),
      (
        "font_of_magic",
        SRDClassFeature {
          name: "Font of Magic",
          desc: "At 2nd level, you tap into a deep wellspring of magic within yourself. This wellspring is represented by sorcery points, which allow you to create a variety of magical effects.\n## Sorcery Points\nYou have 2 sorcery points, and you gain more as you reach higher levels, as shown in the Sorcery Points column of the Sorcerer table. You can never have more sorcery points than shown on the table for your level. You regain all spent sorcery points when you finish a long rest.\n## Flexible Casting\nYou can use your sorcery points to gain additional spell slots, or sacrifice spell slots to gain additional sorcery points. You learn other ways to use your sorcery points as you reach higher levels.\n**Creating Spell Slots.** You can transform unexpended sorcery points into one spell slot as a bonus action on your turn. The Creating Spell Slots table shows the cost of creating a spell slot of a given level. You can create spell slots no higher in level than 5th.\nAny spell slot you create with this feature vanishes when you finish a long rest.\n**Creating Spell Slots (table)**\n\n|Spell Slot Level | Sorcery Point Cost |\n| _ | _ |\n| 1st | 2 |\n| 2nd | 3 |\n| 3rd | 5 |\n| 4th | 6 |\n| 5th | 7 |\n\n**Converting a Spell Slot to Sorcery Points.** As a bonus action on your turn, you can expend one spell slot and gain a number of sorcery points equal to the slot's level."
        },
      ),
      (
        "metamagic",
        SRDClassFeature {
          name: "Metamagic",
          desc: "At 3rd level, you gain the ability to twist your spells to suit your needs. You gain two of the following Metamagic options of your choice. You gain another one at 10th and 17th level.\nYou can use only one Metamagic option on a spell when you cast it, unless otherwise noted.\n## Careful Spell\nWhen you cast a spell that forces other creatures to make a saving throw, you can protect some of those creatures from the spell's full force. To do so, you spend 1 sorcery point and choose a number of those creatures up to your Charisma modifier (minimum of one creature). A chosen creature automatically succeeds on its saving throw against the spell.\n## Distant Spell\nWhen you cast a spell that has a range of 5 feet or greater, you can spend 1 sorcery point to double the range of the spell.\nWhen you cast a spell that has a range of touch, you can spend 1 sorcery point to make the range of the spell 30 feet.\n## Empowered Spell\nWhen you roll damage for a spell, you can spend 1 sorcery point to reroll a number of the damage dice up to your Charisma modifier (minimum of one). You must use the new rolls.\nYou can use Empowered Spell even if you have already used a different Metamagic option during the casting of the spell.\n## Extended Spell\nWhen you cast a spell that has a duration of 1 minute or longer, you can spend 1 sorcery point to double its duration, to a maximum duration of 24 hours.\n## Heightened Spell\nWhen you cast a spell that forces a creature to make a saving throw to resist its effects, you can spend 3 sorcery points to give one target of the spell disadvantage on its first saving throw made against the spell.\n## Quickened Spell\nWhen you cast a spell that has a casting time of 1 action, you can spend 2 sorcery points to change the casting time to 1 bonus action for this casting.\n## Subtle Spell\nWhen you cast a spell, you can spend 1 sorcery point to cast it without any somatic or verbal components.\n## Twinned Spell\nWhen you cast a spell that targets only one creature and doesn't have a range of self, you can spend a number of sorcery points equal to the spell's level to target a second creature in range with the same spell (1 sorcery point if the spell is a cantrip).\nTo be eligible, a spell must be incapable of targeting more than one creature at the spell's current level. For example, *magic missile* and *scorching ray* aren't eligible, but *ray of frost* and *chromatic orb* are."
        },
      ),
      (
        "sorceous_restoration",
        SRDClassFeature {
          name: "Sorceous Restoration",
          desc: "At 20th level, you regain 4 expended sorcery points whenever you finish a short rest."
        }
      )
    ])
  };
}
