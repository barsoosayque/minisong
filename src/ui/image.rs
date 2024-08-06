use std::sync::Arc;

use bevy::prelude::*;
use ratatui_image::{picker::Picker, protocol::StatefulProtocol, StatefulImage};

use super::widget::{WidgetAppExt, WidgetDrawContext};

pub struct ImagePlugin;
impl Plugin for ImagePlugin {
    fn build(&self, app: &mut App) {
        app.register_widget::<Image, _>(image_draw_system);
    }
}

#[derive(Component, Default, Clone)]
pub struct Image {
    buffer: Vec<u8>,
    image: Option<Arc<dyn StatefulProtocol>>,
}

impl Image {
    pub fn new(buffer: Vec<u8>) -> Self {
        Self { buffer, ..default() }
    }
}

fn image_draw_system(In(mut ctx): In<WidgetDrawContext>, mut data_query: Query<&mut Image>) {
    let Ok(mut data) = data_query.get_mut(ctx.entity()) else {
        return;
    };
    let Image { buffer, image } = data.as_mut();

    if image.is_none() {
        let mut picker = Picker::from_termios().unwrap();
        picker.guess_protocol();

        let dyn_image = image::load_from_memory(&buffer).unwrap();
        *image = Some(Arc::from(picker.new_resize_protocol(dyn_image)));
    }

    ctx.draw(|_frame, _rect| {
        let _widget = StatefulImage::new(None);
        todo!()
        // let image = Box::from(image)
        // frame.render_stateful_widget(widget, rect, &mut image);
    });
}
