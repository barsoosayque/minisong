use bevy::prelude::*;
use ratatui_image::{picker::Picker, protocol::StatefulProtocol, StatefulImage};

use super::widget::{WidgetAppExt, WidgetDrawContext};

pub struct ImagePlugin;
impl Plugin for ImagePlugin {
    fn build(&self, app: &mut App) {
        app.register_widget::<Image, _>(image_draw_system);
    }
}

#[derive(Component)]
pub struct Image {
    image: Box<dyn StatefulProtocol>,
}

impl Image {
    pub fn try_new(buffer: Vec<u8>) -> anyhow::Result<Self> {
        let mut picker = Picker::from_termios().unwrap();
        // alacritty doesn't work well with guessing
        // picker.guess_protocol();

        let dyn_image = image::load_from_memory(&buffer)?;
        let image = Box::from(picker.new_resize_protocol(dyn_image));
        Ok(Self { image })
    }
}

fn image_draw_system(In(mut ctx): In<WidgetDrawContext>, mut data_query: Query<&mut Image>) {
    let Ok(mut data) = data_query.get_mut(ctx.entity()) else {
        return;
    };
    let Image { image } = data.as_mut();

    ctx.draw(|frame, rect| {
        let widget = StatefulImage::new(None);
        frame.render_stateful_widget(widget, rect, image);
    });
}
