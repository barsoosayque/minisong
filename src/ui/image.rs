use std::sync::{Arc, LazyLock};

use bevy::prelude::*;
use ratatui_image::{picker::Picker, protocol::StatefulProtocol, StatefulImage};

use super::{
    widget::{WidgetDrawContext, WidgetTag},
    RatatuiAppExt,
};

static TAG: LazyLock<WidgetTag> = LazyLock::new(|| WidgetTag::new::<Data>());

pub struct ImagePlugin;
impl Plugin for ImagePlugin {
    fn build(&self, app: &mut App) {
        app.register_draw_system(*TAG, image_draw_system);
    }
}

#[derive(Component, Default, Clone)]
struct Data {
    buffer: Vec<u8>,
    image: Option<Arc<dyn StatefulProtocol>>,
}

#[derive(Bundle)]
pub struct Image {
    data: Data,
    tag: WidgetTag,
}

impl Image {
    pub fn new(buffer: Vec<u8>) -> Self {
        Self { data: Data { buffer, ..default() }, tag: *TAG }
    }
}

fn image_draw_system(In(mut ctx): In<WidgetDrawContext>, mut data_query: Query<&mut Data>) {
    let Ok(mut data) = data_query.get_mut(ctx.entity()) else {
        return;
    };
    let Data { buffer, image } = data.as_mut();

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
