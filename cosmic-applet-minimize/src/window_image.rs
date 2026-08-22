// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use lingmo::{
    Element,
    desktop::{IconSourceExt, fde},
    iced::Limits,
    iced::core::{Border, Layout, Length, Size, Vector, layout, overlay, widget::Tree},
    theme::{Button, Container},
    widget::{Image, Widget, button, container, image::Handle},
};

use crate::wayland_subscription::WaylandImage;

pub struct WindowImage<'a, Msg> {
    image_button: Element<'a, Msg>,
    icon: Element<'a, Msg>,
}

impl<Msg> WindowImage<'_, Msg>
where
    Msg: 'static + Clone,
{
    pub fn new(
        img: Option<WaylandImage>,
        icon: &fde::IconSource,
        size: f32,
        on_press: Msg,
        padding: (u16, u16),
    ) -> Self {
        let border = 1.0;
        Self {
            image_button: button::custom(
                container(
                    container(if let Some(img) = img {
                        let max_dim = img.width.max(img.height).max(1);
                        let ratio = max_dim as f32 / (size - border * 2.0).max(1.0);
                        let adjusted_width = img.width as f32 / ratio;
                        let adjusted_height = img.height as f32 / ratio;

                        Element::from(
                            Image::new(Handle::from_rgba(img.width, img.height, img.img))
                                .width(Length::Fixed(adjusted_width))
                                .height(Length::Fixed(adjusted_height))
                                .content_fit(lingmo::iced::core::ContentFit::Contain),
                        )
                    } else {
                        Element::from(
                            lingmo::widget::icon(icon.as_cosmic_icon())
                                .width(Length::Fixed((size - border * 2.0).max(0.)))
                                .height(Length::Fixed((size - border * 2.0).max(0.))),
                        )
                    })
                    .class(Container::Custom(Box::new(move |theme| container::Style {
                        border: Border {
                            color: theme.cosmic().bg_divider().into(),
                            width: border,
                            radius: 0.0.into(),
                        },
                        ..Default::default()
                    })))
                    .padding(border as u16)
                    .height(Length::Shrink)
                    .width(Length::Shrink),
                )
                .center_x(Length::Fixed(size + padding.0 as f32 * 2.0))
                .center_y(Length::Fixed(size + padding.1 as f32 * 2.0))
                .padding([padding.0 as f32, padding.1 as f32]),
            )
            .on_press(on_press)
            .width(Length::Shrink)
            .height(Length::Shrink)
            .class(Button::AppletIcon)
            .padding(0)
            .into(),
            icon: lingmo::widget::icon(icon.as_cosmic_icon())
                .width(Length::Fixed(size / 3.0))
                .height(Length::Fixed(size / 3.0))
                .into(),
        }
    }
}

