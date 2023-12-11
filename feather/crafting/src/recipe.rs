use crate::{model, solver, Grid, TABLE_SIZE};
use anyhow::anyhow;
use arrayvec::ArrayVec;
use libcraft_items::{Item, ItemStack};
use std::collections::BTreeMap;

#[derive(Clone, Debug)]
pub struct ShapedRecipe {
    pub input: Grid,
    pub output: ItemStack,
}

#[derive(Clone, Debug)]
pub struct ShapelessRecipe {
    pub input: ArrayVec<[Item; TABLE_SIZE]>,
    pub output: ItemStack,
}

#[derive(Clone, Debug)]
pub enum Recipe {
    Shaped(ShapedRecipe),
    Shapeless(ShapelessRecipe),
}

pub fn convert(model: model::Recipe) -> anyhow::Result<Recipe> {
    match model {
        model::Recipe::Shaped {
            pattern,
            key,
            output,
        } => convert_shaped(pattern, key, output),
        model::Recipe::Shapeless {
            ingredients,
            output,
        } => convert_shapeless(&ingredients, output),
    }
}

fn convert_shaped(
    pattern: model::Table,
    key: BTreeMap<char, model::Key>,
    output: model::Output,
) -> anyhow::Result<Recipe> {
    let mut input = Grid::default();

    for (y, row) in pattern.iter().enumerate() {
        for (x, slot) in row.as_str().chars().enumerate() {
            let key = key
                .get(&slot)
                .ok_or_else(|| anyhow!("No entry in key for character '{}'", slot))?;

            let item = convert_key(key)?;

            if let Some(item) = item {
                input[x][y] = Some(item);
            }
        }
    }

    solver::normalize(&mut input);

    let output = convert_output(&output)?;

    Ok(Recipe::Shaped(ShapedRecipe { input, output }))
}

fn convert_shapeless(ingredients: &[model::Key], output: model::Output) -> anyhow::Result<Recipe> {
    let mut input = ArrayVec::new();

    for ingredient in ingredients {
        let item = convert_key(ingredient)?;

        if let Some(item) = item {
            input.push(item);
        }
    }

    input.sort_unstable();

    let output = convert_output(&output)?;

    Ok(Recipe::Shapeless(ShapelessRecipe { input, output }))
}

fn convert_key(key: &model::Key) -> anyhow::Result<Option<Item>> {
    match key {
        model::Key::Item(identifier) => Ok(Some(
            Item::from_identifier(*identifier)
                .ok_or_else(|| anyhow!("Invalid item identifier: '{}'", identifier))?,
        )),
        model::Key::Tag(_) => Ok(None),
        // TODO: implement this
    }
}

fn convert_output(output: &model::Output) -> anyhow::Result<ItemStack> {
    let ty = Item::from_identifier(output.item)
        .ok_or_else(|| anyhow!("Invalid item '{}'", output.item))?;
    Ok(ItemStack::new(ty, output.count).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_output() {
        assert_eq!(
            convert_output(&model::Output {
                item: "minecraft:stone",
                count: 5
            })
            .unwrap(),
            ItemStack::new(Item::Stone, 5).unwrap()
        );
    }

    #[test]
    fn test_convert_output_invalid_item() {
        assert!(convert_output(&model::Output {
            item: "minecraft:doesnotexist",
            count: 1
        })
        .is_err());
    }

    #[test]
    fn test_convert_key() {
        assert_eq!(
            convert_key(&model::Key::Item("minecraft:diamond_sword")).unwrap(),
            Some(Item::DiamondSword)
        );
        assert_eq!(
            convert_key(&model::Key::Tag("unimplemented")).unwrap(),
            None
        );
    }
}
