use std::any::TypeId;

use bevy::{ecs::system::SystemId, prelude::*};
use ratatui::layout::{Constraint, Direction, Flex, Layout, Rect};

use crate::ui::RatatuiDrawContext;

use super::RatatuiContext;

/// Extension for [`World`] to register new widgets.
pub trait WidgetAppExt {
    /// Register a widget with its drawing system.
    fn register_widget<W: Component, M>(
        &mut self,
        draw_system: impl IntoSystem<WidgetDrawContext, (), M> + 'static,
    ) -> &mut Self;
}

impl WidgetAppExt for App {
    fn register_widget<W: Component, M>(
        &mut self,
        system: impl IntoSystem<WidgetDrawContext, (), M> + 'static,
    ) -> &mut Self {
        let tag = WidgetTag::new::<W>();

        // Register draw system
        if self.world().resource::<RatatuiContext>().draw_systems.contains_key(&tag) {
            warn!("Widget already registered: {}", std::any::type_name::<W>());
            return self;
        }
        let system_id = self.world_mut().register_system(system);
        self.world_mut().resource_mut::<RatatuiContext>().draw_systems.insert(tag, system_id);

        // Add a hook to insert widget tag for this widget
        self.world_mut().register_component_hooks::<W>().on_add(
            |mut world, entity, _component_id| {
                let mut commands = world.commands();
                commands.entity(entity).insert(WidgetTag::new::<W>());
            },
        );

        self
    }
}

/// [`SystemId`] for widget draw systems.
pub type WidgetSystemId = SystemId<WidgetDrawContext, ()>;

/// Widget tag used identify what draw system should be used.
#[derive(Component, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct WidgetTag(pub TypeId);

impl WidgetTag {
    /// Create a new widget tag for widget `C`.
    pub fn new<C: Component>() -> Self {
        Self(std::any::TypeId::of::<C>())
    }
}

/// Widget layout configuration.
#[derive(Component, Default, Clone)]
pub struct WidgetStyle {
    /// Horizontal aligment of self inside the allocated area, usable if widget is sized.
    pub align_horizontal: Align,
    /// Vertical aligment of self inside the allocated area, usable if widget is sized.
    pub align_vertical: Align,
    /// Constraint for this widget allocated area, used by parent's layout.
    pub constraint: Constraint,
    /// Direction of this widget's children widgets.
    pub content_direction: Direction,
    /// Flex mode of this wiudget's children widgets.
    pub content_flex: Flex,
}

/// A singular node of widget tree.
#[derive(Bundle)]
pub struct WidgetBundle<W: Bundle> {
    widget: W,
    style: WidgetStyle,
}

impl<W: Bundle + Default> WidgetBundle<W> {
    /// Create a new default widget.
    pub fn new() -> Self {
        Self::from(W::default())
    }
}

impl<W: Bundle> WidgetBundle<W> {
    /// Create a new widget with data.
    pub fn from(widget: W) -> Self {
        Self { widget, style: WidgetStyle::default() }
    }

    /// See `[WidgetStyle::align_horizontal]`.
    pub fn align_horizontal(self, align: Align) -> Self {
        Self { style: WidgetStyle { align_horizontal: align, ..self.style }, ..self }
    }

    /// See `[WidgetStyle::align_vertical]`.
    pub fn align_vertical(self, align: Align) -> Self {
        Self { style: WidgetStyle { align_vertical: align, ..self.style }, ..self }
    }

    /// See `[WidgetStyle::constraint]`.
    pub fn constraint(self, constraint: Constraint) -> Self {
        Self { style: WidgetStyle { constraint, ..self.style }, ..self }
    }

    /// See `[WidgetStyle::content_direction]`.
    pub fn content_direction(self, content_direction: Direction) -> Self {
        Self { style: WidgetStyle { content_direction, ..self.style }, ..self }
    }

    /// See `[WidgetStyle::content_flex]`.
    pub fn content_flex(self, content_flex: Flex) -> Self {
        Self { style: WidgetStyle { content_flex, ..self.style }, ..self }
    }
}

/// Single widget draw context.
pub struct WidgetDrawContext {
    entity: Entity,
    style: WidgetStyle,
    frame: *mut ratatui::Frame<'static>, // FIXME: why pointer I forgor
    rect: Rect,
}

