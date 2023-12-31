#![allow(unused, non_upper_case_globals)]
use crate::models::classes::*;
use crate::models::equipment::*;
use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
  pub static ref wizard: SRDClass<SRDWizardAttributes> = SRDClass::<SRDWizardAttributes> {
    hit_points: SRDClassHitPoints {
      hit_dice: 6,
      static_option: 4,
      desc: "**Hit Dice:** 1d6 per Wizard level\n**Hit Points at 1st Level:** 6 + your Constitution modifier\n**Hit Points at Higher Levels:** 1d6 (or 4) + your Constitution modifier per wizard level after 1st"
    },
    proficiencies: SRDClassProficiencies {
      armor: vec![],
      weapons: vec![
        Equipment::Open5e(Open5eEquipment::Open5eItem(SRDItem { key: "dagger", source: "weapons", qty: 0 })),
        Equipment::Open5e(Open5eEquipment::Open5eItem(SRDItem { key: "dart", source: "weapons", qty: 0 })),
        Equipment::Open5e(Open5eEquipment::Open5eItem(SRDItem { key: "sling", source: "weapons", qty: 0 })),
        Equipment::Open5e(Open5eEquipment::Open5eItem(SRDItem { key: "quaterstaff", source: "weapons", qty: 0 })),
        Equipment::Open5e(Open5eEquipment::Open5eItem(SRDItem { key: "crossbow-light", source: "weapons", qty: 0 })),
      ],
      tools: vec![],
      saving_throws: vec![ "intelligence", "wisdom" ],
      skills: SRDClassProficientSkills {
        choices: 3,
        options: vec![ "arcana", "history", "insight", "investigation", "medicine", "religion" ]
      },
      desc: "**Armor:** None\n**Weapons:** Daggers, darts, slings, quarterstaffs, light crossbows\n**Tools:** None\n**Saving Throws:** Intelligence, Wisdom\n**Skills:** Choose two from Arcana, History, Insight, Investigation, Medicine, and Religion"
    },
    starting_equipment: SRDClassStartingEquipment {
      choice_1: vec![
        vec![
          Equipment::Open5e(Open5eEquipment::Open5eItem(SRDItem { key: "quaterstaff", source: "weapons", qty: 1 })),
        ],
        vec![
          Equipment::Open5e(Open5eEquipment::Open5eItem(SRDItem { key: "dagger", source: "weapons", qty: 1 })),
        ]
      ],
      choice_2: vec![
        vec![
          Equipment::DnDex(DnDexEquipment::DnDexItem(SRDItem { key: "component_pouch", source: "adventuring_gear", qty: 1 })),
        ],
        vec![
          Equipment::DnDex(DnDexEquipment::DnDexItem(SRDItem { key: "arcane_focus", source: "adventuring_gear", qty: 1 })),
        ]
      ],
      choice_3: vec![
        vec![
          Equipment::DnDex(DnDexEquipment::DnDexItem(SRDItem { key: "scholars_pack", source: "equipment_packs", qty: 1 })),
        ],
        vec![
          Equipment::DnDex(DnDexEquipment::DnDexItem(SRDItem { key: "dungeoneers_pack", source: "equipment_packs", qty: 1 })),
        ]
      ],
      choice_4: vec![],
      defaults: vec![
        Equipment::DnDex(DnDexEquipment::CustomItem(SRDCustomItem { name: "Spellbook", qty: 1 }))
      ],
      desc: "You start with the following equipment, in addition to the equipment granted by your background:\n- *(a)* a quarterstaff or *(b)* a dagger\n- *(a)* a component pouch or *(b)* an arcane focus\n- *(a)* a scholar's pack or *(b)* an explorer's pack\n- A spellbook"
    },
    spellcasting: Some(SRDClassSpellcasting {
      name: "Spellcasting",
      ability: "intelligence",
      desc: "As a student of arcane magic, you have a spellbook containing spells that show the first glimmerings of your true power.\n## Cantrips\nAt 1st level, you know three cantrips of your choice from the wizard spell list. You learn additional wizard cantrips of your choice at higher levels, as shown in the Cantrips Known column of the Wizard table.\n## Spellbook\nAt 1st level, you have a spellbook containing six 1st- level wizard spells of your choice. Your spellbook is the repository of the wizard spells you know, except your cantrips, which are fixed in your mind.\n\n>## Your Spellbook\n>The spells that you add to your spellbook as you gain levels reflect the arcane research you conduct on your own, as well as intellectual breakthroughs you have had about the nature of the multiverse. You might find other spells during your adventures. You could discover a spell recorded on a scroll in an evil wizard's chest, for example, or in a dusty tome in an ancient library.\n>**Copying a Spell into the Book.** When you find a wizard spell of 1st level or higher, you can add it to your spellbook if it is of a spell level you can prepare and if you can spare the time to decipher and copy it.\n>Copying that spell into your spellbook involves reproducing the basic form of the spell, then deciphering the unique system of notation used by the wizard who wrote it. You must practice the spell until you understand the sounds or gestures required, then transcribe it into your spellbook using your own notation.\n>For each level of the spell, the process takes 2 hours and costs 50 gp. The cost represents material components you expend as you experiment with the spell to master it, as well as the fine inks you need to record it. Once you have spent this time and money, you can prepare the spell just like your other spells.\n>**Replacing the Book.** You can copy a spell from your own spellbook into another book-for example, if you want to make a backup copy of your spellbook. This is just like copying a new spell into your spellbook, but faster and easier, since you understand your own notation and already know how to cast the spell. You need spend only 1 hour and 10 gp for each level of the copied spell.\n>If you lose your spellbook, you can use the same procedure to transcribe the spells that you have prepared into a new spellbook. Filling out the remainder of your spellbook requires you to find new spells to do so, as normal. For this reason, many wizards keep backup spellbooks in a safe place.\n>**The Book's Appearance.** Your spellbook is a unique compilation of spells, with its own decorative flourishes and margin notes. It might be a plain, functional leather volume that you received as a gift from your master, a finely bound gilt-edged tome you found in an ancient library, or even a loose collection of notes scrounged together after you lost your previous spellbook in a mishap.\n\n## Preparing and Casting Spells\nThe Wizard table shows how many spell slots you have to cast your spells of 1st level and higher. To cast one of these spells, you must expend a slot of the spell's level or higher. You regain all expended spell slots when you finish a long rest.\nYou prepare the list of wizard spells that are available for you to cast. To do so, choose a number of wizard spells from your spellbook equal to your Intelligence modifier + your wizard level (minimum of one spell). The spells must be of a level for which you have spell slots.\nFor example, if you're a 3rd-level wizard, you have four 1st-level and two 2nd-level spell slots. With an Intelligence of 16, your list of prepared spells can include six spells of 1st or 2nd level, in any combination, chosen from your spellbook. If you prepare the 1st-level spell magic missile, you can cast it using a 1st-level or a 2nd-level slot. Casting the spell doesn't remove it from your list of prepared spells.\nYou can change your list of prepared spells when you finish a long rest. Preparing a new list of wizard spells requires time spent studying your spellbook and memorizing the incantations and gestures you must make to cast the spell: at least 1 minute per spell level for each spell on your list.\n## Spellcasting Ability\nIntelligence is your spellcasting ability for your wizard spells, since you learn your spells through dedicated study and memorization. You use your Intelligence whenever a spell refers to your spellcasting ability. In addition, you use your Intelligence modifier when setting the saving throw DC for a wizard spell you cast and when making an attack roll with one.\n**Spell save DC** = 8 + your proficiency bonus + your Intelligence modifier\n**Spell attack modifier** = your proficiency bonus + your Intelligence modifier\n## Ritual Casting\nYou can cast a wizard spell as a ritual if that spell has the ritual tag and you have the spell in your spellbook. You don't need to have the spell prepared.\n## Spellcasting Focus\nYou can use an arcane focus as a spellcasting focus for your wizard spells.\n## Learning Spells of 1st Level and Higher\nEach time you gain a wizard level, you can add two wizard spells of your choice to your spellbook for free. Each of these spells must be of a level for which you have spell slots, as shown on the Wizard table. On your adventures, you might find other spells that you can add to your spellbook (see the “Your Spellbook” sidebar).",
      at_level: 1
    }),
    levels: SRDClassLevels::<SRDWizardAttributes> {
      level_1: SRDWizardAttributes {
        level: 1,
        features: vec![
          SRDClassLevelFeature { key: "arcane_recovery", name: None }
        ],
        cantrips_known: 3,
        spell_slots: [ 2, 0, 0, 0, 0, 0, 0, 0, 0 ]
      },
      level_2: SRDWizardAttributes {
        level: 2,
        features: vec![
          SRDClassLevelFeature { key: "arcane_tradition", name: None }
        ],
        cantrips_known: 3,
        spell_slots: [ 3, 0, 0, 0, 0, 0, 0, 0, 0 ]
      },
      level_3: SRDWizardAttributes {
        level: 3,
        features: vec![],
        cantrips_known: 3,
        spell_slots: [ 4, 2, 0, 0, 0, 0, 0, 0, 0 ]
      },
      level_4: SRDWizardAttributes {
        level: 4,
        features: vec![
          SRDClassLevelFeature { key: "ability_score", name: None }
        ],
        cantrips_known: 4,
        spell_slots: [ 4, 3, 0, 0, 0, 0, 0, 0, 0 ]
      },
      level_5: SRDWizardAttributes {
        level: 5,
        features: vec![],
        cantrips_known: 4,
        spell_slots: [ 4, 3, 2, 0, 0, 0, 0, 0, 0 ]
      },
      level_6: SRDWizardAttributes {
        level: 6,
        features: vec![
          SRDClassLevelFeature { key: "arcane_tradition", name: Some("Arcane Tradition Feature") }
        ],
        cantrips_known: 4,
        spell_slots: [ 4, 3, 3, 0, 0, 0, 0, 0, 0 ]
      },
      level_7: SRDWizardAttributes {
        level: 7,
        features: vec![],
        cantrips_known: 4,
        spell_slots: [ 4, 3, 3, 1, 0, 0, 0, 0, 0 ]
      },
      level_8: SRDWizardAttributes {
        level: 8,
        features: vec![
          SRDClassLevelFeature { key: "ability_score", name: None }
        ],
        cantrips_known: 4,
        spell_slots: [ 4, 3, 3, 2, 0, 0, 0, 0, 0 ]
      },
      level_9: SRDWizardAttributes {
        level: 9,
        features: vec![],
        cantrips_known: 4,
        spell_slots: [ 4, 3, 3, 3, 1, 0, 0, 0, 0 ]
      },
      level_10: SRDWizardAttributes {
        level: 10,
        features: vec![
          SRDClassLevelFeature { key: "arcane_tradition", name: Some("Arcane Tradition Feature") }
        ],
        cantrips_known: 5,
        spell_slots: [ 4, 3, 3, 3, 2, 0, 0, 0, 0 ]
      },
      level_11: SRDWizardAttributes {
        level: 11,
        features: vec![],
        cantrips_known: 5,
        spell_slots: [ 4, 3, 3, 3, 2, 1, 0, 0, 0 ]
      },
      level_12: SRDWizardAttributes {
        level: 12,
        features: vec![
          SRDClassLevelFeature { key: "ability_score", name: None }
        ],
        cantrips_known: 5,
        spell_slots: [ 4, 3, 3, 3, 2, 1, 0, 0, 0 ]
      },
      level_13: SRDWizardAttributes {
        level: 13,
        features: vec![],
        cantrips_known: 5,
        spell_slots: [ 4, 3, 3, 3, 2, 1, 1, 0, 0 ]
      },
      level_14: SRDWizardAttributes {
        level: 14,
        features: vec![
          SRDClassLevelFeature { key: "arcane_tradition", name: Some("Arcane Tradition Feature") }
        ],
        cantrips_known: 5,
        spell_slots: [ 4, 3, 3, 3, 2, 1, 1, 0, 0 ]
      },
      level_15: SRDWizardAttributes {
        level: 15,
        features: vec![],
        cantrips_known: 5,
        spell_slots: [ 4, 3, 3, 3, 2, 1, 1, 1, 0 ]
      },
      level_16: SRDWizardAttributes {
        level: 16,
        features: vec![
          SRDClassLevelFeature { key: "ability_score", name: None }
        ],
        cantrips_known: 5,
        spell_slots: [ 4, 3, 3, 3, 2, 1, 1, 1, 0 ]
      },
      level_17: SRDWizardAttributes {
        level: 17,
        features: vec![],
        cantrips_known: 5,
        spell_slots: [ 4, 3, 3, 3, 2, 1, 1, 1, 1 ]
      },
      level_18: SRDWizardAttributes {
        level: 18,
        features: vec![
          SRDClassLevelFeature { key: "spell_mastery", name: None }
        ],
        cantrips_known: 5,
        spell_slots: [ 4, 3, 3, 3, 3, 1, 1, 1, 1 ]
      },
      level_19: SRDWizardAttributes {
        level: 19,
        features: vec![
          SRDClassLevelFeature { key: "ability_score", name: None }
        ],
        cantrips_known: 5,
        spell_slots: [ 4, 3, 3, 3, 3, 2, 1, 1, 1 ]
      },
      level_20: SRDWizardAttributes {
        level: 20,
        features: vec![
          SRDClassLevelFeature { key: "signature_spells", name: None }
        ],
        cantrips_known: 5,
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
        "arcane_recovery",
        SRDClassFeature {
          name: "Arcane Recovery",
          desc: "You have learned to regain some of your magical energy by studying your spellbook. Once per day when you finish a short rest, you can choose expended spell slots to recover. The spell slots can have a combined level that is equal to or less than half your wizard level (rounded up), and none of the slots can be 6th level or higher.\nFor example, if you're a 4th_level wizard, you can recover up to two levels worth of spell slots. You can recover either a 2nd_level spell slot or two 1st_level spell slots."
        },
      ),
      (
        "spell_mastery",
        SRDClassFeature {
          name: "Spell Mastery",
          desc: "At 18th level, you have achieved such mastery over certain spells that you can cast them at will. Choose a 1st_level wizard spell and a 2nd_level wizard spell that are in your spellbook. You can cast those spells at their lowest level without expending a spell slot when you have them prepared. If you want to cast either spell at a higher level, you must expend a spell slot as normal.\nBy spending 8 hours in study, you can exchange one or both of the spells you chose for different spells of the same levels."
        },
      ),
      (
        "signature_spells",
        SRDClassFeature {
          name: "Signature Spells",
          desc: "When you reach 20th level, you gain mastery over two powerful spells and can cast them with little effort. Choose two 3rd_level wizard spells in your spellbook as your signature spells. You always have these spells prepared, they don't count against the number of spells you have prepared, and you can cast each of them once at 3rd level without expending a spell slot. When you do so, you can't do so again until you finish a short or long rest.\nIf you want to cast either spell at a higher level, you must expend a spell slot as normal."
        },
      ),
      (
        "arcane_tradition",
        SRDClassFeature {
          name: "Arcane Tradition",
          desc: "When you reach 2nd level, you choose an arcane tradition, shaping your practice of magic through one of eight schools: Abjuration, Conjuration, Divination, Enchantment, Evocation, Illusion, Necromancy, or Transmutation, all detailed at the end of the class description.\nYour choice grants you features at 2nd level and again at 6th, 10th, and 14th level.\n The study of wizardry is ancient, stretching back to the earliest mortal discoveries of magic. It is firmly established in fantasy gaming worlds, with various traditions dedicated to its complex study.\nThe most common arcane traditions in the multiverse revolve around the schools of magic. Wizards through the ages have cataloged thousands of spells, grouping them into eight categories called schools. In some places, these traditions are literally schools; a wizard might study at the School of Illusion while another studies across town at the School of Enchantment. In other institutions, the schools are more like academic departments, with rival faculties competing for students and funding. Even wizards who train apprentices in the solitude of their own towers use the division of magic into schools as a learning device, since the spells of each school require mastery of different techniques."
        }
      )
    ])
  };
}
