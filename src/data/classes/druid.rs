#![allow(unused)]
// pub const DRUID_DATA: &str = r#"
// {
//   "hit_points": {
//     "hit_dice": 8,
//     "static_option": 5,
//     "desc": "**Hit Dice:** 1d8 per druid level\n**Hit Points at 1st Level:** 8 + your Constitution modifier\n**Hit Points at Higher Levels:** 1d8 (or 5) + your Constitution modifier per druid level after 1st"
//   },
//   "proficiencies": {
//     "armor": [
//       { "category": "Light" },
//       { "category": "Medium" },
//       { "category": "Shield" }
//     ],
//     "weapons": [
//       { "key": "club" },
//       { "key": "dagger" },
//       { "key": "dart" },
//       { "key": "javelin" },
//       { "key": "mace" },
//       { "key": "quarterstaff" },
//       { "key": "scimitar" },
//       { "key": "sickle" },
//       { "key": "sling" },
//       { "key": "spear" }
//     ],
//     "tools": [
//       [
//         { "key": "herbalism_kit" }
//       ]
//     ],
//     "saving_throws": [ "intelligence", "wisdom" ],
//     "skills": { "choices": 2, "options": [ "arcana", "animal_handling", "insight", "medicine", "nature", "perception", "religion", "survival" ] },
//     "desc": "**Armor:** Light armor, medium armor, shields (druids will not wear armor or use shields made of metal)\n**Weapons:** Clubs, daggers, darts, javelins, maces, quarterstaffs, scimitars, sickles, slings, spears\n**Tools:** Herbalism kit\n**Saving Throws:** Intelligence, Wisdom\n**Skills:** Choose two from Arcana, Animal Handling, Insight, Medicine, Nature, Perception, Religion, and Survival"
//   },
//   "equipment": {
//     "choice_1": [
//       [
//         { "source": "open5e", "location": "armor", "key": "shield" }
//       ],
//       [
//         { "source": "open5e", "location": "weapons", "category": "Simple" }
//       ]
//     ],
//     "choice_2": [
//       [
//         { "source": "open5e", "location": "weapons", "key": "scimitar" }
//       ],
//       [
//         { "source": "open5e", "location": "weapons", "category": "Simple Melee" }
//       ]
//     ],
//     "choice_3": [],
//     "choice_4": [],
//     "defaults": [
//       { "source": "open5e", "location": "armor" , "key": "leather" },
//       { "source": "data", "location": "explorers_packs", "key": "explorers_pack" },
//       { "source": "data", "location": "adventuring_gear", "key": "druidic_focus" }
//     ],
//     "desc": "You start with the following equipment, in addition to the equipment granted by your background:\n- *(a)* a wooden shield or *(b)* any simple weapon\n- *(a)* a scimitar or *(b)* any simple melee weapon\n- Leather armor, an explorer's pack, and a druidic focus"
//   },
//   "spellcasting": {
//     "ability": "wisdom",
//     "desc": "Drawing on the divine essence of nature itself, you can cast spells to shape that essence to your will.\n## Cantrips At 1st level, you know two cantrips of your choice from the druid spell list. You learn additional druid cantrips of your choice at higher levels, as shown in the Cantrips Known column of the Druid table.\n## Preparing and Casting Spells The Druid table shows how many spell slots you have to cast your spells of 1st level and higher. To cast one of these druid spells, you must expend a slot of the spell's level or higher. You regain all expended spell slots when you finish a long rest.\nYou prepare the list of druid spells that are available for you to cast, choosing from the druid spell list. When you do so, choose a number of druid spells equal to your Wisdom modifier + your druid level (minimum of one spell). The spells must be of a level for which you have spell slots.\nFor example, if you are a 3rd-level druid, you have four 1st-level and two 2nd-level spell slots. With a Wisdom of 16, your list of prepared spells can include six spells of 1st or 2nd level, in any combination. If you prepare the 1st-level spell *cure wounds*, you can cast it using a 1st-level or 2nd-level slot. Casting the spell doesn't remove it from your list of prepared spells.\nYou can also change your list of prepared spells when you finish a long rest. Preparing a new list of druid spells requires time spent in prayer and meditation: at least 1 minute per spell level for each spell on your list.\n## Spellcasting Ability Wisdom is your spellcasting ability for your druid spells, since your magic draws upon your devotion and attunement to nature. You use your Wisdom whenever a spell refers to your spellcasting ability. In addition, you use your Wisdom modifier when setting the saving throw DC for a druid spell you cast and when making an attack roll with one.\n**Spell save DC** = 8 + your proficiency bonus + your Wisdom modifier\n**Spell attack modifier** = your proficiency bonus + your Wisdom modifier\n## Ritual Casting You can cast a druid spell as a ritual if that spell has the ritual tag and you have the spell prepared.\n## Spellcasting Focus You can use a druidic focus (see [Adventuring Gear]({{ base_url }}/equipment/adventuring-gear)) as a spellcasting focus for your druid spells."
//   },
//   "levels": [
//     {
//       "level": 1,
//       "features": [
//         { "key": "druidic" },
//         { "key": "spellcasting" }
//       ],
//       "cantrips_known": 3,
//       "spell_slots": [ 2, 0, 0, 0, 0, 0, 0, 0, 0 ]
//     },
//     {
//       "level": 2,
//       "features": [
//         { "key": "wild_shape" },
//         { "key": "druid_circle" }
//       ],
//       "cantrips_known": 3,
//       "spell_slots": [ 3, 0, 0, 0, 0, 0, 0, 0, 0 ]
//     },
//     {
//       "level": 3,
//       "features": [],
//       "cantrips_known": 3,
//       "spell_slots": [ 4, 2, 0, 0, 0, 0, 0, 0, 0 ]
//     },
//     {
//       "level": 4,
//       "features": [
//         { "key": "wild_shape", "name": "Wild Shape Improvement" },
//         { "key": "ability_score" }
//       ],
//       "cantrips_known": 3,
//       "spell_slots": [ 4, 3, 0, 0, 0, 0, 0, 0, 0 ]
//     },
//     {
//       "level": 5,
//       "features": [],
//       "cantrips_known": 3,
//       "spell_slots": [ 4, 3, 2, 0, 0, 0, 0, 0, 0 ]
//     },
//     {
//       "level": 6,
//       "features": [
//         { "key": "druid_circle", "name": "Druid Circle Feature" }
//       ],
//       "cantrips_known": 3,
//       "spell_slots": [ 4, 3, 3, 0, 0, 0, 0, 0, 0 ]
//     },
//     {
//       "level": 7,
//       "features": [],
//       "cantrips_known": 3,
//       "spell_slots": [ 4, 3, 3, 1, 0, 0, 0, 0, 0 ]
//     },
//     {
//       "level": 8,
//       "features": [
//         { "key": "wild_shape", "name": "Wild Shape Improvement" },
//         { "key": "ability_score" }
//       ],
//       "cantrips_known": 3,
//       "spell_slots": [ 4, 3, 3, 2, 0, 0, 0, 0, 0 ]
//     },
//     {
//       "level": 9,
//       "features": [],
//       "cantrips_known": 3,
//       "spell_slots": [ 4, 3, 3, 3, 1, 0, 0, 0, 0 ]
//     },
//     {
//       "level": 10,
//       "features": [
//         { "key": "druid_circle", "name": "Druid Circle Feature" }
//       ],
//       "cantrips_known": 4,
//       "spell_slots": [ 4, 3, 3, 3, 2, 0, 0, 0, 0 ]
//     },
//     {
//       "level": 11,
//       "features": [],
//       "cantrips_known": 4,
//       "spell_slots": [ 4, 3, 3, 3, 2, 1, 0, 0, 0 ]
//     },
//     {
//       "level": 12,
//       "features": [
//         { "key": "ability_score" }
//       ],
//       "cantrips_known": 4,
//       "spell_slots": [ 4, 3, 3, 3, 2, 1, 0, 0, 0 ]
//     },
//     {
//       "level": 13,
//       "features": [ ],
//       "cantrips_known": 4,
//       "spell_slots": [ 4, 3, 3, 3, 2, 1, 1, 0, 0 ]
//     },
//     {
//       "level": 14,
//       "features": [
//         { "key": "druid_circle", "name": "Druid Circle Feature" }
//       ],
//       "cantrips_known": 4,
//       "spell_slots": [ 4, 3, 3, 3, 2, 1, 1, 0, 0 ]
//     },
//     {
//       "level": 15,
//       "features": [],
//       "cantrips_known": 4,
//       "spell_slots": [ 4, 3, 3, 3, 2, 1, 1, 1, 0 ]
//     },
//     {
//       "level": 16,
//       "features": [
//         { "key": "ability_score" }
//       ],
//       "cantrips_known": 4,
//       "spell_slots": [ 4, 3, 3, 3, 2, 1, 1, 1, 0 ]
//     },
//     {
//       "level": 17,
//       "features": [],
//       "cantrips_known": 4,
//       "spell_slots": [ 4, 3, 3, 3, 2, 1, 1, 1, 1 ]
//     },
//     {
//       "level": 18,
//       "features": [
//         { "key": "druid_timeless_body" },
//         { "key": "beast_spells" }
//       ],
//       "cantrips_known": 4,
//       "spell_slots": [ 4, 3, 3, 3, 3, 1, 1, 1, 1 ]
//     },
//     {
//       "level": 19,
//       "features": [
//         { "key": "ability_score" }
//       ],
//       "cantrips_known": 4,
//       "spell_slots": [ 4, 3, 3, 3, 3, 2, 1, 1, 1 ]
//     },
//     {
//       "level": 20,
//       "features": [
//         { "key": "archdruid" }
//       ],
//       "cantrips_known": 4,
//       "spell_slots": [ 4, 3, 3, 3, 3, 2, 2, 1, 1 ]
//     }
//   ],
//   "features": {
//     "ability_score": {
//       "name": "Ability Score Improvement",
//       "desc": "When you reach 4th level, and again at 8th, 12th, 16th, and 19th level, you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1. As normal, you can't increase an ability score above 20 using this feature."
//     },
//     "druidic": {
//       "name": "Druidic",
//       "desc": "You know Druidic, the secret language of druids. You can speak the language and use it to leave hidden messages. You and others who know this language automatically spot such a message. Others spot the message's presence with a successful DC 15 Wisdom (Perception) check but can't decipher it without magic."
//     },
//     "wild_shape": {
//       "name": "Wild Shape",
//       "desc": "Starting at 2nd level, you can use your action to magically assume the shape of a beast that you have seen before. You can use this feature twice. You regain expended uses when you finish a short or long rest.\nYour druid level determines the beasts you can transform into, as shown in the Beast Shapes table. At 2nd level, for example, you can transform into any beast that has a challenge rating of 1/4 or lower that doesn't have a flying or swimming speed.\n**Beast Shapes (table)**\n| Level | Max. CR | Limitations | Example |\n|_|_|_|_|\n| 2nd | 1/4 | No flying or swimming speed | Wolf |\n| 4th | 1/2 | No flying speed | Crocodile |\n| 8th | 1 | | Giant eagle |\n\nYou can stay in a beast shape for a number of hours equal to half your druid level (rounded down). You then revert to your normal form unless you expend another use of this feature. You can revert to your normal form earlier by using a bonus action on your turn. You automatically revert if you fall unconscious, drop to 0 hit points, or die.\nWhile you are transformed, the following rules apply:\nYour game statistics are replaced by the statistics of the beast, but you retain your alignment, personality, and Intelligence, Wisdom, and Charisma scores. You also retain all of your skill and saving throw proficiencies, in addition to gaining those of the creature. If the creature has the same proficiency as you and the bonus in its stat block is higher than yours, use the creature's bonus instead of yours. If the creature has any legendary or lair actions, you can't use them.\nWhen you transform, you assume the beast's hit points and Hit Dice. When you revert to your normal form, you return to the number of hit points you had before you transformed. However, if you revert as a result of dropping to 0 hit points, any excess damage carries over to your normal form. For example, if you take 10 damage in animal form and have only 1 hit point left, you revert and take 9 damage. As long as the excess damage doesn't reduce your normal form to 0 hit points, you aren't knocked unconscious.\nYou can't cast spells, and your ability to speak or take any action that requires hands is limited to the capabilities of your beast form. Transforming doesn't break your concentration on a spell you've already cast, however, or prevent you from taking actions that are part of a spell, such as *call lightning*, that you've already cast.\nYou retain the benefit of any features from your class, race, or other location and can use them if the new form is physically capable of doing so. However, you can't use any of your special senses, such as darkvision, unless your new form also has that sense.\nYou choose whether your equipment falls to the ground in your space, merges into your new form, or is worn by it. Worn equipment functions as normal, but the GM decides whether it is practical for the new form to wear a piece of equipment, based on the creature's shape and size. Your equipment doesn't change size or shape to match the new form, and any equipment that the new form can't wear must either fall to the ground or merge with it. Equipment that merges with the form has no effect until you leave the form."
//     },
//     "druid_cirlce": {
//       "name": "Druid Circle",
//       "desc": "At 2nd level, you choose to identify with a circle of druids: the Circle of the Land or the Circle of the Moon, both detailed at the end of the class description. Your choice grants you features at 2nd level and again at 6th, 10th, and 14th level."
//     },
//     "druid_timeless_body": {
//       "name": "Timeless Body",
//       "desc": "Starting at 18th level, the primal magic that you wield causes you to age more slowly. For every 10 years that pass, your body ages only 1 year."
//     },
//     "beast_spells": {
//       "name": "Beast Spells",
//       "desc": "Beginning at 18th level, you can cast many of your druid spells in any shape you assume using Wild Shape. You can perform the somatic and verbal components of a druid spell while in a beast shape, but you aren't able to provide material components."
//     },
//     "archdruid": {
//       "name": "Archdruid",
//       "desc": "At 20th level, you can use your Wild Shape an unlimited number of times.\nAdditionally, you can ignore the verbal and somatic components of your druid spells, as well as any material components that lack a cost and aren't consumed by a spell. You gain this benefit in both your normal shape and your beast shape from Wild Shape."
//     }
//   }
// }
// "#;
