#![allow(unused)]
pub const ROGUE_DATA: &str = r#"
{
  "hit_points": {
    "hit_dice": 8,
    "static_option": 5,
    "desc": "**Hit Dice:** 1d8 per Rogue level\n**Hit Points at 1st Level:** 8 + your Constitution modifier\n**Hit Points at Higher Levels:** 1d8 (or 5) + your Constitution modifier per rogue level after 1st"
  },
  "proficiencies": {
    "armor": [
      { "category": "Light" }
    ],
    "weapons": [
      { "category": "Simple" },
      { "key": "crowsbow-hand" },
      { "key": "longsword" },
      { "key": "rapier" },
      { "key": "shortsword" }
    ],
    "tools": [
      [
        { "key": "thieves_tools" }
      ]
    ],
    "saving_throws": [ "dexterity", "intelligence" ],
    "skills": { "choices": 4, "options": [ "acrobatics", "athletics", "deception", "insight", "intimidation", "investigation", "perception", "performance", "persuasion", "sleight_of_hand", "stealth" ] },
    "desc": "**Armor:** Light armor\n**Weapons:** Simple weapons, hand crossbows, longswords, rapiers, shortswords\n**Tools:** Thieves' tools\n**Saving Throws:** Dexterity, Intelligence\n**Skills:** Choose four from Acrobatics, Athletics, Deception, Insight, Intimidation, Investigation, Perception, Performance, Persuasion, Sleight of Hand, and Stealth"
  },
  "equipment": {
    "choice_1": [
      [
        { "source": "open5e", "location": "weapons", "key": "rapier" }
      ],
      [
        { "source": "open5e", "location": "weapons", "key": "shortsword" }
      ]
    ],
    "choice_2": [
      [
        { "source": "open5e", "location": "weapons", "key": "shortbow" },
        { "name": "Arrows", "qty": 20 }
      ],
      [
        { "source": "open5e", "location": "weapons", "key": "shortsword" }
      ]
    ],
    "choice_3": [
      [
        { "source": "data", "location": "equipment_packs", "key": "burglars_pack" }
      ],
      [
        { "source": "data", "location": "equipment_packs", "key": "dungeoneers_pack" }
      ],
      [
        { "source": "data", "location": "equipment_packs", "key": "explorers_pack" }
      ]
    ],
    "choice_4": [],
    "defaults": [
      { "source": "open5e", "location": "armor", "key": "leather" },
      { "source": "open5e", "location": "weapons", "key": "dagger", "qty": 2},
      { "source": "open5e", "location": "tools", "key": "thieves-tools" }
    ],
    "desc": "You start with the following equipment, in addition to the equipment granted by your background:\n- *(a)* a rapier or *(b)* a shortsword\n- *(a)* a shortbow and quiver of 20 arrows or *(b)* a shortsword\n- *(a)* a burglar's pack, *(b)* a dungeoneer's pack, or *(c)* an explorer's pack\n- *(a)* Leather armor, two daggers, and thieves' tools"
  },
  "spellcasting": {},
  "levels": [
    {
      "level": 1,
      "features": [
        { "key": "rogue_expertise" },
        { "key": "sneak_attack" },
        { "key": "thieves_cant" }
      ],
      "sneak_attack": 2
    },
    {
      "level": 2,
      "features": [
        { "key": "cunning_action" }
      ],
      "sneak_attack": 2
    },
    {
      "level": 3,
      "features": [
        { "key": "roguish_archetype" }
      ],
      "sneak_attack": 3
    },
    {
      "level": 4,
      "features": [
        { "key": "ability_score" }
      ],
      "sneak_attack": 3
    },
    {
      "level": 5,
      "features": [
        { "key": "uncanny_dodge" }
      ],
      "sneak_attack": 4
    },
    {
      "level": 6,
      "features": [
        { "key": "rogue_expertise" }
      ],
      "sneak_attack": 4
    },
    {
      "level": 7,
      "features": [
        { "key": "rogue_evasion" }
      ],
      "sneak_attack": 4
    },
    {
      "level": 8,
      "features": [
        { "key": "ability_score" }
      ],
      "sneak_attack": 4
    },
    {
      "level": 9,
      "features": [
        { "key": "roguish_archetype", "name": "Roguish Archetype Feature" }
      ],
      "sneak_attack": 4
    },
    {
      "level": 10,
      "features": [
        { "key": "ability_score" }
      ],
      "sneak_attack": 4
    },
    {
      "level": 11,
      "features": [
        { "key": "reliable_talent" }
      ],
      "sneak_attack": 4
    },
    {
      "level": 12,
      "features": [
        { "key": "ability_score" }
      ],
      "sneak_attack": 5
    },
    {
      "level": 13,
      "features": [
        { "key": "roguish_archetype", "name": "Roguish Archetype Feature" }
      ],
      "sneak_attack": 5
    },
    {
      "level": 14,
      "features": [
        { "key": "blindsense" }
      ],
      "sneak_attack": 5
    },
    {
      "level": 15,
      "features": [
        { "key": "slippery_mind" }
      ],
      "sneak_attack": 5
    },
    {
      "level": 16,
      "features": [
        { "key": "ability_score" }
      ],
      "sneak_attack": 5
    },
    {
      "level": 17,
      "features": [
        { "key": "roguish_archetype", "name": "Roguish Archetype Feature" }
      ],
      "sneak_attack": 5
    },
    {
      "level": 18,
      "features": [
        { "key": "elusive" }
      ],
      "sneak_attack": 5
    },
    {
      "level": 19,
      "features": [
        { "key": "ability_score" }
      ],
      "sneak_attack": 5
    },
    {
      "level": 20,
      "features": [
        { "key": "stroke_of_luck" }
      ],
      "sneak_attack": 0
    }
  ],
  "features": {
    "ability_score": {
      "name": "Ability Score Improvement",
      "desc": "When you reach 4th level, and again at 8th, 12th, 16th, and 19th level, you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1. As normal, you can't increase an ability score above 20 using this feature."
    },
    "rogue_expertise": {
      "name": "Expertise",
      "desc": "At 1st level, choose two of your skill proficiencies, or one of your skill proficiencies and your proficiency with thieves' tools. Your proficiency bonus is doubled for any ability check you make that uses either of the chosen proficiencies.\nAt 6th level, you can choose two more of your proficiencies (in skills or with thieves' tools) to gain this benefit."
    },
    "sneak_attack": {
      "name": "Sneak Attack",
      "desc": "Beginning at 1st level, you know how to strike subtly and exploit a foe's distraction. Once per turn, you can deal an extra 1d6 damage to one creature you hit with an attack if you have advantage on the attack roll. The attack must use a finesse or a ranged weapon.\nYou don't need advantage on the attack roll if another enemy of the target is within 5 feet of it, that enemy isn't incapacitated, and you don't have disadvantage on the attack roll.\nThe amount of the extra damage increases as you gain levels in this class, as shown in the Sneak Attack column of the Rogue table."
    },
    "thieves_cant": {
      "name": "Thieves' Cant",
      "desc": "During your rogue training you learned thieves' cant, a secret mix of dialect, jargon, and code that allows you to hide messages in seemingly normal conversation. Only another creature that knows thieves' cant understands such messages. It takes four times longer to convey such a message than it does to speak the same idea plainly.\nIn addition, you understand a set of secret signs and symbols used to convey short, simple messages, such as whether an area is dangerous or the territory of a thieves' guild, whether loot is nearby, or whether the people in an area are easy marks or will provide a safe house for thieves on the run."
    },
    "cunning_action": {
      "name": "Cunning Action",
      "desc": "Starting at 2nd level, your quick thinking and agility allow you to move and act quickly. You can take a bonus action on each of your turns in combat. This action can be used only to take the Dash, Disengage, or Hide action."
    },
    "roguish_archetype": {
      "name": "Roguish Archetype",
      "desc": "At 3rd level, you choose an archetype that you emulate in the exercise of your rogue abilities: Thief, Assassin, or Arcane Trickster, all detailed at the end of the class description. Your archetype choice grants you features at 3rd level and then again at 9th, 13th, and 17th level.\nRogues have many features in common, including their emphasis on perfecting their skills, their precise and deadly approach to combat, and their increasingly quick reflexes. But different rogues steer those talents in varying directions, embodied by the rogue archetypes. Your choice of archetype is a reflection of your focus_not necessarily an indication of your chosen profession, but a description of your preferred techniques."
    },
    "uncanny_dodge": {
      "name": "Uncanny Dodge",
      "desc": "Starting at 5th level, when an attacker that you can see hits you with an attack, you can use your reaction to halve the attack's damage against you."
    },
    "rogue_evasion": {
      "name": "Evasion",
      "desc": "Beginning at 7th level, you can nimbly dodge out of the way of certain area effects, such as a red dragon's fiery breath or an ice storm spell. When you are subjected to an effect that allows you to make a Dexterity saving throw to take only half damage, you instead take no damage if you succeed on the saving throw, and only half damage if you fail."
    },
    "reliable_talent": {
      "name": "Reliable Talent",
      "desc": "By 11th level, you have refined your chosen skills until they approach perfection. Whenever you make an ability check that lets you add your proficiency bonus, you can treat a d20 roll of 9 or lower as a 10."
    },
    "blindsense": {
      "name": "Blindsense",
      "desc": "Starting at 14th level, if you are able to hear, you are aware of the location of any hidden or invisible creature within 10 feet of you."
    },
    "slippery_mind": {
      "name": "Slippery Mind",
      "desc": "By 15th level, you have acquired greater mental strength. You gain proficiency in Wisdom saving throws."
    },
    "elusive": {
      "name": "Elusive",
      "desc": "Beginning at 18th level, you are so evasive that attackers rarely gain the upper hand against you. No attack roll has advantage against you while you aren't incapacitated."
    },
    "stroke_of_luck": {
      "name": "Stroke of Luck",
      "desc": "At 20th level, you have an uncanny knack for succeeding when you need to. If your attack misses a target within range, you can turn the miss into a hit. Alternatively, if you fail an ability check, you can treat the d20 roll as a 20.\nOnce you use this feature, you can't use it again until you finish a short or long rest."
    }
  }
}
"#;