impl WidgetDrawContext {
    /// Create a new widget draw context.
    fn new(entity: Entity, frame: &mut ratatui::Frame, rect: Rect, style: WidgetStyle) -> Self {
        let frame_cell = unsafe { std::mem::transmute(frame as *mut ratatui::Frame) };
        Self { entity, frame: frame_cell, rect, style }
    }

    /// Whole ratatui frame area.
    pub fn frame(&mut self) -> &mut ratatui::Frame {
        unsafe { std::mem::transmute(self.frame.as_mut().unwrap()) }
    }

    /// Area available for drawing for this widget.
    pub fn rect(&self) -> &Rect {
        &self.rect
    }

    /// Widget entity.
    pub fn entity(&self) -> Entity {
        self.entity
    }

    /// Draw a widget via `op`, but provide `size` (enabling alignment).
    pub fn draw_sized(&mut self, size: Size, op: impl FnOnce(&mut ratatui::Frame, Rect)) {
        let area = self.rect().clone();
        let [area] = Layout::horizontal([Constraint::Length(size.width as u16)])
            .flex(self.style.align_horizontal.as_flex())
            .areas(area);
        let [area] = Layout::vertical([Constraint::Length(size.height as u16)])
            .flex(self.style.align_vertical.as_flex())
            .areas(area);

        op(self.frame(), area);
    }

    /// Draw a widget via `op`.
    pub fn draw(&mut self, op: impl FnOnce(&mut ratatui::Frame, Rect)) {
        let area = self.rect().clone();
        op(self.frame(), area);
    }
}

/// Widget size.
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Size {
    width: usize,
    height: usize,
}

impl Size {
    /// Create a new size.
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }

    /// Create a new size with equal dimensions.
    pub fn rect(size: usize) -> Self {
        Self::new(size, size)
    }
}

/// Widget alignment inside an allocated area.
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum Align {
    #[default]
    Start,
    Center,
    End,
}

impl Align {
    fn as_flex(&self) -> Flex {
        match self {
            Align::Start => Flex::Start,
            Align::Center => Flex::Center,
            Align::End => Flex::End,
        }
    }
}

/// System to draw the whole widget tree.
/// NOTE: it only work with a single root widget.
pub fn draw_hierarchy_system(world: &mut World) {
    world.resource_scope(|world, mut ctx: Mut<RatatuiContext>| {
        ctx.bypass_change_detection().draw(move |mut draw_ctx| {
            let root = {
                let mut query =
                    world.query_filtered::<Entity, (With<WidgetTag>, With<WidgetStyle>, Without<Parent>)>();
                match query.get_single(world) {
                    Ok(entity) => entity,
                    Err(_) => return Ok(()),
                }
            };

            let mut widgets_query = world.query::<(&WidgetTag, &WidgetStyle, Option<&Children>)>();
            let frame_size = draw_ctx.frame_size();
            draw_single(&mut draw_ctx, root, frame_size, world, &mut widgets_query)
        });
    });
}

/// Draw a single widget using it's draw system, then recursively
/// draw its children widgets.
fn draw_single(
    draw_ctx: &mut RatatuiDrawContext,
    entity: Entity,
    area: Rect,
    world: &mut World,
    query: &mut QueryState<(&WidgetTag, &WidgetStyle, Option<&Children>), ()>,
) -> anyhow::Result<()> {
    let (tag, style, children) = query.get(world, entity)?;
    let system_id = draw_ctx
        .get_draw_system_id(*tag)
        .ok_or_else(|| anyhow::anyhow!("No draw system id for tag: {tag:?}"))?;

    let mut children =
        children.iter().flat_map(|children| children.iter().cloned()).collect::<Vec<_>>();

    let child_areas = Layout::default()
        .direction(style.content_direction)
        .constraints(children.iter().map(|child| {
            let child_style = world.get::<WidgetStyle>(*child).unwrap();
            child_style.constraint.clone()
        }))
        .flex(style.content_flex)
        .split(area);

    let widget_draw_context = WidgetDrawContext::new(entity, draw_ctx.frame(), area, style.clone());
    world.run_system_with_input(system_id, widget_draw_context)?;

    for (i, child) in children.drain(..).enumerate() {
        let area = child_areas[i];
        draw_single(draw_ctx, child, area, world, query)?;
    }

    Ok(())
}
