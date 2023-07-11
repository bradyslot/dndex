#![allow(unused)]
pub const BARD_DATA: &str = r#"
{
  "hit_points": {
    "hit_dice": 8,
    "static_option": 5,
    "desc": "**Hit Dice:** 1d8 per bard level\n**Hit Points at 1st Level:** 8 + your Constitution modifier\n**Hit Points at Higher Levels:** 1d8 (or 5) + your Constitution modifier per bard level after 1st"
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
        { "key": "musical_instrument" }
      ]
    ],
    "saving_throws": [ "dexterity", "charisma" ],
    "skills": { "choices": 3, "options": [] },
    "desc": "**Armor:** Light armor\n**Weapons:** Simple weapons, hand crossbows, longswords, rapiers, shortswords\n**Tools:** Three musical instruments of your choice\n**Saving Throws:** Dexterity, Charisma\n**Skills:** Choose any three"
  },
  "equipment": {
    "choice_1": [
      [
        { "source": "open5e", "location": "weapons", "key": "rapier" }
      ],
      [
        { "source": "open5e", "location": "weapons", "key": "longsword" }
      ],
      [
        { "source": "open5e", "location": "weapons", "category": "Simple Melee" }
      ]
    ],
    "choice_2": [
      [
        { "source": "open5e", "location": "equipment_packs", "key": "diplomats_pack" }
      ],
      [
        { "source": "open5e", "location": "equipment_packs", "key": "entertainers_pack" }
      ]
    ],
    "choice_3": [
      [
        { "source": "data", "location": "tools", "key": "lute" }
      ],
      [
        { "source": "data", "location": "tools", "key": "musical_instrument" }
      ]
    ],
    "choice_4": [],
    "defaults": [
      { "source": "open5e", "location": "weapons", "key": "dagger" },
      { "source": "open5e", "location": "weapons", "key": "leather" }
    ],
    "desc": "You start with the following equipment, in addition to the equipment granted by your background:\n- *(a)* a rapier, *(b)* a longsword, or *(c)* any simple weapon\n- *(a)* a diplomat's pack or *(b)* an entertainer's pack\n- *(a)* a lute or *(b)* any other musical instrument\n- Leather armor and a dagger"
  },
  "spellcasting": {
    "ability": "charisma",
    "desc": "You have learned to untangle and reshape the fabric of reality in harmony with your wishes and music.\nYour spells are part of your vast repertoire, magic that you can tune to different situations.\n## Cantrips\nYou know two cantrips of your choice from the [Bard Spell List]({{ base_url }}/spells/by-class/bard). You learn additional bard cantrips of your choice at higher levels, as shown in the Cantrips Known column of the Bard table.\n## Spell Slots\nThe Bard table shows how many spell slots you have to cast your spells of 1st level and higher. To cast one of these spells, you must expend a slot of the spell's level or higher. You regain all expended spell slots when you finish a long rest.\nFor example, if you know the 1st-level spell cure wounds and have a 1st-level and a 2nd-level spell slot available, you can cast cure wounds using either slot.\n## Spells Known of 1st Level and Higher\nYou know four 1st-level spells of your choice from the [Bard Spell List]({{ base_url }}/spells/by-class/bard).\nThe Spells Known column of the Bard table shows when you learn more bard spells of your choice. Each of these spells must be of a level for which you have spell slots, as shown on the table. For instance, when you reach 3rd level in this class, you can learn one new spell of 1st or 2nd level.\nAdditionally, when you gain a level in this class, you can choose one of the bard spells you know and replace it with another spell from the bard spell list, which also must be of a level for which you have spell slots.\n## Spellcasting Ability\nCharisma is your spellcasting ability for your bard spells. Your magic comes from the heart and soul you pour into the performance of your music or oration. You use your Charisma whenever a spell refers to your spellcasting ability. In addition, you use your Charisma modifier when setting the saving throw DC for a bard spell you cast and when making an attack roll with one.\n**Spell save DC** = 8 + your proficiency bonus + your Charisma modifier\n**Spell attack modifier** = your proficiency bonus + your Charisma modifier\n## Ritual Casting\nYou can cast any bard spell you know as a ritual if that spell has the ritual tag.\n## Spellcasting Focus\nYou can use a musical instrument (see [Adventuring Gear]({{ base_url }}/sections/adventuring-gear)) as a spellcasting focus for your bard spells."
  },
  "levels": [
    {
      "level": 1,
      "features": [
        { "key": "spellcasting" },
        { "key": "bardic_inspiration", "name": "Bardic Inspiration (d6)" }
      ],
      "cantrips_known": 2,
      "spells_known": 4,
      "spell_slots": [ 2, 0, 0, 0, 0, 0, 0, 0, 0 ]
    },
    {
      "level": 2,
      "features": [
        { "key": "jack_of_all_trades" },
        { "key": "song_of_rest", "name": "Song of Rest (d6)" }
      ],
      "cantrips_known": 2,
      "spells_known": 5,
      "spell_slots": [ 3, 0, 0, 0, 0, 0, 0, 0, 0 ]
    },
    {
      "level": 3,
      "features": [
        { "key": "bard_college" },
        { "key": "expertise" }
      ],
      "cantrips_known": 3,
      "spells_known": 6,
      "spell_slots": [ 4, 2, 0, 0, 0, 0, 0, 0, 0 ]
    },
    {
      "level": 4,
      "features": [
        { "key": "ability_score" }
      ],
      "cantrips_known": 3,
      "spells_known": 7,
      "spell_slots": [ 4, 3, 0, 0, 0, 0, 0, 0, 0 ]
    },
    {
      "level": 5,
      "features": [
        { "key": "bardic_inspiration", "name": "Bardic Inspiration (d8)" },
        { "key": "font_of_inspiration" }
      ],
      "cantrips_known": 3,
      "spells_known": 8,
      "spell_slots": [ 4, 3, 2, 0, 0, 0, 0, 0, 0 ]
    },
    {
      "level": 6,
      "features": [
        { "key": "countercharm" },
        { "key": "bard_college", "name": "Bard College Feature" }
      ],
      "cantrips_known": 3,
      "spells_known": 9,
      "spell_slots": [ 4, 3, 3, 0, 0, 0, 0, 0, 0 ]
    },
    {
      "level": 7,
      "features": [],
      "cantrips_known": 3,
      "spells_known": 10,
      "spell_slots": [ 4, 3, 3, 1, 0, 0, 0, 0, 0 ]
    },
    {
      "level": 8,
      "features": [
        { "key": "ability_score" }
      ],
      "cantrips_known": 3,
      "spells_known": 11,
      "spell_slots": [ 4, 3, 3, 2, 0, 0, 0, 0, 0 ]
    },
    {
      "level": 9,
      "features": [
        { "key": "song_of_rest", "name": "Song of Rest (d8)" }
      ],
      "cantrips_known": 3,
      "spells_known": 12,
      "spell_slots": [ 4, 3, 3, 3, 1, 0, 0, 0, 0 ]
    },
    {
      "level": 10,
      "features": [
        { "key": "bardic_inspiration", "name": "Bardic Inspiration (d10)" },
        { "key": "expertise", "name": "Expertise Upgrade" },
        { "key": "magical_secrets" }
      ],
      "cantrips_known": 4,
      "spells_known": 14,
      "spell_slots": [ 4, 3, 3, 3, 2, 0, 0, 0, 0 ]
    },
    {
      "level": 11,
      "features": [],
      "cantrips_known": 4,
      "spells_known": 15,
      "spell_slots": [ 4, 3, 3, 3, 2, 1, 0, 0, 0 ]
    },
    {
      "level": 12,
      "features": [
        { "key": "ability_score" }
      ],
      "cantrips_known": 4,
      "spells_known": 15,
      "spell_slots": [ 4, 3, 3, 3, 2, 1, 0, 0, 0 ]
    },
    {
      "level": 13,
      "features": [
        { "key": "song_of_rest", "name": "Song of Rest (d10)" }
      ],
      "cantrips_known": 4,
      "spells_known": 16,
      "spell_slots": [ 4, 3, 3, 3, 2, 1, 1, 0, 0 ]
    },
    {
      "level": 14,
      "features": [
        { "key": "magical_secrets" },
        { "key": "bard_college", "name": "Bard College Feature" }
      ],
      "cantrips_known": 4,
      "spells_known": 18,
      "spell_slots": [ 4, 3, 3, 3, 2, 1, 1, 0, 0 ]
    },
    {
      "level": 15,
      "features": [
        { "key": "bardic_inspiration", "name": "Bardic Inspiration (d12)" }
      ],
      "cantrips_known": 4,
      "spells_known": 19,
      "spell_slots": [ 4, 3, 3, 3, 2, 1, 1, 1, 0 ]
    },
    {
      "level": 16,
      "features": [
        { "key": "ability_score" }
      ],
      "cantrips_known": 4,
      "spells_known": 19,
      "spell_slots": [ 4, 3, 3, 3, 2, 1, 1, 1, 0 ]
    },
    {
      "level": 17,
      "features": [
        { "key": "song_of_rest", "name": "Song of Rest (d12)" }
      ],
      "cantrips_known": 4,
      "spells_known": 20,
      "spell_slots": [ 4, 3, 3, 3, 2, 1, 1, 1, 1 ]
    },
    {
      "level": 18,
      "features": [
        { "key": "magical_secrets", "name": "Magical Secrets Upgrade" }
      ],
      "cantrips_known": 4,
      "spells_known": 22,
      "spell_slots": [ 4, 3, 3, 3, 3, 1, 1, 1, 1 ]
    },
    {
      "level": 19,
      "features": [
        { "key": "ability_score" }
      ],
      "cantrips_known": 4,
      "spells_known": 22,
      "spell_slots": [ 4, 3, 3, 3, 3, 2, 1, 1, 1 ]
    },
    {
      "level": 20,
      "features": [
        { "key": "superior_inspiration" }
      ],
      "cantrips_known": 4,
      "spells_known": 22,
      "spell_slots": [ 4, 3, 3, 3, 3, 2, 2, 1, 1 ]
    }
  ],
  "features": {
    "ability_score": {
      "name": "Ability Score Improvement",
      "desc": "When you reach 4th level, and again at 8th, 12th, 16th, and 19th level, you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1. As normal, you can't increase an ability score above 20 using this feature."
    },
    "bardic_inspiration": {
      "name": "Bardic Inspiration",
      "desc": "You can inspire others through stirring words or music. To do so, you use a bonus action on your turn to choose one creature other than yourself within 60 feet of you who can hear you. That creature gains one Bardic Inspiration die, a d6.\nOnce within the next 10 minutes, the creature can roll the die and add the number rolled to one ability check, attack roll, or saving throw it makes. The creature can wait until after it rolls the d20 before deciding to use the Bardic Inspiration die, but must decide before the GM says whether the roll succeeds or fails. Once the Bardic Inspiration die is rolled, it is lost. A creature can have only one Bardic Inspiration die at a time.\nYou can use this feature a number of times equal to your Charisma modifier (a minimum of once). You regain any expended uses when you finish a long rest.\nYour Bardic Inspiration die changes when you reach certain levels in this class. The die becomes a d8 at 5th level, a d10 at 10th level, and a d12 at 15th level."
    },
    "jack_of_all_trades": {
      "name": "Jack of All Trades",
      "desc": "Jack of all trades applies only to ability checks, not attack rolls or saving throws. However since initiative rolls are Dexterity checks Jack of All Trades does apply to initiative rolls as long as it is not already benefiting from the character's proficiency bonus.\nStarting at 2nd level, you can add half your proficiency bonus, rounded down, to any ability check you make that doesn't already include your proficiency bonus."
    },
    "song_of_rest": {
      "name": "Song of Rest",
      "desc": "Beginning at 2nd level, you can use soothing music or oration to help revitalize your wounded allies during a short rest. If you or any friendly creatures who can hear your performance regain hit points at the end of the short rest by spending one or more Hit Dice, each of those creatures regains an extra 1d6 hit points.\nThe extra hit points increase when you reach certain levels in this class: to 1d8 at 9th level, to 1d10 at 13th level, and to 1d12 at 17th level."
    },
    "bard_college": {
      "name": "Bard College",
      "desc": "At 3rd level, you delve into the advanced techniques of a bard college of your choice, such as the College of Lore. Your choice grants you features at 3rd level and again at 6th and 14th level."
    },
    "expertise": {
      "name": "Expertise",
      "desc": "At 3rd level, choose two of your skill proficiencies. Your proficiency bonus is doubled for any ability check you make that uses either of the chosen proficiencies.\nAt 10th level, you can choose another two skill proficiencies to gain this benefit."
    },
    "font_of_inspiration": {
      "name": "Font of Inspiration",
      "desc": "Beginning when you reach 5th level, you regain all of your expended uses of Bardic Inspiration when you finish a short or long rest."
    },
    "countercharm": {
      "name": "Countercharm",
      "desc": "At 6th level, you gain the ability to use musical notes or words of power to disrupt mind-influencing effects. As an action, you can start a performance that lasts until the end of your next turn. During that time, you and any friendly creatures within 30 feet of you have advantage on saving throws against being [frightened]({{ base_url }}/conditions/frightened) or [charmed]({{ base_url }}/conditions/charmed). A creature must be able to hear you to gain this benefit. The performance ends early if you are [incapacitated]({{ base_url }}/conditions/incapacitated) or silenced or if you voluntarily end it (no action required)."
    },
    "magical_secrets": {
      "name": "Magical Secrets",
      "desc": "By 10th level, you have plundered magical knowledge from a wide spectrum of disciplines. Choose two spells from any class, including this one. A spell you choose must be of a level you can cast, as shown on the Bard table, or a cantrip.\nThe chosen spells count as bard spells for you and are included in the number in the Spells Known column of the Bard table.\nYou learn two additional spells from any class at 14th level and again at 18th level."
    },
    "superior_inspiration": {
      "name": "Superior Inspiration",
      "desc": "At 20th level, when you roll initiative and have no uses of Bardic Inspiration left, you regain one use."
    }
  }
}
"#;