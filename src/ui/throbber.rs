use bevy::prelude::*;

use super::widget::{WidgetAppExt, WidgetDrawContext};

pub struct ThrobberPlugin;
impl Plugin for ThrobberPlugin {
    fn build(&self, app: &mut App) {
        app.register_widget::<Throbber, _>(throbber_draw_system);
    }
}

#[derive(Component, Default, Clone)]
pub struct Throbber {
    pub label: String,
    state: throbber_widgets_tui::ThrobberState,
}

impl Throbber {
    pub fn new(label: impl ToString) -> Self {
        Self { label: label.to_string(), ..default() }
    }
}

fn throbber_draw_system(In(mut ctx): In<WidgetDrawContext>, mut data_query: Query<&mut Throbber>) {
    let Ok(mut data) = data_query.get_mut(ctx.entity()) else {
        return;
    };
    let Throbber { label, state } = data.as_mut();
    state.calc_next();

    ctx.draw_sized((label.len() as u16 + 3, 1), |frame, rect| {
        let widget = throbber_widgets_tui::Throbber::default()
            .label(label.clone())
            .throbber_set(throbber_widgets_tui::BRAILLE_EIGHT_DOUBLE)
            .use_type(throbber_widgets_tui::WhichUse::Spin);
        frame.render_stateful_widget(widget, rect, state);
    });
}
