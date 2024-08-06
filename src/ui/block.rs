use bevy::prelude::*;
use ratatui::widgets::Borders;

use super::widget::{WidgetAppExt, WidgetDrawContext};

pub struct BlockPlugin;
impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        app.register_widget::<Block, _>(block_draw_system);
    }
}

#[derive(Component, Default, Clone, PartialEq)]
pub struct Block {
    pub borders: Borders,
}

fn block_draw_system(In(mut ctx): In<WidgetDrawContext>, data_query: Query<&Block>) {
    let Ok(data) = data_query.get(ctx.entity()) else {
        return;
    };

    ctx.draw(|frame, rect| {
        let widget = ratatui::widgets::Block::default().borders(data.borders);
        frame.render_widget(widget, rect);
    });
}
