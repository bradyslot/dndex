#![allow(unused)]
pub const RANGER_DATA: &str = r#"
{
  "hit_points": {
    "hit_dice": 10,
    "static_option": 6,
    "desc": "**Hit Dice:** 1d10 per Ranger level\n**Hit Points at 1st Level:** 10 + your Constitution modifier\n**Hit Points at Higher Levels:** 1d10 (or 6) + your Constitution modifier per ranger level after 1st"
  },
  "proficiencies": {
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
    "saving_throws": [ "strength", "dexterity" ],
    "skills": { "choices": 3, "options": [ "animal_handling", "athletics", "insight", "investigation", "nature", "perception", "stealth", "survival" ] },
    "desc": "**Armor:** Light armor, medium armor, shields\n**Weapons:** Simple weapons, martial weapons\n**Tools:** None\n**Saving Throws:** Strength, Dexterity\n**Skills:** Choose three from Animal Handling, Athletics, Insight, Investigation, Nature, Perception, Stealth, and Survival"
  },
  "equipment": {
    "choice_1": [
      [
        { "source": "open5e", "location": "armor", "key": "scale-mail" }
      ],
      [
        { "source": "open5e", "location": "armor", "key": "leather" }
      ]
    ],
    "choice_2": [
      [
        { "source": "open5e", "location": "weapons", "key": "shortsword", "qty": 2 }
      ],
      [
        { "source": "open5e", "location": "weapons", "category": "Simple Melee", "qty": 2 }
      ]
    ],
    "choice_3": [
      [
        { "source": "data", "location": "equipment_packs", "key": "dungeoneers_pack" }
      ],
      [
        { "source": "data", "location": "equipment_packs", "key": "explorers_pack" }
      ]
    ],
    "choice_4": [],
    "defaults": [
      { "source": "open5e", "location": "weapons", "key": "longbow" },
      { "name": "Arrows", "qty": 20 }
    ],
    "desc": "You start with the following equipment, in addition to the equipment granted by your background:\n- *(a)* scale mail or *(b)* leather armor\n- *(a)* two shortswords or *(b)* two simple melee weapons\n- *(a)* a dungeoneer's pack or *(b)* an explorer's pack\n- A longbow and a quiver of 20 arrows"
  },
  "spellcasting": {
    "ability": "wisdom",
    "desc": "By the time you reach 2nd level, you have learned to use the magical essence of nature to cast spells, much as a druid does. See chapter 10 for the general rules of spellcasting and chapter 11 for the ranger spell list.\n## Spell Slots\nThe Ranger table shows how many spell slots you have to cast your spells of 1st level and higher. To cast one of these spells, you must expend a slot of the spell's level or higher. You regain all expended spell slots when you finish a long rest.\nFor example, if you know the 1st-level spell *animal friendship* and have a 1st-level and a 2nd-level spell slot available, you can cast *animal friendship* using either slot.\n## Spells Known of 1st Level and Higher\nYou know two 1st-level spells of your choice from the ranger spell list.\nThe Spells Known column of the Ranger table shows when you learn more ranger spells of your choice. Each of these spells must be of a level for which you have spell slots. For instance, when you reach 5th level in this class, you can learn one new spell of 1st or 2nd level.\nAdditionally, when you gain a level in this class, you can choose one of the ranger spells you know and replace it with another spell from the ranger spell list, which also must be of a level for which you have spell slots.\n## Spellcasting Ability\nWisdom is your spellcasting ability for your ranger spells, since your magic draws on your attunement to nature. You use your Wisdom whenever a spell refers to your spellcasting ability. In addition, you use your Wisdom modifier when setting the saving throw DC for a ranger spell you cast and when making an attack roll with one.\n**Spell save DC** = 8 + your proficiency bonus + your Wisdom modifier\n**Spell attack modifier** = your proficiency bonus + your Wisdom modifier"
  },
  "levels": [
    {
      "level": 1,
      "features": [
        { "key": "favored_enemy" },
        { "key": "natural_explorer" }
      ],
      "spells_known": 4,
      "spell_slots": [ 0, 0, 0, 0, 0, 0, 0, 0, 0 ]
    },
    {
      "level": 2,
      "features": [
        { "key": "ranger_fighting_style" },
        { "key": "spellcasting" }
      ],
      "spells_known": 2,
      "spell_slots": [ 2, 0, 0, 0, 0, 0, 0, 0, 0 ]
    },
    {
      "level": 3,
      "features": [
        { "key": "ranger_archetype" },
        { "key": "primeval_awareness" }
      ],
      "spells_known": 3,
      "spell_slots": [ 3, 0, 0, 0, 0, 0, 0, 0, 0 ]
    },
    {
      "level": 4,
      "features": [
        { "key": "ability_score" }
      ],
      "spells_known": 3,
      "spell_slots": [ 3, 0, 0, 0, 0, 0, 0, 0, 0 ]
    },
    {
      "level": 5,
      "features": [
        { "key": "ranger_extra_attack" }
      ],
      "spells_known": 4,
      "spell_slots": [ 4, 2, 0, 0, 0, 0, 0, 0, 0 ]
    },
    {
      "level": 6,
      "features": [
        { "key": "favored_enemy", "name": "Favored Enemy Improvement" },
        { "key": "natural_explorer", "name": "Natural Explorer Improvment" }
      ],
      "spells_known": 4,
      "spell_slots": [ 4, 2, 0, 0, 0, 0, 0, 0, 0 ]
    },
    {
      "level": 7,
      "features": [
        { "key": "ranger_archetype", "name": "Ranger Archetype Feature" }
      ],
      "spells_known": 5,
      "spell_slots": [ 4, 3, 0, 0, 0, 0, 0, 0, 0 ]
    },
    {
      "level": 8,
      "features": [
        { "key": "ability_score" },
        { "key": "lands_stride" }
      ],
      "spells_known": 5,
      "spell_slots": [ 4, 3, 0, 0, 0, 0, 0, 0, 0 ]
    },
    {
      "level": 9,
      "features": [],
      "spells_known": 6,
      "spell_slots": [ 4, 3, 2, 0, 0, 0, 0, 0, 0 ]
    },
    {
      "level": 10,
      "features": [
        { "key": "natural_explorer", "name": "Natural Explorer Improvement" },
        { "key": "hide_in_plain_sight" }
      ],
      "spells_known": 6,
      "spell_slots": [ 4, 3, 2, 0, 0, 0, 0, 0, 0 ]
    },
    {
      "level": 11,
      "features": [
        { "key": "ranger_archetype", "name": "Ranger Archetype Feature" }
      ],
      "spells_known": 7,
      "spell_slots": [ 4, 3, 3, 0, 0, 0, 0, 0, 0 ]
    },
    {
      "level": 12,
      "features": [
        { "key": "ability_score" }
      ],
      "spells_known": 7,
      "spell_slots": [ 4, 3, 3, 0, 0, 0, 0, 0, 0 ]
    },
    {
      "level": 13,
      "features": [],
      "spells_known": 8,
      "spell_slots": [ 4, 3, 3, 1, 0, 0, 0, 0, 0 ]
    },
    {
      "level": 14,
      "features": [
        { "key": "favored_enemy", "name": "Favored Enemy Improvement" },
        { "key": "vanish" }
      ],
      "spells_known": 8,
      "spell_slots": [ 4, 3, 3, 1, 0, 0, 0, 0, 0 ]
    },
    {
      "level": 15,
      "features": [
        { "key": "ranger_archetype", "name": "Ranger Archetype Feature" }
      ],
      "spells_known": 9,
      "spell_slots": [ 4, 3, 3, 2, 0, 0, 0, 0, 0 ]
    },
    {
      "level": 16,
      "features": [
        { "key": "ability_score" }
      ],
      "spells_known": 9,
      "spell_slots": [ 4, 3, 3, 2, 0, 0, 0, 0, 0 ]
    },
    {
      "level": 17,
      "features": [],
      "spells_known": 10,
      "spell_slots": [ 4, 3, 3, 3, 1, 0, 0, 0, 0 ]
    },
    {
      "level": 18,
      "features": [
        { "key": "feral_senses" }
      ],
      "spells_known": 10,
      "spell_slots": [ 4, 3, 3, 3, 1, 0, 0, 0, 0 ]
    },
    {
      "level": 19,
      "features": [
        { "key": "ability_score" }
      ],
      "spells_known": 11,
      "spell_slots": [ 4, 3, 3, 3, 2, 0, 0, 0, 0 ]
    },
    {
      "level": 20,
      "features": [
        { "key": "foe_slayer" }
      ],
      "spells_known": 11,
      "spell_slots": [ 4, 3, 3, 3, 2, 0, 0, 0, 0 ]
    }
  ],
  "features": {
    "ability_score": {
      "name": "Ability Score Improvement",
      "desc": "When you reach 4th level, and again at 8th, 12th, 16th, and 19th level, you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1. As normal, you can't increase an ability score above 20 using this feature."
    },
    "favored_enemy": {
      "name": "Favored Enemy",
      "desc": "Beginning at 1st level, you have significant experience studying, tracking, hunting, and even talking to a certain type of enemy.\nChoose a type of favored enemy: aberrations, beasts, celestials, constructs, dragons, elementals, fey, fiends, giants, monstrosities, oozes, plants, or undead. Alternatively, you can select two races of humanoid (such as gnolls and orcs) as favored enemies.\nYou have advantage on Wisdom (Survival) checks to track your favored enemies, as well as on Intelligence checks to recall information about them.\nWhen you gain this feature, you also learn one language of your choice that is spoken by your favored enemies, if they speak one at all.\nYou choose one additional favored enemy, as well as an associated language, at 6th and 14th level. As you gain levels, your choices should reflect the types of monsters you have encountered on your adventures."
    },
    "natural_explorer": {
      "name": "Natural Explorer",
      "desc": "You are particularly familiar with one type of natural environment and are adept at traveling and surviving in such regions. Choose one type of favored terrain: arctic, coast, desert, forest, grassland, mountain, or swamp. When you make an Intelligence or Wisdom check related to your favored terrain, your proficiency bonus is doubled if you are using a skill that you're proficient in.\nWhile traveling for an hour or more in your favored terrain, you gain the following benefits:\nDifficult terrain doesn't slow your group's travel.\nYour group can't become lost except by magical means.\nEven when you are engaged in another activity while traveling (such as foraging, navigating, or tracking), you remain alert to danger.\nIf you are traveling alone, you can move stealthily at a normal pace.\nWhen you forage, you find twice as much food as you normally would.\nWhile tracking other creatures, you also learn their exact number, their sizes, and how long ago they passed through the area.\nYou choose additional favored terrain types at 6th and 10th level."
    },
    "ranger_fighting_style": {
      "name": "Fighting Style",
      "desc": "At 2nd level, you adopt a particular style of fighting as your specialty. Choose one of the following options. You can't take a Fighting Style option more than once, even if you later get to choose again.\n## Archery You gain a +2 bonus to attack rolls you make with ranged weapons.\n## Defense While you are wearing armor, you gain a +1 bonus to AC.\n## Dueling When you are wielding a melee weapon in one hand and no other weapons, you gain a +2 bonus to damage rolls with that weapon.\n## Two_Weapon Fighting When you engage in two_weapon fighting, you can add your ability modifier to the damage of the second attack."
    },
    "ranger_archetype": {
      "name": "Ranger Archetype",
      "desc": "At 3rd level, you choose an archetype that you strive to emulate: Hunter or Beast Master, both detailed at the end of the class description. Your choice grants you features at 3rd level and again at 7th, 11th, and 15th level."
    },
    "primeval_awareness": {
      "name": "Primeval Awareness",
      "desc": "Beginning at 3rd level, you can use your action and expend one ranger spell slot to focus your awareness on the region around you. For 1 minute per level of the spell slot you expend, you can sense whether the following types of creatures are present within 1 mile of you (or within up to 6 miles if you are in your favored terrain): aberrations, celestials, dragons, elementals, fey, fiends, and undead. This feature doesn't reveal the creatures' location or number."
    },
    "ranger_extra_attack": {
      "name": "Extra Attack",
      "desc": "Beginning at 5th level, you can attack twice, instead of once, whenever you take the Attack action on your turn."
    },
    "lands_stride": {
      "name": "Land's Stride",
      "desc": "Starting at 8th level, moving through nonmagical difficult terrain costs you no extra movement. You can also pass through nonmagical plants without being slowed by them and without taking damage from them if they have thorns, spines, or a similar hazard.\nIn addition, you have advantage on saving throws against plants that are magically created or manipulated to impede movement, such those created by the *entangle* spell."
    },
    "hide_in_plain_sight": {
      "name": "Hide in Plain Sight",
      "desc": "Starting at 10th level, you can spend 1 minute creating camouflage for yourself. You must have access to fresh mud, dirt, plants, soot, and other naturally occurring materials with which to create your camouflage.\nOnce you are camouflaged in this way, you can try to hide by pressing yourself up against a solid surface, such as a tree or wall, that is at least as tall and wide as you are. You gain a +10 bonus to Dexterity (Stealth) checks as long as you remain there without moving or taking actions. Once you move or take an action or a reaction, you must camouflage yourself again to gain this benefit."
    },
    "vanish": {
      "name": "Vanish",
      "desc": "Starting at 14th level, you can use the Hide action as a bonus action on your turn. Also, you can't be tracked by nonmagical means, unless you choose to leave a trail."
    },
    "feral_senses": {
      "name": "Feral Senses",
      "desc": "At 18th level, you gain preternatural senses that help you fight creatures you can't see. When you attack a creature you can't see, your inability to see it doesn't impose disadvantage on your attack rolls against it.\nYou are also aware of the location of any invisible creature within 30 feet of you, provided that the creature isn't hidden from you and you aren't blinded or deafened."
    },
    "foe_slayer": {
      "name": "Foe Slayer",
      "desc": "At 20th level, you become an unparalleled hunter of your enemies. Once on each of your turns, you can add your Wisdom modifier to the attack roll or the damage roll of an attack you make against one of your favored enemies. You can choose to use this feature before or after the roll, but before any effects of the roll are applied."
    }
  }
}
"#;