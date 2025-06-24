use gpui::*;
use itertools::Itertools;

use crate::{
    project::{LayerState, ProjectState},
    theme::THEME,
};

pub struct TitleBar;

impl Render for TitleBar {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        div()
            .border_b_1()
            .border_color(THEME.divider)
            .window_control_area(WindowControlArea::Drag)
            .pl(px(71.))
            .bg(THEME.titlebar)
            .child("Project")
    }
}

pub struct ToolBar;

impl Render for ToolBar {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        div()
            .border_b_1()
            .border_color(THEME.divider)
            .flex()
            .h(px(34.))
            .w_full()
            .bg(THEME.sidebar)
            .child("Tools")
    }
}

pub struct SideBar {
    layers: Vec<Entity<LayerControl>>,
}

pub struct LayerControl {
    state: Entity<LayerState>,
}

impl SideBar {
    pub fn new(cx: &mut Context<Self>, state: Entity<ProjectState>) -> Self {
        let layers = state
            .read(cx)
            .layers
            .iter()
            .map(|layer| layer.clone())
            .collect_vec();

        let layers = layers
            .into_iter()
            .map(|layer| cx.new(|_cx| LayerControl { state: layer }))
            .collect();

        Self { layers }
    }
}

impl LayerControl {
    fn on_mouse_down(
        &mut self,
        event: &MouseDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        println!("test")
    }
}

impl Render for LayerControl {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let state = self.state.read(cx);
        div()
            .w_full()
            .on_mouse_down(MouseButton::Left, cx.listener(Self::on_mouse_down))
            .child(format!(
                "{} - {}",
                &state.name,
                if state.visible { "V" } else { "NV" }
            ))
    }
}

impl Render for SideBar {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        div()
            .flex()
            .flex_col()
            .overflow_hidden()
            .h_full()
            .w(px(200.))
            .border_r_1()
            .border_color(THEME.divider)
            .bg(THEME.sidebar)
            .child("Layers")
            .children(self.layers.iter().map(|layer| layer.clone()))
    }
}
