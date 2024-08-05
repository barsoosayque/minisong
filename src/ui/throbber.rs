use std::sync::LazyLock;

use bevy::prelude::*;

use super::{
    widget::{Size, WidgetDrawContext, WidgetTag},
    RatatuiAppExt,
};

static TAG: LazyLock<WidgetTag> = LazyLock::new(|| WidgetTag::new::<Data>());

pub struct ThrobberPlugin;
impl Plugin for ThrobberPlugin {
    fn build(&self, app: &mut App) {
        app.register_draw_system(*TAG, throbber_draw_system);
    }
}

#[derive(Component, Default, Clone)]
struct Data {
    label: String,
    state: throbber_widgets_tui::ThrobberState,
}

#[derive(Bundle)]
pub struct Throbber {
    data: Data,
    tag: WidgetTag,
}

impl Default for Throbber {
    fn default() -> Self {
        Self::new()
    }
}

impl Throbber {
    pub fn new() -> Self {
        Self { data: Data::default(), tag: *TAG }
    }

    pub fn with_label(self, label: impl ToString) -> Self {
        Self { data: Data { label: label.to_string(), ..self.data }, ..self }
    }
}

fn throbber_draw_system(In(mut ctx): In<WidgetDrawContext>, mut data_query: Query<&mut Data>) {
    let Ok(mut data) = data_query.get_mut(ctx.entity()) else {
        return;
    };
    let Data { label, state } = data.as_mut();
    state.calc_next();

    ctx.draw_sized(Size::new(label.len() + 3, 1), |frame, rect| {
        let widget = throbber_widgets_tui::Throbber::default()
            .label(label.clone())
            .throbber_set(throbber_widgets_tui::BRAILLE_EIGHT_DOUBLE)
            .use_type(throbber_widgets_tui::WhichUse::Spin);
        frame.render_stateful_widget(widget, rect, state);
    });
}
