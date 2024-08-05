use std::sync::LazyLock;

use bevy::prelude::*;

use super::{
    widget::{Size, WidgetDrawContext, WidgetTag},
    RatatuiAppExt,
};

static TAG: LazyLock<WidgetTag> = LazyLock::new(|| WidgetTag::new::<Data>());

pub struct TextPlugin;
impl Plugin for TextPlugin {
    fn build(&self, app: &mut App) {
        app.register_draw_system(*TAG, text_draw_system);
    }
}

#[derive(Component, Default, Clone, PartialEq)]
struct Data {
    text: ratatui::text::Text<'static>,
}

#[derive(Bundle)]
pub struct Label {
    data: Data,
    tag: WidgetTag,
}

impl Label {
    pub fn new(text: impl Into<ratatui::text::Text<'static>>) -> Self {
        Self { data: Data { text: text.into() }, tag: *TAG }
    }
}

fn text_draw_system(In(mut ctx): In<WidgetDrawContext>, data_query: Query<&Data>) {
    let Ok(data) = data_query.get(ctx.entity()) else {
        return;
    };

    ctx.draw_sized(Size::new(data.text.width(), data.text.lines.len()), |frame, rect| {
        frame.render_widget(&data.text, rect);
    });
}
