use bevy::prelude::*;

use super::widget::{WidgetAppExt, WidgetDrawContext};

pub struct TextPlugin;
impl Plugin for TextPlugin {
    fn build(&self, app: &mut App) {
        app.register_widget::<Label, _>(text_draw_system);
    }
}

#[derive(Component, Default, Clone, PartialEq)]
pub struct Label {
    pub text: ratatui::text::Text<'static>,
}

impl Label {
    pub fn new(text: impl Into<ratatui::text::Text<'static>>) -> Self {
        Self { text: text.into() }
    }
}

fn text_draw_system(In(mut ctx): In<WidgetDrawContext>, data_query: Query<&Label>) {
    let Ok(data) = data_query.get(ctx.entity()) else {
        return;
    };

    ctx.draw_sized((data.text.width() as u16, data.text.lines.len() as u16), |frame, rect| {
        frame.render_widget(&data.text, rect);
    });
}
