use egui::{Color32, Frame, Stroke, Vec2};

use super::*;

/// Style for rendering Snarl.
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
// #[cfg_attr(feature = "egui-probe", derive(egui_probe::EguiProbe))]
pub struct SnarlStyle {
    /// Controls how nodes are laid out.
    /// Defaults to [`NodeLayout::Basic`].
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Option::is_none", default)
    )]
    pub node_layout: Option<NodeLayout>,

    /// Frame used to draw nodes.
    /// Defaults to [`Frame::window`] constructed from current ui's style.
    #[cfg_attr(
        feature = "serde",
        serde(
            skip_serializing_if = "Option::is_none",
            default,
            with = "serde_frame_option"
        )
    )]
    pub node_frame: Option<Frame>,

    /// Frame used to draw node headers.
    /// Defaults to [`node_frame`] without shadow and transparent fill.
    ///
    /// If set, it should not have shadow and fill should be either opaque of fully transparent
    /// unless layering of header fill color with node fill color is desired.
    #[cfg_attr(
        feature = "serde",
        serde(
            skip_serializing_if = "Option::is_none",
            default,
            with = "serde_frame_option"
        )
    )]
    pub header_frame: Option<Frame>,

    /// Blank space for dragging node by its header.
    /// Elements in the header are placed after this space.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Option::is_none", default)
    )]
    pub header_drag_space: Option<Vec2>,

    /// Whether nodes can be collapsed.
    /// If true, headers will have collapsing button.
    /// When collapsed, node will not show its pins, body and footer.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Option::is_none", default)
    )]
    pub collapsible: Option<bool>,

    /// Size of pins.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Option::is_none", default)
    )]
    pub pin_size: Option<f32>,

    /// Default fill color for pins.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Option::is_none", default)
    )]
    pub pin_fill: Option<Color32>,

    /// Default stroke for pins.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Option::is_none", default)
    )]
    pub pin_stroke: Option<Stroke>,

    /// Shape of pins.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Option::is_none", default)
    )]
    pub pin_shape: Option<PinShape>,

    /// Placement of pins.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Option::is_none", default)
    )]
    pub pin_placement: Option<PinPlacement>,

    /// Width of wires.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Option::is_none", default)
    )]
    pub wire_width: Option<f32>,

    /// Size of wire frame which controls curvature of wires.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Option::is_none", default)
    )]
    pub wire_frame_size: Option<f32>,

    /// Whether to downscale wire frame when nodes are close.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Option::is_none", default)
    )]
    pub downscale_wire_frame: Option<bool>,

    /// Weather to upscale wire frame when nodes are far.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Option::is_none", default)
    )]
    pub upscale_wire_frame: Option<bool>,

    /// Controls default style of wires.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Option::is_none", default)
    )]
    pub wire_style: Option<WireStyle>,

    /// Layer where wires are rendered.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Option::is_none", default)
    )]
    pub wire_layer: Option<WireLayer>,

    /// Frame used to draw background
    #[cfg_attr(
        feature = "serde",
        serde(
            skip_serializing_if = "Option::is_none",
            default,
            with = "serde_frame_option"
        )
    )]
    pub bg_frame: Option<Frame>,

    /// Background pattern.
    /// Defaults to [`BackgroundPattern::Grid`].
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Option::is_none", default)
    )]
    pub bg_pattern: Option<BackgroundPattern>,

    /// Stroke for background pattern.
    /// Defaults to `ui.visuals().widgets.noninteractive.bg_stroke`.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Option::is_none", default)
    )]
    pub bg_pattern_stroke: Option<Stroke>,

    /// Minimum viewport scale that can be set.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Option::is_none", default)
    )]
    pub min_scale: Option<f32>,

    /// Maximum viewport scale that can be set.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Option::is_none", default)
    )]
    pub max_scale: Option<f32>,

    /// Velocity of viewport scale when scaling with mouse wheel.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Option::is_none", default)
    )]
    pub scale_velocity: Option<f32>,

    /// Enable centering by double click on background
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Option::is_none", default)
    )]
    pub centering: Option<bool>,

    /// Stroke for selection.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Option::is_none", default)
    )]
    pub select_stoke: Option<Stroke>,

    /// Fill for selection.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Option::is_none", default)
    )]
    pub select_fill: Option<Color32>,

    /// Flag to control how rect selection works.
    /// If set to true, only nodes fully contained in selection rect will be selected.
    /// If set to false, nodes intersecting with selection rect will be selected.
    pub select_rect_contained: Option<bool>,

    /// Style for node selection.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Option::is_none", default)
    )]
    pub select_style: Option<SelectionStyle>,

    #[doc(hidden)]
    #[cfg_attr(feature = "serde", serde(skip_serializing, default))]
    /// Do not access other than with .., here to emulate `#[non_exhaustive(pub)]`
    pub _non_exhaustive: (),
}

impl SnarlStyle {
    pub(crate) fn get_node_layout(&self) -> NodeLayout {
        self.node_layout.unwrap_or(NodeLayout::Basic)
    }

