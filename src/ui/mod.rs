use bevy::ecs::world::Command;
use bevy::utils::{HashMap, HashSet};
use bevy::{input::InputSystem, prelude::*};

use crossterm::event::PopKeyboardEnhancementFlags;
use crossterm::event::{KeyboardEnhancementFlags, PushKeyboardEnhancementFlags};
use crossterm::{
    cursor::SetCursorStyle,
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io::{stdout, Stdout};

mod block;
mod image;
mod input;
mod label;
mod throbber;
mod widget;

pub use block::Block;
pub use image::Image;
pub use label::Label;
pub use throbber::Throbber;
pub use widget::{Align, WidgetAppExt, WidgetBundle, WidgetDrawContext};
use widget::{WidgetSystemId, WidgetTag};

/// Plugin for building and rendering ratatui's UI.
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        let ctx = RatatuiContext::try_new().expect("Error creating a ratatui context");
        app.insert_resource(ctx).add_plugins((
            block::BlockPlugin,
            label::TextPlugin,
            throbber::ThrobberPlugin,
            image::ImagePlugin,
        ));

        app.add_systems(PreUpdate, input::ui_input_system.in_set(InputSystem))
            .add_systems(Last, widget::draw_hierarchy_system);
    }
}

/// Ratatui state.
///
/// Creating this resource will force terminal into an UI mode, and
/// dropping will exit it.
#[derive(Resource)]
pub struct RatatuiContext {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    draw_systems: HashMap<WidgetTag, WidgetSystemId>,
}

impl RatatuiContext {
    /// Try to create a new ratatui state and enter an UI mode.
    pub fn try_new() -> anyhow::Result<Self> {
        Self::enter_ui()?;
        let backend = CrosstermBackend::new(stdout());
        let terminal = Terminal::new(backend)?;
        Ok(Self { terminal, draw_systems: HashMap::new() })
    }

    /// Draw a frame using ratatui.
    pub fn draw(&mut self, mut op: impl FnMut(RatatuiDrawContext) -> anyhow::Result<()>) {
        let _ = self
            .terminal
            .draw(|frame| {
                let ctx = RatatuiDrawContext { draw_systems: &self.draw_systems, frame };
                let _ = op(ctx).inspect_err(|err| error!("Drawing error: {err}"));
            })
            .inspect_err(|err| error!("Internal drawing error: {err}"));
    }

    fn enter_ui() -> anyhow::Result<()> {
        execute!(stdout(), EnterAlternateScreen)?;
        execute!(stdout(), SetCursorStyle::SteadyBar)?;
        execute!(
            stdout(),
            PushKeyboardEnhancementFlags(
                KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES
                    | KeyboardEnhancementFlags::REPORT_ALL_KEYS_AS_ESCAPE_CODES
            )
        )?;
        enable_raw_mode()?;
        Ok(())
    }

    fn exit_ui() -> anyhow::Result<()> {
        execute!(stdout(), LeaveAlternateScreen)?;
        execute!(stdout(), SetCursorStyle::DefaultUserShape)?;
        execute!(stdout(), PopKeyboardEnhancementFlags)?;
        disable_raw_mode()?;
        Ok(())
    }
}

/// Context of current ratatui's drawing frame.
pub struct RatatuiDrawContext<'a, 'b> {
    draw_systems: &'a HashMap<WidgetTag, WidgetSystemId>,
    frame: &'a mut ratatui::Frame<'b>,
}

impl<'a, 'b> RatatuiDrawContext<'a, 'b> {
    /// Get a draw system for `tag`.
    pub fn get_draw_system_id(&self, tag: WidgetTag) -> Option<WidgetSystemId> {
        self.draw_systems.get(&tag).cloned()
    }

    /// Get current drawing frame.
    pub fn frame(&mut self) -> &mut ratatui::Frame<'b> {
        &mut self.frame
    }

    /// Get current drawing frame area.
    fn frame_size(&self) -> ratatui::prelude::Rect {
        self.frame.size()
    }
}

impl Drop for RatatuiContext {
    fn drop(&mut self) {
        let _ = Self::exit_ui();
    }
}

/// Command to despawn all widgets in world.
pub struct DespawnUI;

impl Command for DespawnUI {
    fn apply(self, world: &mut World) {
        let mut query = world.query_filtered::<Entity, (With<WidgetTag>, Without<Parent>)>();
        let widgets = query.iter(world).collect::<HashSet<Entity>>();
        for widget in widgets {
            despawn_with_children_recursive(world, widget);
        }
    }
}
