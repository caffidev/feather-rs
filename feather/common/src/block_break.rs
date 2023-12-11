use base::{ValidBlockPosition, ItemStack};
use ecs::{SystemExecutor, SysResult, EntityBuilder};
use quill_common::entity_init::EntityInit;

use crate::{Game,World};

pub type BlockBreaker = Option<ActiveBlockBreaker>;
pub struct ActiveBlockBreaker {
    pub position: ValidBlockPosition,
    pub drop_item: bool,
    pub ticks_remaining: u32,
}

impl ActiveBlockBreaker {
    pub fn tick(&mut self) -> bool {
        self.ticks_remaining = self.ticks_remaining.saturating_sub(1);
        self.ticks_remaining == 0
    }

    pub fn break_block(self, game: &mut Game) -> SysResult {
        let target_block = match game.block(self.position) {
            Some(b) => b,
            // Returns Error
            None => anyhow::bail!("Cannot break unloaded block!") 
        };
        game.break_block(self.position);
        if let Some(_item_drop) = base::Item::from_name(target_block.kind().name()) {
            if !self.drop_item {
                return Ok(())
            }
            let mut item_entity = EntityBuilder::new();
            crate::entities::item::build_default(&mut item_entity);
            let builder = game.create_entity_builder(self.position.position(), EntityInit::Item);
            game.spawn_entity(builder);
        }
        Ok(())
    }

    pub fn new_player(_world: &mut World,
        block_pos: ValidBlockPosition,
        _mainhand: Option<&ItemStack>,
        _offhand: Option<&ItemStack>) -> Option<Self> {
        // TODO: implement block breaking
        Some(Self {
            position: block_pos,
            drop_item: true,
            ticks_remaining: 100,
        })
    }
}

pub fn register(systems: &mut SystemExecutor<Game>) {
    systems.add_system(process_block_breaking);
}

fn process_block_breaking(game: &mut Game) -> SysResult {
    let mut break_queue = vec![];
    for (entity, breaker) in game.ecs.query::<&mut BlockBreaker>().iter() {
        if let Some(active) = breaker {
            if active.tick() {
                break_queue.push(entity);
            }
        }
    }
    for entity in break_queue.into_iter() {
        // Set Block Breakers to None
        let breaker = {
            game.ecs.get_mut::<BlockBreaker>(entity)?.take()
                .unwrap()
        };
        breaker.break_block(game)?;
    }
    Ok(())
}