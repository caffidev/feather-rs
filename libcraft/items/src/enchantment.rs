//! Data sourced from: <https://minecraft.wiki/w/Enchanting#Enchantments>

use serde::{Deserialize, Serialize};

/// An enchantment attached to an item.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Enchantment {
    /// The type of the enchantment.
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
    AquaAffinity,
    /// Increases damage and applies Slowness IV to arthropod
    /// mobs (spiders, cave spiders, silverfish, endermites and bees).
    BaneOfArthropods,
    /// Reduces explosion damage and knockback.
    BlastProtection,
    /// During thunderstorms, trident summons a lightning bolt
    /// on the target when hitting it.
    Channeling,
    /// Increases damage and shield stunning.
    Cleaving,
    /// Items cannot be removed from armor slots.
    CurseOfBinding,
    /// Item disappears on death.
    CurseOfVanishing,
    /// Increased underwater movement speed.
    DepthStrider,
    /// Increases tool speed, as well as the chance
    /// for axes to disable shields.
    Efficiency,
    /// Reduces fall damage.
    FeatherFalling,
    /// Sets target on fire.
    FireAspect,
    /// Reduces fire damage and burn time.
    /// Mutually exclusive with other protections.
    FireProtection,
    /// Arrows shot are ignited and deal fire damage to the target.
    Flame,
    /// Increases the amount of block drops.
    Fortune,
    /// Allows the player to walk on water by
    /// freezing the water under their feet.
    FrostWalker,
    /// Increases damage against aquatic mobs.
    Impaling,
    /// Prevents consumption of arrows.
    Infinity,
    /// Increases knockback.
    Knockback,
    /// Increases mob loot.
    Looting,
    /// Trident returns after being thrown.
    Loyalty,
    /// Increases rate of good loot (enchanting books, etc.)
    LuckOfTheSea,
    /// Decreases time for bites.
    Lure,
    /// Repairs the item using experience.
    Mending,
    /// Fires 3 arrows at the same time.
    Multishot,
    /// Arrows pierce entities, allowing for arrows to pierce
    /// through stacks of mobs.
    Piercing,
    /// Increases arrow damage.
    Power,
    /// Reduces damage from projectiles.
    ProjectileProtection,
    /// Reduces generic damage.
    Protection,
    /// Increases arrow knockback.
    Punch,
    /// Decreases crossbow charging time.
    QuickCharge,
    /// Extends underwater breathing time.
    Respiration,
    /// Trident launches player with itself when thrown
    /// while in water or rain.
    Riptide,
    /// Increases melee damage.
    Sharpness,
    /// Mined blocks drop themselves.
    SilkTouch,
    /// Increases damage to the undead.
    Smite,
    /// Increases movement speed on soul sand and soul soil.
    SoulSpeed,
    /// Increases sweeping attack damage.
    SweepingEdge,
    /// Taking damage causes the attacker to also take damage.
    Thorns,
    /// Increases sneaking speed.
    // SwiftSneak, 1.19 v.
    /// Reduces durability damage.
    Unbreaking,
}
