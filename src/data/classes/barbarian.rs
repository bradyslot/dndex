pub const BARBARIAN_DATA: &str = r#"
{
  "class_hit_points": {
    "hit_dice": 12,
    "static_option": 7,
    "desc": "**Hit Dice:** 1d12 per barbarian level\n**Hit Points at 1st Level:** 12 + your Constitution modifier\n**Hit Points at Higher Levels:** 1d12 (or 7) + your Constitution modifier per barbarian level after 1st"
  },
  "class_proficiencies": {
    "armor": [
      { "category": "Light" },
      { "category": "Medium" },
      { "category": "Shield" }
    ],
    "weapons": [
      { "category": "Simple" },
      { "category": "Martial" }
    ],
    "tools": [],
    "saving_throws": [ "strength", "constitution" ],
    "skills": { "choices": 2, "options": [ "animal_handling", "athletics", "intimidation", "nature", "perception", "survival" ] },
    "desc": "**Armor:** Light armor, medium armor, shields\n**Weapons:** Simple weapons, martial weapons\n**Tools:** None\n**Saving Throws:** Strength, Constitution\n**Skills:** Choose two from Animal Handling, Athletics, Intimidation, Nature, Perception, and Survival"
  },
  "class_equipment": {
    "choice_1": [
      [
        { "source": "open5e", "location": "weapons", "key": "greataxe" }
      ],
      [
        { "source": "open5e", "location": "weapons", "category": "Martial Melee" }
      ]
    ],
    "choice_2": [
      [
        { "source": "open5e", "location": "weapons", "key": "handaxe", "qty": 2 }
      ],
      [
        { "source": "open5e", "location": "weapons", "category": "Simple Melee" }
      ]
    ],
    "choice_3": [],
    "choice_4": [],
    "defaults": [
      { "source": "data", "location": "equipment_packs", "key": "explorers_pack" },
      { "source": "open5e", "location": "weapons", "key": "javelin", "qty": 4 }
    ],
    "desc": "You start with the following equipment, in addition to the equipment granted by your background:\n- *(a)* a greataxe or *(b)* any martial melee weapon \n- *(a)* two handaxes or *(b)* any simple weapon\nAn explorer's pack and four javelins"
  },
  "class_spellcasting": {},
  "class_levels": [
    {
      "level": 1,
      "features": [
        { "key": "rage" },
        { "key": "unarmored_defense" }
      ],
      "rages": 2,
      "rage_damage": 2
    },
    {
      "level": 2,
      "features": [
        { "key": "reckless_attack" },
        { "key": "danger_sense" }
      ],
      "rages": 2,
      "rage_damage": 2
    },
    {
      "level": 3,
      "features": [
        { "key": "primal_path" }
      ],
      "rages": 3,
      "rage_damage": 2
    },
    {
      "level": 4,
      "features": [
        { "key": "ability_score" }
      ],
      "rages": 3,
      "rage_damage": 2
    },
    {
      "level": 5,
      "features": [
        { "key": "extra_attack" },
        { "key": "fast_movement" }
      ],
      "rages": 4,
      "rage_damage": 2
    },
    {
      "level": 6,
      "features": [
        { "key": "primal_path", "name": "Path Feature" }
      ],
      "rages": 4,
      "rage_damage": 2
    },
    {
      "level": 7,
      "features": [
        { "key": "feral_instinct" }
      ],
      "rages": 4,
      "rage_damage": 2
    },
    {
      "level": 8,
      "features": [
        { "key": "ability_score" }
      ],
      "rages": 4,
      "rage_damage": 2
    },
    {
      "level": 9,
      "features": [
        { "key": "brutal_critical", "name": "Brutal Critical (1 die)" }
      ],
      "rages": 4,
      "rage_damage": 3
    },
    {
      "level": 10,
      "features": [
        { "key": "primal_path", "name": "Path Feature" }
      ],
      "rages": 4,
      "rage_damage": 3
    },
    {
      "level": 11,
      "features": [
        { "key": "relentless_rage" }
      ],
      "rages": 4,
      "rage_damage": 3
    },
    {
      "level": 12,
      "features": [
        { "key": "ability_score" }
      ],
      "rages": 5,
      "rage_damage": 3
    },
    {
      "level": 13,
      "features": [
        { "key": "brutal_critical", "name": "Brutal Critical (2 dice)" }
      ],
      "rages": 5,
      "rage_damage": 3
    },
    {
      "level": 14,
      "features": [
        { "key": "primal_path", "name": "Path Feature" }
      ],
      "rages": 5,
      "rage_damage": 3
    },
    {
      "level": 15,
      "features": [
        { "key": "persistent_rage" }
      ],
      "rages": 5,
      "rage_damage": 3
    },
    {
      "level": 16,
      "features": [
        { "key": "ability_score" }
      ],
      "rages": 5,
      "rage_damage": 4
    },
    {
      "level": 17,
      "features": [
        { "key": "brutal_critical", "name": "Brutal Critical (3 dice)" }
      ],
      "rages": 5,
      "rage_damage": 4
    },
    {
      "level": 18,
      "features": [
        { "key": "indomitable_might" }
      ],
      "rages": 5,
      "rage_damage": 4
    },
    {
      "level": 19,
      "features": [
        { "key": "ability_score" }
      ],
      "rages": 5,
      "rage_damage": 4
    },
    {
      "level": 20,
      "features": [
        { "key": "primal_champion" }
      ],
      "rages": 0,
      "rage_damage": 4
    }
  ],
  "class_features": {
    "ability_score": {
      "name": "Ability Score Improvement",
      "desc": "When you reach 4th level, and again at 8th, 12th, 16th, and 19th level, you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1. As normal, you can't increase an ability score above 20 using this feature."
    },
    "rage": {
      "name": "Rage",
      "desc": "In battle, you fight with primal ferocity. On your turn, you can enter a rage as a bonus action.\nWhile raging, you gain the following benefits if you aren't wearing heavy armor:\nYou have advantage on Strength checks and Strength saving throws.\nWhen you make a melee weapon attack using Strength, you gain a bonus to the damage roll that increases as you gain levels as a barbarian, as shown in the Rage Damage column of the Barbarian table.\nYou have resistance to bludgeoning, piercing, and slashing damage.\nIf you are able to cast spells, you can't cast them or concentrate on them while raging.\nYour rage lasts for 1 minute. It ends early if you are knocked srd:unconscious or if your turn ends and you haven't attacked a hostile creature since your last turn or taken damage since then. You can also end your rage on your turn as a bonus action.\nOnce you have raged the number of times shown for your barbarian level in the Rages column of the Barbarian table, you must finish a long rest before you can rage again."
    },
    "unarmored_defense": {
      "name": "Unarmored Defense",
      "desc": "While you are not wearing any armor, your Armor Class equals 10 + your Dexterity modifier + your Constitution modifier. You can use a shield and still gain this benefit."
    },
    "reckless_attack": {
      "name": "Reckless Attack",
      "desc": "Starting at 2nd level, you can throw aside all concern for defense to attack with fierce desperation. When you make your first attack on your turn, you can decide to attack recklessly. Doing so gives you advantage on melee weapon attack rolls using Strength during this turn, but attack rolls against you have advantage until your next turn."
    },
    "danger_sense": {
      "name": "Danger Sense",
      "desc": "At 2nd level, you gain an uncanny sense of when things nearby aren't as they should be, giving you an edge when you dodge away from danger.\nYou have advantage on Dexterity saving throws against effects that you can see, such as traps and spells. To gain this benefit, you can't be [blinded]({{ base_url }}/conditions/blinded), [deafened]({{ base_url }}/conditions/deafened), or [incapacitated]({{ base_url }}/conditions/incapacitated)."
    },
    "primal_path": {
      "name": "Primal Path",
      "desc": "At 3rd level, you choose a path that shapes the nature of your rage. Choose the Path of the Berserker or the Path of the Totem Warrior, both detailed at the end of the class description. Your choice grants you features at 3rd level and again at 6th, 10th, and 14th levels."
    },
    "extra_attack": {
      "name": "Extra Attack",
      "desc": "Beginning at 5th level, you can attack twice, instead of once, whenever you take the Attack action on your turn."
    },
    "fast_movement": {
      "name": "Fast Movement",
      "desc": "Starting at 5th level, your speed increases by 10 feet while you aren't wearing heavy armor."
    },
    "feral_instinct": {
      "name": "Feral Instinct",
      "desc": "By 7th level, your instincts are so honed that you have advantage on initiative rolls.\nAdditionally, if you are surprised at the beginning of combat and aren't srd:incapacitated, you can act normally on your first turn, but only if you enter your rage before doing anything else on that turn."
    },
    "brutal_critical": {
      "name": "Brutal Critical",
      "desc": "Beginning at 9th level, you can roll one additional weapon damage die when determining the extra damage for a critical hit with a melee attack.\nThis increases to two additional dice at 13th level and three additional dice at 17th level."
    },
    "relentless_rage": {
      "name": "Relentless Rage",
      "desc": "Starting at 11th level, your rage can keep you fighting despite grievous wounds. If you drop to 0 hit points while you're raging and don't die outright, you can make a DC 10 Constitution saving throw. If you succeed, you drop to 1 hit point instead.\nEach time you use this feature after the first, the DC increases by 5. When you finish a short or long rest, the DC resets to 10. "
    },
    "persistent_rage": {
      "name": "Persistent Rage",
      "desc": "Beginning at 15th level, your rage is so fierce that it ends early only if you fall srd:unconscious or if you choose to end it."
    },
    "indomitable_might": {
      "name": "Indomitable Might",
      "desc": "Beginning at 18th level, if your total for a Strength check is less than your Strength score, you can use that score in place of the total."
    },
    "primal_champion": {
      "name": "Primal Champion",
      "desc": "At 20th level, you embody the power of the wilds. Your Strength and Constitution scores increase by 4. Your maximum for those scores is now 24."
    }
  }
}
"#;
