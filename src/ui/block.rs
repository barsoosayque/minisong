use std::sync::LazyLock;

use bevy::prelude::*;
use ratatui::widgets::Borders;

use super::{
    widget::{WidgetDrawContext, WidgetTag},
    RatatuiAppExt,
};

static TAG: LazyLock<WidgetTag> = LazyLock::new(|| WidgetTag::new::<Data>());

pub struct BlockPlugin;
impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        app.register_draw_system(*TAG, block_draw_system);
    }
}

#[derive(Component, Default, Clone, PartialEq)]
struct Data {
    borders: Borders,
}

#[derive(Bundle)]
pub struct Block {
    data: Data,
    tag: WidgetTag,
}

impl Default for Block {
    fn default() -> Self {
        Self::new()
    }
}

impl Block {
    pub fn new() -> Self {
        Self { data: Data::default(), tag: *TAG }
    }

    pub fn with_borders(self, borders: Borders) -> Self {
        Self { data: Data { borders, ..self.data }, ..self }
    }
}

fn block_draw_system(In(mut ctx): In<WidgetDrawContext>, data_query: Query<&Data>) {
    let Ok(data) = data_query.get(ctx.entity()) else {
        return;
    };

    ctx.draw(|frame, rect| {
        let widget = ratatui::widgets::Block::default().borders(data.borders);
        frame.render_widget(widget, rect);
    });
}
