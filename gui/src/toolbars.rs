use gpui::prelude::*;
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
            .h(px(34.))
            .bg(THEME.sidebar)
            .child("Tools")
    }
}

pub struct SideBar {
    layers: Vec<Entity<LayerControl>>,
}

pub struct LayerControl {
    clicked: bool,
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
            .map(|layer| cx.new(|cx| LayerControl::new(cx, layer)))
            .collect();

        Self { layers }
    }
}

impl LayerControl {
    pub fn new(cx: &mut Context<Self>, state: Entity<LayerState>) -> Self {
        Self {
            clicked: false,
            state,
        }
    }
    fn on_click(&mut self, event: &ClickEvent, window: &mut Window, cx: &mut Context<Self>) {
        println!("test");
        self.state.update(cx, |state, cx| {
            state.visible = !state.visible;
            cx.notify();
        });
    }
}

impl Render for LayerControl {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let state = self.state.read(cx);
        div()
            .id("layer_control")
            .flex()
            .on_click(cx.listener(Self::on_click))
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
            .h_full()
            .w(px(200.))
            .border_r_1()
            .border_color(THEME.divider)
            .bg(THEME.sidebar)
            .min_h_0()
            .child("Layers")
            .child(
                div()
                    .flex()
                    .size_full()
                    .items_start()
                    .id("layers_scroll_vert")
                    .overflow_scroll()
                    .child(
                        div().flex().child(
                            div()
                                .flex()
                                .flex_col()
                                .children(self.layers.iter().map(|layer| layer.clone())),
                        ),
                    ),
            )
    }
}
