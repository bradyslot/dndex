#![allow(unused)]
// pub const MONK_DATA: &str = r#"
// {
//   "hit_points": {
//     "hit_dice": 8,
//     "static_option": 5,
//     "desc": "**Hit Dice:** 1d8 per Monk level\n**Hit Points at 1st Level:** 8 + your Constitution modifier\n**Hit Points at Higher Levels:** 1d8 (or 5) + your Constitution modifier per monk level after 1st"
//   },
//   "proficiencies": {
//     "armor": [],
//     "weapons": [
//       { "category": "Simple" },
//       { "key": "shortsword" }
//     ],
//     "tools": [
//       [
//         { "key": "artisans_tools" }
//       ],
//       [
//         { "key": "musical_instrument" }
//       ]
//     ],
//     "saving_throws": [ "strength", "dexterity" ],
//     "skills": { "choices": 2, "options": [ "acrobatics", "athletics", "history", "insight", "religion", "stealth" ] },
//     "desc": "**Armor:** None\n**Weapons:** Simple weapons, shortswords\n**Tools:** Choose one type of artisan's tools or one musical instrument\n**Saving Throws:** Strength, Dexterity\n**Skills:** Choose two from Acrobatics, Athletics, History, Insight, Religion, and Stealth"
//   },
//   "equipment": {
//     "choice_1": [
//       [
//         { "source": "open5e", "location": "weapons", "key": "shortsword" }
//       ],
//       [
//         { "source": "open5e", "location": "weapons", "category": "Simple" }
//       ]
//     ],
//     "choice_2": [
//       [
//         { "source": "data", "location": "equipment_packs", "key": "dungeoneers_pack" }
//       ],
//       [
//         { "source": "data", "location": "equipment_packs", "key": "explorers_pack" }
//       ]
//     ],
//     "choice_3": [],
//     "choice_4": [],
//     "defaults": [
//       { "source": "open5e", "location": "weapons", "key": "dart" }
//     ],
//     "desc": "You start with the following equipment, in addition to the equipment granted by your background:\n- *(a)* a shortsword or *(b)* any simple weapon\n- *(a)* a dungeoneer's pack or *(b)* an explorer’s pack\n- 10 darts"
//   },
//   "spellcasting": {},
//   "levels": [
//     {
//       "level": 1,
//       "features": [
//         { "key": "unarmored_defence" },
//         { "key": "martial_arts" }
//       ],
//       "martial_arts": 4,
//       "ki_points": 0,
//       "unarmored_movement": 0
//     },
//     {
//       "level": 2,
//       "features": [
//         { "key": "ki" },
//         { "key": "unarmored_movement" }
//       ],
//       "martial_arts": 4,
//       "ki_points": 2,
//       "unarmored_movement": 10
//     },
//     {
//       "level": 3,
//       "features": [
//         { "key": "monastic_traditions" },
//         { "key": "deflect_missiles" }
//       ],
//       "martial_arts": 4,
//       "ki_points": 3,
//       "unarmored_movement": 10
//     },
//     {
//       "level": 4,
//       "features": [
//         { "key": "ability_score" },
//         { "key": "slow_fall" }
//       ],
//       "martial_arts": 4,
//       "ki_points": 4,
//       "unarmored_movement": 10
//     },
//     {
//       "level": 5,
//       "features": [
//         { "key": "monk_extra_attack" },
//         { "key": "stunning_strike" }
//       ],
//       "martial_arts": 6,
//       "ki_points": 5,
//       "unarmored_movement": 10
//     },
//     {
//       "level": 6,
//       "features": [
//         { "key": "ki_empowered_strikes" },
//         { "key": "monastic_traditions", "name": "Monastic Tradition Feature" }
//       ],
//       "martial_arts": 6,
//       "ki_points": 6,
//       "unarmored_movement": 15
//     },
//     {
//       "level": 7,
//       "features": [
//         { "key": "monk_evasion" },
//         { "key": "stillness_of_mind" }
//       ],
//       "martial_arts": 6,
//       "ki_points": 7,
//       "unarmored_movement": 15
//     },
//     {
//       "level": 8,
//       "features": [
//         { "key": "ability_score" }
//       ],
//       "martial_arts": 6,
//       "ki_points": 8,
//       "unarmored_movement": 15
//     },
//     {
//       "level": 9,
//       "features": [
//         { "key": "unarmored_movement", "name": "Unarmored Movement Improvement" }
//       ],
//       "martial_arts": 6,
//       "ki_points": 9,
//       "unarmored_movement": 15
//     },
//     {
//       "level": 10,
//       "features": [
//         { "key": "purity_of_body" }
//       ],
//       "martial_arts": 6,
//       "ki_points": 10,
//       "unarmored_movement": 20
//     },
//     {
//       "level": 11,
//       "features": [
//         { "key": "monastic_traditions", "name": "Monastic Tradition Feature" }
//       ],
//       "martial_arts": 8,
//       "ki_points": 11,
//       "unarmored_movement": 20
//     },
//     {
//       "level": 12,
//       "features": [
//         { "key": "ability_score" }
//       ],
//       "martial_arts": 8,
//       "ki_points": 12,
//       "unarmored_movement": 20
//     },
//     {
//       "level": 13,
//       "features": [
//         { "key": "tongue_of_the_sun_and_moon" }
//       ],
//       "martial_arts": 8,
//       "ki_points": 13,
//       "unarmored_movement": 20
//     },
//     {
//       "level": 14,
//       "features": [
//         { "key": "diamond_soul" }
//       ],
//       "martial_arts": 8,
//       "ki_points": 14,
//       "unarmored_movement": 25
//     },
//     {
//       "level": 15,
//       "features": [
//         { "key": "monk_timeless_body" }
//       ],
//       "martial_arts": 8,
//       "ki_points": 15,
//       "unarmored_movement": 25
//     },
//     {
//       "level": 16,
//       "features": [
//         { "key": "ability_score" }
//       ],
//       "martial_arts": 8,
//       "ki_points": 16,
//       "unarmored_movement": 25
//     },
//     {
//       "level": 17,
//       "features": [
//         { "key": "monastic_traditions", "name": "Monastic Tradition Feature" }
//       ],
//       "martial_arts": 10,
//       "ki_points": 17,
//       "unarmored_movement": 25
//     },
//     {
//       "level": 18,
//       "features": [
//         { "key": "empty_body" }
//       ],
//       "martial_arts": 10,
//       "ki_points": 18,
//       "unarmored_movement": 30
//     },
//     {
//       "level": 19,
//       "features": [
//         { "key": "ability_score" }
//       ],
//       "martial_arts": 10,
//       "ki_points": 19,
//       "unarmored_movement": 30
//     },
//     {
//       "level": 20,
//       "features": [
//         { "key": "perfect_self" }
//       ],
//       "martial_arts": 10,
//       "ki_points": 20,
//       "unarmored_movement": 30
//     }
//   ],
//   "features": {
//     "ability_score": {
//       "name": "Ability Score Improvement",
//       "desc": "When you reach 4th level, and again at 8th, 12th, 16th, and 19th level, you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1. As normal, you can't increase an ability score above 20 using this feature."
//     },
//     "unarmored_defense": {
//       "name": "Unarmored Defense",
//       "desc": "Beginning at 1st level, while you are wearing no armor and not wielding a shield, your AC equals 10 + your Dexterity modifier + your Wisdom modifier."
//     },
//     "martial_arts": {
//       "name": "Martial Arts",
//       "desc": "At 1st level, your practice of martial arts gives you mastery of combat styles that use unarmed strikes and monk weapons, which are shortswords and any simple melee weapons that don't have the two_ handed or heavy property.\nYou gain the following benefits while you are unarmed or wielding only monk weapons and you aren't wearing armor or wielding a shield:\nYou can use Dexterity instead of Strength for the attack and damage rolls of your unarmed strikes and monk weapons.\nYou can roll a d4 in place of the normal damage of your unarmed strike or monk weapon. This die changes as you gain monk levels, as shown in the Martial Arts column of the Monk table.\nWhen you use the Attack action with an unarmed strike or a monk weapon on your turn, you can make one unarmed strike as a bonus action. For example, if you take the Attack action and attack with a quarterstaff, you can also make an unarmed strike as a bonus action, assuming you haven't already taken a bonus action this turn.\nCertain monasteries use specialized forms of the monk weapons. For example, you might use a club that is two lengths of wood connected by a short chain (called a nunchaku) or a sickle with a shorter, straighter blade (called a kama). Whatever name you use for a monk weapon, you can use the game statistics provided for the weapon."
//     },
//     "ki": {
//       "name": "Ki",
//       "desc": "Starting at 2nd level, your training allows you to harness the mystic energy of ki. Your access to this energy is represented by a number of ki points. Your monk level determines the number of points you have, as shown in the Ki Points column of the Monk table.\nYou can spend these points to fuel various ki features. You start knowing three such features: Flurry of Blows, Patient Defense, and Step of the Wind. You learn more ki features as you gain levels in this class.\nWhen you spend a ki point, it is unavailable until you finish a short or long rest, at the end of which you draw all of your expended ki back into yourself. You must spend at least 30 minutes of the rest meditating to regain your ki points.\nSome of your ki features require your target to make a saving throw to resist the feature's effects. The saving throw DC is calculated as follows:\n**Ki save DC** = 8 + your proficiency bonus + your Wisdom modifier\n## Flurry of Blows\nImmediately after you take the Attack action on your turn, you can spend 1 ki point to make two unarmed strikes as a bonus action.\n## Patient Defense\nYou can spend 1 ki point to take the Dodge action as a bonus action on your turn.\n## Step of the Wind\nYou can spend 1 ki point to take the Disengage or Dash action as a bonus action on your turn, and your jump distance is doubled for the turn."
//     },
//     "unarmored_movement": {
//       "name": "Unarmored Movement",
//       "desc": "Starting at 2nd level, your speed increases by 10 feet while you are not wearing armor or wielding a shield. This bonus increases when you reach certain monk levels, as shown in the Monk table.\nAt 9th level, you gain the ability to move along vertical surfaces and across liquids on your turn without falling during the move."
//     },
//     "monastic_tradition": {
//       "name": "Monastic Tradition",
//       "desc": "When you reach 3rd level, you commit yourself to a monastic tradition: the Way of the Open Hand, the Way of Shadow, or the Way of the Four Elements, all detailed at the end of the class description. Your tradition grants you features at 3rd level and again at 6th, 11th, and 17th level. Three traditions of monastic pursuit are common in the monasteries scattered across the multiverse. Most monasteries practice one tradition exclusively, but a few honor the three traditions and instruct each monk according to his or her aptitude and interest. All three traditions rely on the same basic techniques, diverging as the student grows more adept. Thus, a monk need choose a tradition only upon reaching 3rd level."
//     },
//     "deflect_missiles": {
//       "name": "Deflect Missiles",
//       "desc": "Starting at 3rd level, you can use your reaction to deflect or catch the missile when you are hit by a ranged weapon attack. When you do so, the damage you take from the attack is reduced by 1d10 + your Dexterity modifier + your monk level.\nIf you reduce the damage to 0, you can catch the missile if it is small enough for you to hold in one hand and you have at least one hand free. If you catch a missile in this way, you can spend 1 ki point to make a ranged attack with the weapon or piece of ammunition you just caught, as part of the same reaction. You make this attack with proficiency, regardless of your weapon proficiencies, and the missile counts as a monk weapon for the attack, which has a normal range of 20 feet and a long range of 60 feet."
//     },
//     "slow_fall": {
//       "name": "Slow Fall",
//       "desc": "Beginning at 4th level, you can use your reaction when you fall to reduce any falling damage you take by an amount equal to five times your monk level."
//     },
//     "monk_extra_attack": {
//       "name": "Extra Attack",
//       "desc": "Beginning at 5th level, you can attack twice, instead of once, whenever you take the Attack action on your turn."
//     },
//     "stunning_strike": {
//       "name": "Stunning Strike",
//       "desc": "Starting at 5th level, you can interfere with the flow of ki in an opponent's body. When you hit another creature with a melee weapon attack, you can spend 1 ki point to attempt a stunning strike. The target must succeed on a Constitution saving throw or be stunned until the end of your next turn."
//     },
//     "ki_empowered_strikes": {
//       "name": "Ki Empowered Strikes",
//       "desc": "Starting at 6th level, your unarmed strikes count as magical for the purpose of overcoming resistance and immunity to nonmagical attacks and damage."
//     },
//     "monk_evasion": {
//       "name": "Evasion",
//       "desc": "At 7th level, your instinctive agility lets you dodge out of the way of certain area effects, such as a blue dragon's lightning breath or a *fireball* spell. When you are subjected to an effect that allows you to make a Dexterity saving throw to take only half damage, you instead take no damage if you succeed on the saving throw, and only half damage if you fail."
//     },
//     "stillness_of_mind": {
//       "name": "Stillness of Mind",
//       "desc": "Starting at 7th level, you can use your action to end one effect on yourself that is causing you to be charmed or frightened."
//     },
//     "purity_of_body": {
//       "name": "Purity of Body",
//       "desc": "At 10th level, your mastery of the ki flowing through you makes you immune to disease and poison."
//     },
//     "tongue_of_the_sun_and_moon": {
//       "name": "Tongue of the Sun and Moon",
//       "desc": "Starting at 13th level, you learn to touch the ki of other minds so that you understand all spoken languages. Moreover, any creature that can understand a language can understand what you say."
//     },
//     "diamond_soul": {
//       "name": "Diamond Soul",
//       "desc": "Beginning at 14th level, your mastery of ki grants you proficiency in all saving throws.\nAdditionally, whenever you make a saving throw and fail, you can spend 1 ki point to reroll it and take the second result."
//     },
//     "monk_timeless_body": {
//       "name": "Timeless Body",
//       "desc": "At 15th level, your ki sustains you so that you suffer none of the frailty of old age, and you can't be aged magically. You can still die of old age, however. In addition, you no longer need food or water."
//     },
//     "empty_body": {
//       "name": "Empty Body",
//       "desc": "Beginning at 18th level, you can use your action to spend 4 ki points to become invisible for 1 minute. During that time, you also have resistance to all damage but force damage.\nAdditionally, you can spend 8 ki points to cast the *astral projection* spell, without needing material components. When you do so, you can't take any other creatures with you."
//     },
//     "perfect_self": {
//       "name": "Perfect Self",
//       "desc": "At 20th level, when you roll for initiative and have no ki points remaining, you regain 4 ki points."
//     }
//   }
// }
// "#;