impl<Msg> Widget<Msg, lingmo::Theme, lingmo::Renderer> for WindowImage<'_, Msg> {
    fn children(&self) -> Vec<lingmo::iced::core::widget::Tree> {
        vec![Tree::new(&self.image_button), Tree::new(&self.icon)]
    }

    fn diff(&mut self, tree: &mut lingmo::iced::core::widget::Tree) {
        tree.diff_children(&mut [&mut self.image_button, &mut self.icon]);
    }

    fn overlay<'b>(
        &'b mut self,
        state: &'b mut Tree,
        layout: Layout<'b>,
        renderer: &lingmo::Renderer,
        viewport: &lingmo::iced::core::Rectangle,
        translation: Vector,
    ) -> Option<lingmo::iced::core::overlay::Element<'b, Msg, lingmo::Theme, lingmo::Renderer>>
    {
        let children = [&mut self.image_button, &mut self.icon]
            .into_iter()
            .zip(&mut state.children)
            .zip(layout.children())
            .filter_map(|((child, state), layout)| {
                child
                    .as_widget_mut()
                    .overlay(state, layout, renderer, viewport, translation)
            })
            .collect::<Vec<_>>();

        (!children.is_empty()).then(|| overlay::Group::with_children(children).overlay())
    }

    fn size(&self) -> Size<Length> {
        Size::new(Length::Shrink, Length::Shrink)
    }

    fn layout(
        &mut self,
        tree: &mut lingmo::iced::core::widget::Tree,
        renderer: &lingmo::Renderer,
        limits: &lingmo::iced::core::layout::Limits,
    ) -> lingmo::iced::core::layout::Node {
        let children = &mut tree.children;
        let button = &mut children[0];
        let button_node = self
            .image_button
            .as_widget_mut()
            .layout(button, renderer, limits);
        let img_node = &button_node.children()[0].children()[0];

        let button_bounds = img_node.size();
        let icon_width = button_bounds.width.max(button_bounds.height) / 3.0;
        let icon_height = button_bounds.height.max(button_bounds.width) / 3.0;
        let icon = &mut children[1];
        let icon_node = self
            .icon
            .as_widget_mut()
            .layout(
                icon,
                renderer,
                &Limits::NONE.width(icon_width).height(icon_height),
            )
            .translate(Vector::new(
                img_node.bounds().x + 2. * button_bounds.width / 3.0,
                img_node.bounds().y + 2. * button_bounds.height / 3.0,
            ));

        layout::Node::with_children(
            limits.resolve(Length::Shrink, Length::Shrink, button_node.size()),
            vec![button_node, icon_node],
        )
    }

    fn draw(
        &self,
        tree: &lingmo::iced::core::widget::Tree,
        renderer: &mut lingmo::Renderer,
        theme: &lingmo::Theme,
        style: &lingmo::iced::core::renderer::Style,
        layout: lingmo::iced::core::Layout<'_>,
        cursor: lingmo::iced::core::mouse::Cursor,
        viewport: &lingmo::iced::core::Rectangle,
    ) {
        let children = &[&self.image_button, &self.icon];
        // draw children in order
        for (i, (layout, child)) in layout.children().zip(children).enumerate() {
            let tree = &tree.children[i];
            child
                .as_widget()
                .draw(tree, renderer, theme, style, layout, cursor, viewport);
        }
    }

    fn size_hint(&self) -> Size<Length> {
        self.size()
    }

    fn tag(&self) -> lingmo::iced::core::widget::tree::Tag {
        lingmo::iced::core::widget::tree::Tag::stateless()
    }

    fn state(&self) -> lingmo::iced::core::widget::tree::State {
        lingmo::iced::core::widget::tree::State::None
    }

    fn operate(
        &mut self,
        tree: &mut lingmo::iced::core::widget::Tree,
        layout: lingmo::iced::core::Layout<'_>,
        renderer: &lingmo::Renderer,
        operation: &mut dyn lingmo::widget::Operation<()>,
    ) {
        let layout = layout.children().collect::<Vec<_>>();
        let children = [&mut self.image_button, &mut self.icon];
        for (i, (layout, child)) in layout
            .into_iter()
            .zip(children.into_iter())
            .enumerate()
            .rev()
        {
            let tree = &mut tree.children[i];
            child
                .as_widget_mut()
                .operate(tree, layout, renderer, operation);
        }
    }

    fn update(
        &mut self,
        state: &mut lingmo::iced::core::widget::Tree,
        event: &lingmo::iced::core::Event,
        layout: lingmo::iced::core::Layout<'_>,
        cursor: lingmo::iced::core::mouse::Cursor,
        renderer: &lingmo::Renderer,
        clipboard: &mut dyn lingmo::iced::core::Clipboard,
        shell: &mut lingmo::iced::core::Shell<'_, Msg>,
        viewport: &lingmo::iced::core::Rectangle,
    ) {
        let children = [&mut self.image_button, &mut self.icon];

        let layout = layout.children().collect::<Vec<_>>();
        // draw children in order
        for (i, (layout, child)) in layout
            .into_iter()
            .zip(children.into_iter())
            .enumerate()
            .rev()
        {
            let tree = &mut state.children[i];

            child.as_widget_mut().update(
                tree, event, layout, cursor, renderer, clipboard, shell, viewport,
            );
            if shell.is_event_captured() {
                return;
            }
        }
    }

    fn mouse_interaction(
        &self,
        state: &lingmo::iced::core::widget::Tree,
        layout: lingmo::iced::core::Layout<'_>,
        cursor: lingmo::iced::core::mouse::Cursor,
        viewport: &lingmo::iced::core::Rectangle,
        renderer: &lingmo::Renderer,
    ) -> lingmo::iced::core::mouse::Interaction {
        let children = [&self.image_button, &self.icon];
        let layout = layout.children().collect::<Vec<_>>();
        for (i, (layout, child)) in layout
            .into_iter()
            .zip(children.into_iter())
            .enumerate()
            .rev()
        {
            let tree = &state.children[i];
            let interaction = child
                .as_widget()
                .mouse_interaction(tree, layout, cursor, viewport, renderer);
            if cursor.is_over(layout.bounds()) {
                return interaction;
            }
        }
        lingmo::iced::core::mouse::Interaction::Idle
    }

    fn id(&self) -> Option<lingmo::widget::Id> {
        None
    }

    fn set_id(&mut self, _id: lingmo::widget::Id) {}
}

impl<'a, Message> From<WindowImage<'a, Message>> for lingmo::Element<'a, Message>
where
    Message: 'static + Clone,
{
    fn from(w: WindowImage<'a, Message>) -> lingmo::Element<'a, Message> {
        Element::new(w)
    }
}
