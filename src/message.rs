use num_derive::{
    FromPrimitive,
    ToPrimitive,
};
use serde::Deserialize;

/// Flattened representations of the socket's messages used
/// within `komorebi_core`.
///
/// These are used to identify specific messages as a flat
/// integer.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Deserialize, FromPrimitive, ToPrimitive)]
#[serde(rename_all = "snake_case")]
pub enum Message {
    #[serde(rename = "focus_left")]
    FocusWindowLeft,
    #[serde(rename = "focus_right")]
    FocusWindowRight,
    #[serde(rename = "focus_up")]
    FocusWindowUp,
    #[serde(rename = "focus_down")]
    FocusWindowDown,

    #[serde(rename = "move_left")]
    MoveWindowLeft,
    #[serde(rename = "move_right")]
    MoveWindowRight,
    #[serde(rename = "move_up")]
    MoveWindowUp,
    #[serde(rename = "move_down")]
    MoveWindowDown,

    #[serde(rename = "resize_edge_left_dec")]
    ResizeWindowEdgeLeftDecrease,
    #[serde(rename = "resize_edge_left_inc")]
    ResizeWindowEdgeLeftIncrease,
    #[serde(rename = "resize_edge_right_dec")]
    ResizeWindowEdgeRightDecrease,
    #[serde(rename = "resize_edge_right_inc")]
    ResizeWindowEdgeRightIncrease,
    #[serde(rename = "resize_edge_up_dec")]
    ResizeWindowEdgeUpDecrease,
    #[serde(rename = "resize_edge_up_inc")]
    ResizeWindowEdgeUpIncrease,
    #[serde(rename = "resize_edge_down_dec")]
    ResizeWindowEdgeDownDecrease,
    #[serde(rename = "resize_edge_down_inc")]
    ResizeWindowEdgeDownIncrease,

    #[serde(rename = "resize_axis_h_dec")]
    ResizeWindowAxisHorizontalDecrease,
    #[serde(rename = "resize_axis_h_inc")]
    ResizeWindowAxisHorizontalIncrease,
    #[serde(rename = "resize_axis_v_dec")]
    ResizeWindowAxisVerticalDecrease,
    #[serde(rename = "resize_axis_v_inc")]
    ResizeWindowAxisVerticalIncrease,
    #[serde(rename = "resize_axis_hv_dec")]
    ResizeWindowAxisHorizontalAndVerticalDecrease,
    #[serde(rename = "resize_axis_hv_inc")]
    ResizeWindowAxisHorizontalAndVerticalIncrease,

    Stop,
}
