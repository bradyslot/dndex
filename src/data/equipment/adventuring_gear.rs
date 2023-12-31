#![allow(unused, non_upper_case_globals)]
use crate::models::equipment::*;

pub const adventuring_gear: [SRDAdventuringGearItem; 19] = [
  SRDAdventuringGearItem {
    key: "acid",
    name: "Acid",
    desc: "As an action, you can splash the contents of this vial onto a creature within 5 feet of you or throw the vial up to 20 feet, shattering it on impact. In either case, make a ranged attack against a creature or object, treating the acid as an improvised weapon. On a hit, the target takes 2d6 acid damage."
  },
  SRDAdventuringGearItem {
    key: "alchemists_fire",
    name: "Alchemist's Fire",
    desc: "This sticky, adhesive fluid ignites when exposed to air. As an action, you can throw this flask up to 20 feet, shattering it on impact. Make a ranged attack against a creature or object, treating the alchemist's fire as an improvised weapon. On a hit, the target takes 1d4 fire damage at the start of each of its turns. A creature can end this damage by using its action to make a DC 10 Dexterity check to extinguish the flames."
  },
  SRDAdventuringGearItem {
    key: "antitoxin",
    name: "Antitoxin",
    desc: "A creature that drinks this vial of liquid gains advantage on saving throws against poison for 1 hour. It confers no benefit to undead or constructs."
  },
  SRDAdventuringGearItem {
    key: "archane_focus",
    name: "Arcane Focus",
    desc: "An arcane focus is a special item-an orb, a crystal, a rod, a specially constructed staff, a wand-like length of wood, or some similar item- designed to channel the power of arcane spells. A sorcerer, warlock, or wizard can use such an item as a spellcasting focus."
  },
  SRDAdventuringGearItem {
    key: "ball_bearings",
    name: "Ball Bearings",
    desc: "As an action, you can spill these tiny metal balls from their pouch to cover a level, square area that is 10 feet on a side. A creature moving across the covered area must succeed on a DC 10 Dexterity saving throw or fall prone. A creature moving through the area at half speed doesn't need to make the save."
  },
  SRDAdventuringGearItem {
    key: "block_and_tackle",
    name: "Block and Tackle",
    desc: "A set of pulleys with a cable threaded through them and a hook to attach to objects, a block and tackle allows you to hoist up to four times the weight you can normally lift."
  },
  SRDAdventuringGearItem {
    key: "book",
    name: "Book",
    desc: "A book might contain poetry, historical accounts, information pertaining to a particular field of lore, diagrams and notes on gnomish contraptions, or just about anything else that can be represented using text or pictures. A book of spells is a spellbook (described later in this section)."
  },
  SRDAdventuringGearItem {
    key: "caltrops",
    name: "Caltrops",
    desc: "As an action, you can spread a bag of caltrops to cover a square area that is 5 feet on a side. Any creature that enters the area must succeed on a DC 15 Dexterity saving throw or stop moving this turn and take 1 piercing damage. Taking this damage reduces the creature's walking speed by 10 feet until the creature regains at least 1 hit point. A creature moving through the area at half speed doesn't need to make the save."
  },
  SRDAdventuringGearItem {
    key: "candle",
    name: "Candle",
    desc: "For 1 hour, a candle sheds bright light in a 5-foot radius and dim light for an additional 5 feet."
  },
  SRDAdventuringGearItem {
    key: "case_crossbow_bolt",
    name: "Case, Crossbow Bolt",
    desc: "This wooden case can hold up to twenty crossbow bolts."
  },
  SRDAdventuringGearItem {
    key: "case_map_or_scroll",
    name: "Case, Map or Scroll",
    desc: "This cylindrical leather case can hold up to ten rolled-up sheets of paper or five rolled-up sheets of parchment."
  },
  SRDAdventuringGearItem {
    key: "chain",
    name: "Chain",
    desc: "A chain has 10 hit points. It can be burst with a successful DC 20 Strength check."
  },
  SRDAdventuringGearItem {
    key: "climbers_kit",
    name: "Climber's Kit",
    desc: "A climber's kit includes special pitons, boot tips, gloves, and a harness. You can use the climber's kit as an action to anchor yourself; when you do, you can't fall more than 25 feet from the point where you anchored yourself, and you can't climb more than 25 feet away from that point without undoing the anchor."
  },
  SRDAdventuringGearItem {
    key: "component_pouch",
    name: "Component Pouch",
    desc: "A component pouch is a small, watertight leather belt pouch that has compartments to hold all the material components and other special items you need to cast your spells, except for those components that have a specific cost (as indicated in a spell's description)."
  },
  SRDAdventuringGearItem {
    key: "crowbar",
    name: "Crowbar",
    desc: "Using a crowbar grants advantage to Strength checks where the crowbar's leverage can be applied."
  },
  SRDAdventuringGearItem {
    key: "druidic_focus",
    name: "Druidic Focus",
    desc: "A druidic focus might be a sprig of mistletoe or holly, a wand or scepter made of yew or another special wood, a staff drawn whole out of a living tree, or a totem object incorporating feathers, fur, bones, and teeth from sacred animals. A druid can use such an object as a spellcasting focus."
  },
  SRDAdventuringGearItem {
    key: "fishing_tackle",
    name: "Fishing Tackle",
    desc: "This kit includes a wooden rod, silken line, corkwood bobbers, steel hooks, lead sinkers, velvet lures, and narrow netting."
  },
  SRDAdventuringGearItem {
    key: "healers_kit",
    name: "Healer's Kit",
    desc: "This kit is a leather pouch containing bandages, salves, and splints. The kit has ten uses. As an action, you can expend one use of the kit to stabilize a creature that has 0 hit points, without needing to make a Wisdom (Medicine) check."
  },
  SRDAdventuringGearItem {
    key: "holy_symbol",
    name: "Holy Symbol",
    desc: "A holy symbol is a representation of a god or pantheon. It might be an amulet depicting a symbol representing a deity, the same symbol carefully engraved or inlaid as an emblem on a shield, or a tiny box holding a fragment of a sacred relic."
  }
];
