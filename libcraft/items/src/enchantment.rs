//! Data sourced from: <https://minecraft.wiki/w/Enchanting#Enchantments>

use serde::{Deserialize, Serialize};

/// An enchantment attached to an item.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Enchantment {
    /// The kind of the enchantment, contains an id.
    #[serde(rename = "id")]
    kind: EnchantmentKind,
    /// Enchantment level, represented by an `i8` for vanilla compatibility
    #[serde(rename = "lvl")]
    level: i8,
}

impl Enchantment {
    /// Creates an enchantment given the type of
    /// enchantment and the level.
    ///
    /// Will allow any level of enchantment, i.e,
    /// level is not capped by the maximum level
    /// of the enchantment that can be acquired in the game.
    ///
    /// The level is capped at `i8::MAX` for compatability
    /// with Vanilla.
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn new(kind: EnchantmentKind, level: u32) -> Self {
        Self {
            kind,
            level: level.min(i8::MAX as u32) as i8,
        }
    }

    /// Gets the kind of this enchantment.
    #[must_use]
    pub const fn kind(&self) -> EnchantmentKind {
        self.kind
    }

    /// Gets the level of this enchantment.
    #[must_use]
    #[allow(clippy::cast_sign_loss)]
    pub fn level(&self) -> u32 {
        self.level.max(0) as u32
    }

    /// Sets the kind of this enchantment.
    ///
    /// The level is not affected.
    pub fn set_kind(&mut self, kind: EnchantmentKind) {
        self.kind = kind;
    }

    /// Sets the level of this enchantment.
    ///
    /// The level is capped to `i8::MAX`.
    #[allow(clippy::cast_possible_truncation)]
    pub fn set_level(&mut self, level: u32) {
        self.level = level.min(i8::MAX as u32) as i8;
    }
}

/// Kind of an enchantment.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EnchantmentKind {
    /// Increases underwater mining speed.
    AquaAffinity = 6,
    /// Increases damage and applies Slowness IV to arthropod
    /// mobs (spiders, cave spiders, silverfish, endermites and bees).
    BaneOfArthropods = 14,
    /// Reduces explosion damage and knockback.
    BlastProtection = 3,
    /// During thunderstorms, trident summons a lightning bolt
    /// on the target when hitting it.
    Channeling = 32,
    /// Increases damage and shield stunning.
    // Cleaving, bedrock edition - planned to be added
    /// Items cannot be removed from armor slots.
    BindingCurse = 10,
    /// Item disappears on death.
    VanishingCurse = 37,
    /// Increased underwater movement speed.
    DepthStrider = 8,
    /// Increases tool speed, as well as the chance
    /// for axes to disable shields.
    Efficiency = 19,
    /// Reduces fall damage.
    FeatherFalling = 2,
    /// Sets target on fire.
    FireAspect = 16,
    /// Reduces fire damage and burn time.
    /// Mutually exclusive with other protections.
    FireProtection = 1,
    /// Arrows shot are ignited and deal fire damage to the target.
    Flame = 25,
    /// Increases the amount of block drops.
    Fortune = 22,
    /// Allows the player to walk on water by
    /// freezing the water under their feet.
    FrostWalker = 9,
    /// Increases damage against aquatic mobs.
    Impaling = 30,
    /// Prevents consumption of arrows.
    Infinity = 26,
    /// Increases knockback.
    Knockback = 15,
    /// Increases mob loot.
    Looting = 17,
    /// Trident returns after being thrown.
    Loyalty = 29,
    /// Increases rate of good loot (enchanting books, etc.)
    LuckOfTheSea = 27,
    /// Decreases time for bites.
    Lure = 28,
    /// Repairs the item using experience.
    Mending = 36,
    /// Fires 3 arrows at the same time.
    Multishot = 33,
    /// Arrows pierce entities, allowing for arrows to pierce
    /// through stacks of mobs.
    Piercing = 35,
    /// Increases arrow damage.
    Power = 23,
    /// Reduces damage from projectiles.
    ProjectileProtection = 4,
    /// Reduces generic damage.
    Protection = 0,
    /// Increases arrow knockback.
    Punch = 24,
    /// Decreases crossbow charging time.
    QuickCharge = 34,
    /// Extends underwater breathing time.
    Respiration = 5,
    /// Trident launches player with itself when thrown
    /// while in water or rain.
    Riptide = 31,
    /// Increases melee damage.
    Sharpness = 12,
    /// Mined blocks drop themselves.
    SilkTouch = 20,
    /// Increases damage to the undead.
    Smite = 13,
    /// Increases movement speed on soul sand and soul soil.
    SoulSpeed = 11,
    /// Increases sweeping attack damage.
    Sweeping = 18,
    /// Taking damage causes the attacker to also take damage.
    Thorns = 7,
    /// Increases sneaking speed.
    // SwiftSneak, 1.19 v.
    /// Reduces durability damage.
    Unbreaking = 21,
}