    pub(crate) fn get_pin_size(&self, scale: f32, style: &Style) -> f32 {
        self.pin_size
            .zoomed(scale)
            .unwrap_or(style.spacing.interact_size.y * 0.6)
    }

    pub(crate) fn get_pin_fill(&self, style: &Style) -> Color32 {
        self.pin_fill
            .unwrap_or(style.visuals.widgets.active.bg_fill)
    }

    pub(crate) fn get_pin_stroke(&self, scale: f32, style: &Style) -> Stroke {
        self.pin_stroke.zoomed(scale).unwrap_or_else(|| {
            Stroke::new(
                style.visuals.widgets.active.bg_stroke.width,
                style.visuals.widgets.active.bg_stroke.color,
            )
        })
    }

    pub(crate) fn get_pin_shape(&self) -> PinShape {
        self.pin_shape.unwrap_or(PinShape::Circle)
    }

    pub(crate) fn get_pin_placement(&self) -> PinPlacement {
        self.pin_placement.unwrap_or_default()
    }

    pub(crate) fn get_wire_width(&self, scale: f32, style: &Style) -> f32 {
        self.wire_width
            .zoomed(scale)
            .unwrap_or_else(|| self.get_pin_size(scale, style) * 0.1)
    }

    pub(crate) fn get_wire_frame_size(&self, scale: f32, style: &Style) -> f32 {
        self.wire_frame_size
            .zoomed(scale)
            .unwrap_or_else(|| self.get_pin_size(scale, style) * 3.0)
    }

    pub(crate) fn get_downscale_wire_frame(&self) -> bool {
        self.downscale_wire_frame.unwrap_or(true)
    }

    pub(crate) fn get_upscale_wire_frame(&self) -> bool {
        self.upscale_wire_frame.unwrap_or(false)
    }

    pub(crate) fn get_wire_style(&self, scale: f32) -> WireStyle {
        self.wire_style.zoomed(scale).unwrap_or(WireStyle::Bezier5)
    }

    pub(crate) fn get_wire_layer(&self) -> WireLayer {
        self.wire_layer.unwrap_or(WireLayer::BehindNodes)
    }

    pub(crate) fn get_header_drag_space(&self, scale: f32, style: &Style) -> Vec2 {
        self.header_drag_space
            .zoomed(scale)
            .unwrap_or_else(|| vec2(style.spacing.icon_width, style.spacing.icon_width))
    }

    pub(crate) fn get_collapsible(&self) -> bool {
        self.collapsible.unwrap_or(true)
    }

    pub(crate) fn get_bg_frame(&self, style: &Style) -> Frame {
        self.bg_frame.unwrap_or_else(|| Frame::canvas(style))
    }

    pub(crate) fn get_bg_pattern_stroke(&self, scale: f32, style: &Style) -> Stroke {
        self.bg_pattern_stroke
            .zoomed(scale)
            .unwrap_or(style.visuals.widgets.noninteractive.bg_stroke)
    }

    pub(crate) fn get_min_scale(&self) -> f32 {
        self.min_scale.unwrap_or(0.2)
    }

    pub(crate) fn get_max_scale(&self) -> f32 {
        self.max_scale.unwrap_or(5.0)
    }

    pub(crate) fn get_scale_velocity(&self) -> f32 {
        self.scale_velocity.unwrap_or(1.0)
    }

    pub(crate) fn get_node_frame(&self, scale: f32, style: &Style) -> Frame {
        self.node_frame
            .zoomed(scale)
            .unwrap_or_else(|| Frame::window(style))
    }

    pub(crate) fn get_header_frame(&self, scale: f32, style: &Style) -> Frame {
        self.header_frame
            .zoomed(scale)
            .unwrap_or_else(|| self.get_node_frame(scale, style).shadow(Shadow::NONE))
    }

    pub(crate) fn get_centering(&self) -> bool {
        self.centering.unwrap_or(true)
    }

    pub(crate) fn get_select_stroke(&self, scale: f32, style: &Style) -> Stroke {
        self.select_stoke.zoomed(scale).unwrap_or_else(|| {
            Stroke::new(
                style.visuals.selection.stroke.width,
                style.visuals.selection.stroke.color.gamma_multiply(0.5),
            )
        })
    }

    pub(crate) fn get_select_fill(&self, style: &Style) -> Color32 {
        self.select_fill
            .unwrap_or_else(|| style.visuals.selection.bg_fill.gamma_multiply(0.3))
    }

    pub(crate) fn get_select_rect_contained(&self) -> bool {
        self.select_rect_contained.unwrap_or(false)
    }

    pub(crate) fn get_select_style(&self, scale: f32, style: &Style) -> SelectionStyle {
        self.select_style
            .zoomed(scale)
            .unwrap_or_else(|| SelectionStyle {
                margin: style.spacing.window_margin,
                corner_radius: style.visuals.window_corner_radius,
                fill: self.get_select_fill(style),
                stroke: self.get_select_stroke(scale, style),
            })
    }
}
