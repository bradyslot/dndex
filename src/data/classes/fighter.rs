#![allow(unused)]
pub const FIGHTER_DATA: &str = r#"
{
  "hit_points": {
    "hit_dice": 10,
    "static_option": 6,
    "desc": "**Hit Dice:** 1d10 per Fighter level\n**Hit Points at 1st Level:** 10 + your Constitution modifier\n**Hit Points at Higher Levels:** 1d10 (or 6) + your Constitution modifier per fighter level after 1st"
  },
  "proficiencies": {
    "armor": [
      { "category": "Armor" }
    ],
    "weapons": [
      { "category": "Simple" },
      { "category": "Martial" }
    ],
    "tools": [],
    "saving_throws": [ "strength", "constitution" ],
    "skills": { "choices": 2, "options": [ "acrobatics", "animal_handling", "athletics", "history", "insight", "intimidation", "perception", "survival" ] },
    "desc": "**Armor:** All armor, shields\n**Weapons:** Simple weapons, martial weapons\n**Tools:** None\n**Saving Throws:** Strength, Constitution\n**Skills:** Choose two skills from Acrobatics, Animal, Handling, Athletics, History, Insight, Intimidation, Perception, and Survival"
  },
  "equipment": {
    "choice_1": [
      [
        { "source": "open5e", "location": "armor", "key": "chain-mail" }
      ],
      [
        { "source": "open5e", "location": "armor", "key": "leather" },
        { "source": "open5e", "location": "weapons", "key": "longbow" },
        { "name": "Arrows", "qty": 20 }
      ]
    ],
    "choice_2": [
      [
        { "source": "open5e", "location": "weapons", "category": "Martial" },
        { "source": "open5e", "location": "armor", "key": "shield" }
      ],
      [
        { "source": "open5e", "location": "weapons", "category": "Martial", "qty": 2 }
      ]
    ],
    "choice_3": [
      [
        { "source": "open5e", "location": "weapons", "key": "crossbow_light" },
        { "name": "Bolts", "qty": 20 }
      ],
      [
        { "source": "open5e", "location": "weapons", "key": "handaxe", "qty": 2}
      ]
    ],
    "choice_4": [
      [
        { "source": "data", "location": "equipment_packs", "key": "dungeoneers_pack" }
      ],
      [
        { "source": "data", "location": "equipment_packs", "key": "explorers_pack" }
      ]
    ],
    "defaults": [],
    "desc": "You start with the following equipment, in addition to the equipment granted by your background:\n- *(a)* chain mail or *(b)* leather armor, longbow, and 20 arrows\n- *(a)* a martial weapon and a shield or *(b)* two martial weapons\n- *(a)* a light crossbow and 20 bolts or *(b)* two handaxes\n- *(a)* a dungeoneer's pack or *(b)* an explorer's pack"
  },
  "spellcasting": {},
  "levels": [
    {
      "level": 1,
      "features": [
        { "key": "fighter_fighting_style" },
        { "key": "second_wind" }
      ]
    },
    {
      "level": 2,
      "features": [
        { "key": "action_surge", "name": "Action Surge (1 use)" }
      ]
    },
    {
      "level": 3,
      "features": [
        { "key": "martial_archetype" }
      ]
    },
    {
      "level": 4,
      "features": [
        { "key": "fighter_ability_score" }
      ]
    },
    {
      "level": 5,
      "features": [
        { "key": "fighter_extra_attack" }
      ]
    },
    {
      "level": 6,
      "features": [
        { "key": "fighter_ability_score" }
      ]
    },
    {
      "level": 7,
      "features": [
        { "key": "martial_archetype", "name": "Martial Archetype Feature" }
      ]
    },
    {
      "level": 8,
      "features": [
        { "key": "fighter_ability_score" }
      ]
    },
    {
      "level": 9,
      "features": [
        { "key": "indomitable", "name": "Indomitable (1 use)" }
      ]
    },
    {
      "level": 10,
      "features": [
        { "key": "martial_archetype", "name": "Martial Archetype Feature" }
      ]
    },
    {
      "level": 11,
      "features": [
        { "key": "fighter_extra_attack", "name": "Extra Attack (2)" }
      ]
    },
    {
      "level": 12,
      "features": [
        { "key": "fighter_ability_score" }
      ]
    },
    {
      "level": 13,
      "features": [
        { "key": "indomitable", "name": "Indomitable (2 uses)" }
      ]
    },
    {
      "level": 14,
      "features": [
        { "key": "fighter_ability_score" }
      ]
    },
    {
      "level": 15,
      "features": [
        { "key": "martial_archetype", "name": "Martial Archetype Feature" }
      ]
    },
    {
      "level": 16,
      "features": [
        { "key": "fighter_ability_score" }
      ]
    },
    {
      "level": 17,
      "features": [
        { "key": "action_surge", "name": "Action Surge (2 uses)" },
        { "key": "indomitable", "name": "Indomitable (3 uses)" }
      ]
    },
    {
      "level": 18,
      "features": [
        { "key": "martial_archetype", "name": "Martial Archetype Feature" }
      ]
    },
    {
      "level": 19,
      "features": [
        { "key": "fighter_ability_score" }
      ]
    },
    {
      "level": 20,
      "features": [
        { "key": "extra_attack", "name": "Extra Attack (3)" }
      ]
    }
  ],
  "features": {
    "fighter_fighting_style": {
      "name": "Fighting Style",
      "desc": "You adopt a particular style of fighting as your specialty. Choose one of the following options. You can't take a Fighting Style option more than once, even if you later get to choose again.\n## Archery You gain a +2 bonus to attack rolls you make with ranged weapons.\n## Defense While you are wearing armor, you gain a +1 bonus to AC.\n## Dueling When you are wielding a melee weapon in one hand and no other weapons, you gain a +2 bonus to damage rolls with that weapon.\n## Great Weapon Fighting When you roll a 1 or 2 on a damage die for an attack you make with a melee weapon that you are wielding with two hands, you can reroll the die and must use the new roll, even if the new roll is a 1 or a 2. The weapon must have the two_handed or versatile property for you to gain this benefit.\n## Protection When a creature you can see attacks a target other than you that is within 5 feet of you, you can use your reaction to impose disadvantage on the attack roll. You must be wielding a shield.\n## Two_Weapon Fighting When you engage in two_weapon fighting, you can add your ability modifier to the damage of the second attack. "
    },
    "second_wind": {
      "name": "Second Wind",
      "desc": "You have a limited well of stamina that you can draw on to protect yourself from harm. On your turn, you can use a bonus action to regain hit points equal to 1d10 + your fighter level. Once you use this feature, you must finish a short or long rest before you can use it again."
    },
    "action_surge": {
      "name": "Action Surge",
      "desc": "Starting at 2nd level, you can push yourself beyond your normal limits for a moment. On your turn, you can take one additional action on top of your regular action and a possible bonus action.\nOnce you use this feature, you must finish a short or long rest before you can use it again. Starting at 17th level, you can use it twice before a rest, but only once on the same turn. "
    },
    "martial_archetype": {
      "name": "Martial Archetype",
      "desc": "At 3rd level, you choose an archetype that you strive to emulate in your combat styles and techniques. Choose Champion, Battle Master, or Eldritch Knight, all detailed at the end of the class description. The archetype you choose grants you features at 3rd level and again at 7th, 10th, 15th, and 18th level.\nDifferent fighters choose different approaches to perfecting their fighting prowess. The martial archetype you choose to emulate reflects your approach."
    },
    "fighter_ability_score": {
      "name": "Ability Score Improvement",
      "desc": "When you reach 4th level, and again at 6th, 8th, 12th, 14th, 16th, and 19th level, you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1. As normal, you can't increase an ability score above 20 using this feature."
    },
    "fighter_extra_attack": {
      "name": "Extra Attack",
      "desc": "Beginning at 5th level, you can attack twice, instead of once, whenever you take the Attack action on your turn.\nThe number of attacks increases to three when you reach 11th level in this class and to four when you reach 20th level in this class."
    },
    "indomitable": {
      "name": "Indomitable",
      "desc": "Beginning at 9th level, you can reroll a saving throw that you fail. If you do so, you must use the new roll, and you can't use this feature again until you finish a long rest.\nYou can use this feature twice between long rests starting at 13th level and three times between long rests starting at 17th level."
    }
  }
}
"#;