#![allow(unused, non_upper_case_globals)]
use crate::models::classes::*;
use crate::models::equipment::*;
use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
  pub static ref warlock: SRDClass<SRDWarlockAttributes> = SRDClass::<SRDWarlockAttributes> {
    hit_points: SRDClassHitPoints {
      hit_dice: 8,
      static_option: 5,
      desc: "**Hit Dice:** 1d8 per Warlock level\n**Hit Points at 1st Level:*8 8 + your Constitution modifier\n**Hit Points at Higher Levels:** 1d8 (or 5) + your Constitution modifier per warlock level after 1st"
    },
    proficiencies: SRDClassProficiencies {
      armor: vec![
        SRDEquipment::Open5eCategory(SRDItem { key: "Light", source: "armor", qty: 0 }),
      ],
      weapons: vec![
        SRDEquipment::Open5eCategory(SRDItem { key: "Simple", source: "weapons", qty: 0 }),
      ],
      tools: vec![],
      saving_throws: vec![ "wisdom", "charisma" ],
      skills: SRDClassProficientSkills {
        choices: 2,
        options: vec![ "arcana", "deception", "history", "intimidation", "investigation", "nature", "religion" ]
      },
      desc: "**Armor:** Light armor\n**Weapons:** Simple weapons\n**Tools:** None\n**Saving Throws:** Wisdom, Charisma\n**Skills:** Choose two skills from Arcana, Deception, History, Intimidation, Investigation, Nature, and Religion"
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
          SRDEquipment::DnDexItem(SRDItem { key: "scholars_pack", source: "equipment_packs", qty: 1 }),
        ],
        vec![
          SRDEquipment::DnDexItem(SRDItem { key: "dungeoneers_pack", source: "equipment_packs", qty: 1 }),
        ]
      ],
      choice_4: vec![],
      defaults: vec![
        SRDEquipment::Open5eItem(SRDItem { key: "leather", source: "armor", qty: 1 }),
        SRDEquipment::Open5eCategory(SRDItem { key: "Simple", source: "weapons", qty: 1 }),
        SRDEquipment::Open5eItem(SRDItem { key: "dagger", source: "weapons", qty: 2 }),
      ],
      desc: "You start with the following equipment, in addition to the equipment granted by your background:\n- *(a)* a light crossbow and 20 bolts or *(b)* any simple weapon\n- *(a)* a component pouch or *(b)* an arcane focus\n- *(a)* a scholar's pack or *(b)* a dungeoneer's pack\nLeather armor, any simple weapon, and two daggers"
    },
    spellcasting: Some(SRDClassSpellcasting {
      name: "Pact Magic",
      ability: "charisma",
      desc: "Your arcane research and the magic bestowed on you by your patron have given you facility with spells.\n## Cantrips\nYou know two cantrips of your choice from the warlock spell list. You learn additional warlock cantrips of your choice at higher levels, as shown in the Cantrips Known column of the Warlock table.\n## Spell Slots\nThe Warlock table shows how many spell slots you have. The table also shows what the level of those slots is; all of your spell slots are the same level. To cast one of your warlock spells of 1st level or higher, you must expend a spell slot. You regain all expended spell slots when you finish a short or long rest.\nFor example, when you are 5th level, you have two 3rd-level spell slots. To cast the 1st-level spell thunderwave, you must spend one of those slots, and you cast it as a 3rd-level spell.\n## Spells Known of 1st Level and Higher\nAt 1st level, you know two 1st-level spells of your choice from the warlock spell list.\nThe Spells Known column of the Warlock table shows when you learn more warlock spells of your choice of 1st level and higher. A spell you choose must be of a level no higher than what's shown in the table's Slot Level column for your level. When you reach 6th level, for example, you learn a new warlock spell, which can be 1st, 2nd, or 3rd level.\nAdditionally, when you gain a level in this class, you can choose one of the warlock spells you know and replace it with another spell from the warlock spell list, which also must be of a level for which you have spell slots.\n## Spellcasting Ability\nCharisma is your spellcasting ability for your warlock spells, so you use your Charisma whenever a spell refers to your spellcasting ability. In addition, you use your Charisma modifier when setting the saving throw DC for a warlock spell you cast and when making an attack roll with one.\n**Spell save DC** = 8 + your proficiency bonus + your Charisma modifier\n**Spell attack modifier** = your proficiency bonus + your Charisma modifier\n## Spellcasting Focus\nYou can use an arcane focus as a spellcasting focus for your warlock spells.",
      at_level: 1
    }),
    levels: SRDClassLevels::<SRDWarlockAttributes> {
      level_1: SRDWarlockAttributes {
        level: 1,
        features: vec![
          SRDClassLevelFeature { key: "otherworldly_patron", name: None }
        ],
        cantrips_known: 2,
        spells_known: 2,
        available_spell_slots: 1,
        slot_level: 1,
        invocations_known: 0
      },
      level_2: SRDWarlockAttributes {
        level: 2,
        features: vec![
          SRDClassLevelFeature { key: "eldritch_invocations", name: None }
        ],
        cantrips_known: 2,
        spells_known: 3,
        available_spell_slots: 2,
        slot_level: 1,
        invocations_known: 2
      },
      level_3: SRDWarlockAttributes {
        level: 3,
        features: vec![
          SRDClassLevelFeature { key: "pact_boon", name: None }
        ],
        cantrips_known: 2,
        spells_known: 4,
        available_spell_slots: 2,
        slot_level: 2,
        invocations_known: 2
      },
      level_4: SRDWarlockAttributes {
        level: 4,
        features: vec![
          SRDClassLevelFeature { key: "ability_score", name: None }
        ],
        cantrips_known: 3,
        spells_known: 5,
        available_spell_slots: 2,
        slot_level: 2,
        invocations_known: 2
      },
      level_5: SRDWarlockAttributes {
        level: 5,
        features: vec![],
        cantrips_known: 3,
        spells_known: 6,
        available_spell_slots: 2,
        slot_level: 3,
        invocations_known: 3
      },
      level_6: SRDWarlockAttributes {
        level: 6,
        features: vec![
          SRDClassLevelFeature { key: "otherworldly_patron", name: Some("Otherworldly Patron Feature") }
        ],
        cantrips_known: 3,
        spells_known: 7,
        available_spell_slots: 2,
        slot_level: 3,
        invocations_known: 3
      },
      level_7: SRDWarlockAttributes {
        level: 7,
        features: vec![],
        cantrips_known: 3,
        spells_known: 8,
        available_spell_slots: 2,
        slot_level: 4,
        invocations_known: 4
      },
      level_8: SRDWarlockAttributes {
        level: 8,
        features: vec![
          SRDClassLevelFeature { key: "ability_score", name: None }
        ],
        cantrips_known: 3,
        spells_known: 9,
        available_spell_slots: 2,
        slot_level: 4,
        invocations_known: 4
      },
      level_9: SRDWarlockAttributes {
        level: 9,
        features: vec![],
        cantrips_known: 3,
        spells_known: 10,
        available_spell_slots: 2,
        slot_level: 5,
        invocations_known: 5
      },
      level_10: SRDWarlockAttributes {
        level: 10,
        features: vec![
          SRDClassLevelFeature { key: "otherworldly_patron", name: Some("Otherworldly Patron Feature") }
        ],
        cantrips_known: 4,
        spells_known: 10,
        available_spell_slots: 2,
        slot_level: 5,
        invocations_known: 5
      },
      level_11: SRDWarlockAttributes {
        level: 11,
        features: vec![
          SRDClassLevelFeature { key: "mystic_arcanum", name: Some("Mystic Arcanum (6th Level)") }
        ],
        cantrips_known: 4,
        spells_known: 11,
        available_spell_slots: 3,
        slot_level: 5,
        invocations_known: 5
      },
      level_12: SRDWarlockAttributes {
        level: 12,
        features: vec![
          SRDClassLevelFeature { key: "ability_score", name: None }
        ],
        cantrips_known: 4,
        spells_known: 11,
        available_spell_slots: 3,
        slot_level: 5,
        invocations_known: 6
      },
      level_13: SRDWarlockAttributes {
        level: 13,
        features: vec![
          SRDClassLevelFeature { key: "mystic_arcanum", name: Some("Mystic Arcanum (7th Level)") }
        ],
        cantrips_known: 4,
        spells_known: 12,
        available_spell_slots: 3,
        slot_level: 5,
        invocations_known: 6
      },
      level_14: SRDWarlockAttributes {
        level: 14,
        features: vec![
          SRDClassLevelFeature { key: "otherworldly_patron", name: Some("Otherworldly Patron Feature") }
        ],
        cantrips_known: 4,
        spells_known: 12,
        available_spell_slots: 3,
        slot_level: 5,
        invocations_known: 6
      },
      level_15: SRDWarlockAttributes {
        level: 15,
        features: vec![
          SRDClassLevelFeature { key: "mystic_arcanum", name: Some("Mystic Arcanum (8th Level)") }
        ],
        cantrips_known: 4,
        spells_known: 13,
        available_spell_slots: 3,
        slot_level: 5,
        invocations_known: 7
      },
      level_16: SRDWarlockAttributes {
        level: 16,
        features: vec![
          SRDClassLevelFeature { key: "ability_score", name: None }
        ],
        cantrips_known: 4,
        spells_known: 13,
        available_spell_slots: 3,
        slot_level: 5,
        invocations_known: 7
      },
      level_17: SRDWarlockAttributes {
        level: 17,
        features: vec![
          SRDClassLevelFeature { key: "mystic_arcanum", name: Some("Mystic Arcanum (9th Level)") }
        ],
        cantrips_known: 4,
        spells_known: 14,
        available_spell_slots: 4,
        slot_level: 5,
        invocations_known: 7
      },
      level_18: SRDWarlockAttributes {
        level: 18,
        features: vec![],
        cantrips_known: 4,
        spells_known: 15,
        available_spell_slots: 4,
        slot_level: 5,
        invocations_known: 8
      },
      level_19: SRDWarlockAttributes {
        level: 19,
        features: vec![
          SRDClassLevelFeature { key: "ability_score", name: None }
        ],
        cantrips_known: 4,
        spells_known: 15,
        available_spell_slots: 4,
        slot_level: 5,
        invocations_known: 8
      },
      level_20: SRDWarlockAttributes {
        level: 20,
        features: vec![
          SRDClassLevelFeature { key: "eldritch_master", name: Some("Eldritch Master") }
        ],
        cantrips_known: 4,
        spells_known: 15,
        available_spell_slots: 4,
        slot_level: 5,
        invocations_known: 8
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
        "otherworldly_patron",
        SRDClassFeature {
          name: "Otherworldly Patron",
          desc: "At 1st level, you have struck a bargain with an otherworldly being of your choice: the Archfey, the Fiend, or the Great Old One, each of which is detailed at the end of the class description. Your choice grants you features at 1st level and again at 6th, 10th, and 14th level.\nThe beings that serve as patrons for warlocks are mighty inhabitants of other planes of existence_not gods, but almost godlike in their power. Various patrons give their warlocks access to different powers and invocations, and expect significant favors in return.\nSome patrons collect warlocks, doling out mystic knowledge relatively freely or boasting of their ability to bind mortals to their will. Other patrons bestow their power only grudgingly, and might make a pact with only one warlock. Warlocks who serve the same patron might view each other as allies, siblings, or rivals. "
        },
      ),
      (
        "pact_boon",
        SRDClassFeature {
          name: "Pact Boon",
          desc: "At 3rd level, your otherworldly patron bestows a gift upon you for your loyal service. You gain one of the following features of your choice.\n## Pact of the Chain\nYou learn the find familiar spell and can cast it as a ritual. The spell doesn't count against your number of spells known.\nWhen you cast the spell, you can choose one of the normal forms for your familiar or one of the following special forms: imp, pseudodragon, quasit, or sprite.\nAdditionally, when you take the Attack action, you can forgo one of your own attacks to allow your familiar to make one attack of its own with its reaction.\n## Pact of the Blade\nYou can use your action to create a pact weapon in your empty hand. You can choose the form that this melee weapon takes each time you create it. You are proficient with it while you wield it. This weapon counts as magical for the purpose of overcoming resistance and immunity to nonmagical attacks and damage.\nYour pact weapon disappears if it is more than 5 feet away from you for 1 minute or more. It also disappears if you use this feature again, if you dismiss the weapon (no action required), or if you die.\nYou can transform one magic weapon into your pact weapon by performing a special ritual while you hold the weapon. You perform the ritual over the course of 1 hour, which can be done during a short rest. You can then dismiss the weapon, shunting it into an extradimensional space, and it appears whenever you create your pact weapon thereafter. You can't affect an artifact or a sentient weapon in this way. The weapon ceases being your pact weapon if you die, if you perform the 1_hour ritual on a different weapon, or if you use a 1_hour ritual to break your bond to it. The weapon appears at your feet if it is in the extradimensional space when the bond breaks.\n## Pact of the Tome\nYour patron gives you a grimoire called a Book of Shadows. When you gain this feature, choose three cantrips from any class's spell list (the three needn't be from the same list). While the book is on your person, you can cast those cantrips at will. They don't count against your number of cantrips known. If they don't appear on the warlock spell list, they are nonetheless warlock spells for you.\nIf you lose your Book of Shadows, you can perform a 1_hour ceremony to receive a replacement from your patron. This ceremony can be performed during a short or long rest, and it destroys the previous book. The book turns to ash when you die.\n\n>## Your Pact Boon\n>Each Pact Boon option produces a special creature or an object that reflects your patron's nature.\n>**Pact of the Chain.** Your familiar is more cunning than a typical familiar. Its default form can be a reflection of your patron, with sprites and pseudodragons tied to the Archfey and imps and quasits tied to the Fiend. Because the Great Old One's nature is inscrutable, any familiar form is suitable for it.\n>**Pact of the Blade.** If your patron is the Archfey, your weapon might be a slender blade wrapped in leafy vines. If you serve the Fiend, your weapon could be an axe made of black metal and adorned with decorative flames. If your patron is the Great Old One, your weapon might be an ancient_looking spear, with a gemstone embedded in its head, carved to look like a terrible unblinking eye.\n>**Pact of the Tome.** Your Book of Shadows might be a fine, gilt_edged tome with spells of enchantment and illusion, gifted to you by the lordly Archfey. It could be a weighty tome bound in demon hide studded with iron, holding spells of conjuration and a wealth of forbidden lore about the sinister regions of the cosmos, a gift of the Fiend. Or it could be the tattered diary of a lunatic driven mad by contact with the Great Old One, holding scraps of spells that only your own burgeoning insanity allows you to understand and cast."
        },
      ),
      (
        "mystic_arcanum",
        SRDClassFeature {
          name: "Mystic Arcanum",
          desc: "At 11th level, your patron bestows upon you a magical secret called an arcanum. Choose one 6th_ level spell from the warlock spell list as this arcanum.\nYou can cast your arcanum spell once without expending a spell slot. You must finish a long rest before you can do so again.\nAt higher levels, you gain more warlock spells of your choice that can be cast in this way: one 7th_ level spell at 13th level, one 8th_level spell at 15th level, and one 9th_level spell at 17th level. You regain all uses of your Mystic Arcanum when you finish a long rest."
        },
      ),
      (
        "eldritch_master",
        SRDClassFeature {
          name: "Eldritch Master",
          desc: "At 20th level, you can draw on your inner reserve of mystical power while entreating your patron to regain expended spell slots. You can spend 1 minute entreating your patron for aid to regain all your expended spell slots from your Pact Magic feature. Once you regain spell slots with this feature, you must finish a long rest before you can do so again."
        },
      ),
      (
        "eldritch_invocations",
        SRDClassFeature {
          name: "Eldritch Invocations",
          desc: "In your study of occult lore, you have unearthed eldritch invocations, fragments of forbidden knowledge that imbue you with an abiding magical ability.\nAt 2nd level, you gain two eldritch invocations of your choice. Your invocation options are detailed at the end of the class description. When you gain certain warlock levels, you gain additional invocations of your choice, as shown in the Invocations Known column of the Warlock table.\nAdditionally, when you gain a level in this class, you can choose one of the invocations you know and replace it with another invocation that you could learn at that level.\nIf an eldritch invocation has prerequisites, you must meet them to learn it. You can learn the invocation at the same time that you meet its prerequisites. A level prerequisite refers to your level in this class.\n## Agonizing Blast\n*Prerequisite: eldritch blast cantrip*\nWhen you cast eldritch blast, add your Charisma modifier to the damage it deals on a hit.\n## Armor of Shadows\nYou can cast mage armor on yourself at will, without expending a spell slot or material components.\n## Ascendant Step\n*Prerequisite: 9th level*\nYou can cast levitate on yourself at will, without expending a spell slot or material components.\n## Beast Speech\nYou can cast speak with animals at will, without expending a spell slot.\n## Beguiling Influence\nYou gain proficiency in the Deception and Persuasion skills.\n## Bewitching Whispers\n*Prerequisite: 7th level*\nYou can cast compulsion once using a warlock spell slot. You can't do so again until you finish a long rest.\n## Book of Ancient Secrets\n*Prerequisite: Pact of the Tome feature*\nYou can now inscribe magical rituals in your Book of Shadows. Choose two 1st_level spells that have the ritual tag from any class's spell list (the two needn't be from the same list). The spells appear in the book and don't count against the number of spells you know. With your Book of Shadows in hand, you can cast the chosen spells as rituals. You can't cast the spells except as rituals, unless you've learned them by some other means. You can also cast a warlock spell you know as a ritual if it has the ritual tag.\nOn your adventures, you can add other ritual spells to your Book of Shadows. When you find such a spell, you can add it to the book if the spell's level is equal to or less than half your warlock level (rounded up) and if you can spare the time to transcribe the spell. For each level of the spell, the transcription process takes 2 hours and costs 50 gp for the rare inks needed to inscribe it.\n## Chains of Carceri\n*Prerequisite: 15th level, Pact of the Chain feature*\nYou can cast hold monster at will_targeting a celestial, fiend, or elemental_without expending a spell slot or material components. You must finish a long rest before you can use this invocation on the same creature again.\n## Devil's Sight\nYou can see normally in darkness, both magical and nonmagical, to a distance of 120 feet.\n## Dreadful Word\n*Prerequisite: 7th level*\nYou can cast confusion once using a warlock spell slot. You can't do so again until you finish a long rest.\n## Eldritch Sight\nYou can cast detect magic at will, without expending a spell slot.\n## Eldritch Spear\n*Prerequisite: eldritch blast cantrip*\nWhen you cast eldritch blast, its range is 300 feet.\n## Eyes of the Rune Keeper\nYou can read all writing.\n## Fiendish Vigor\nYou can cast false life on yourself at will as a 1st_level spell, without expending a spell slot or material components.\n## Gaze of Two Minds\nYou can use your action to touch a willing humanoid and perceive through its senses until the end of your next turn. As long as the creature is on the same plane of existence as you, you can use your action on subsequent turns to maintain this connection, extending the duration until the end of your next turn. While perceiving through the other creature's senses, you benefit from any special senses possessed by that creature, and you are blinded and deafened to your own surroundings.\n## Lifedrinker\n*Prerequisite: 12th level, Pact of the Blade feature*\nWhen you hit a creature with your pact weapon, the creature takes extra necrotic damage equal to your Charisma modifier (minimum 1).\n## Mask of Many Faces\nYou can cast disguise self at will, without expending a spell slot.\n## Master of Myriad Forms\n*Prerequisite: 15th level*\nYou can cast alter self at will, without expending a spell slot.\n## Minions of Chaos\n*Prerequisite: 9th level*\nYou can cast conjure elemental once using a warlock spell slot. You can't do so again until you finish a long rest.\n## Mire the Mind\n*Prerequisite: 5th level*\nYou can cast slow once using a warlock spell slot. You can't do so again until you finish a long rest.\n## Misty Visions\nYou can cast silent image at will, without expending a spell slot or material components.\n## One with Shadows\n*Prerequisite: 5th level*\nWhen you are in an area of dim light or darkness, you can use your action to become invisible until you move or take an action or a reaction.\n## Otherworldly Leap\n*Prerequisite: 9th level*\nYou can cast jump on yourself at will, without expending a spell slot or material components.\n## Repelling Blast\n*Prerequisite: eldritch blast cantrip*\nWhen you hit a creature with eldritch blast, you can push the creature up to 10 feet away from you in a straight line.\n## Sculptor of Flesh\n*Prerequisite: 7th level*\nYou can cast polymorph once using a warlock spell slot. You can't do so again until you finish a long rest.\n## Sign of Ill Omen\n*Prerequisite: 5th level*\nYou can cast bestow curse once using a warlock spell slot. You can't do so again until you finish a long rest.\n## Thief of Five Fates\nYou can cast bane once using a warlock spell slot. You can't do so again until you finish a long rest.\n## Thirsting Blade\n*Prerequisite: 5th level, Pact of the Blade feature*\nYou can attack with your pact weapon twice, instead of once, whenever you take the Attack action on your turn.\n## Visions of Distant Realms\n*Prerequisite: 15th level*\nYou can cast arcane eye at will, without expending a spell slot.\n## Voice of the Chain Master\n*Prerequisite: Pact of the Chain feature*\nYou can communicate telepathically with your familiar and perceive through your familiar's senses as long as you are on the same plane of existence. Additionally, while perceiving through your familiar's senses, you can also speak through your familiar in your own voice, even if your familiar is normally incapable of speech.\n## Whispers of the Grave\n*Prerequisite: 9th level*\nYou can cast speak with dead at will, without expending a spell slot.\n## Witch Sight\n*Prerequisite: 15th level*\nYou can see the true form of any shapechanger or creature concealed by illusion or transmutation magic while the creature is within 30 feet of you and within line of sight."
        }
      ),
    ])
  };
}
