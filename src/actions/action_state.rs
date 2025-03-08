use std::ops::{Deref, DerefMut};
use crate::actions::brush::BrushState;
use crate::actions::bucket::BucketState;
use crate::actions::color_picker::ColorPickerState;
use crate::actions::ellipse::EllipseState;
use crate::actions::eraser::EraserState;
use crate::actions::line::LineState;
use crate::actions::pencil::PencilState;
use crate::actions::polygon::PolygonState;
use crate::actions::rectangle::RectangleState;
use crate::actions::rounded_rectangle::RoundedRectangleState;
use crate::actions::select::SelectState;
use crate::actions::spray::SprayState;

#[derive(Debug, Clone)]
pub enum ActionState {
    FreeFormSelect,
    Select(SelectState),
    Eraser(EraserState),
    PaintBucket(BucketState),
    ColorPicker(ColorPickerState),
    Magnifier,
    Pencil(PencilState),
    Brush(BrushState),
    Spray(SprayState),
    InsertText,
    Line(LineState),
    Curve,
    Rectangle(RectangleState),
    Polygon(PolygonState),
    Ellipse(EllipseState),
    RoundedRectangle(RoundedRectangleState),
}

#[macro_export]
macro_rules! specify_state {
    ( $action:ident, $state:ident, $expr:expr ) => {
        match $action {
            ActionState::FreeFormSelect => panic!("State not found"),
            ActionState::Select($state) => $expr,
            ActionState::Eraser($state) => $expr,
            ActionState::PaintBucket(ref mut $state) => $expr,
            ActionState::ColorPicker(ref mut $state) => $expr,
            ActionState::Magnifier => panic!("State not found"),
            ActionState::Pencil(ref mut $state) => $expr,
            ActionState::Brush(ref mut $state) => $expr,
            ActionState::Spray(ref mut $state) => $expr,
            ActionState::InsertText => panic!("State not found"),
            ActionState::Line(ref mut $state) => $expr,
            ActionState::Curve => panic!("State not found"),
            ActionState::Rectangle(ref mut $state) => $expr,
            ActionState::Polygon(ref mut $state) => $expr,
            ActionState::Ellipse(ref mut $state) => $expr,
            ActionState::RoundedRectangle(ref mut $state) => $expr,
        }
    };
}

impl From<&ActionState> for u32 {
    fn from(value: &ActionState) -> u32 {
        match value {
            ActionState::FreeFormSelect => 0,
            ActionState::Select(_) => 1,
            ActionState::Eraser(_) => 2,
            ActionState::PaintBucket(_) => 3,
            ActionState::ColorPicker(_) => 4,
            ActionState::Magnifier => 5,
            ActionState::Pencil(_) => 6,
            ActionState::Brush(_) => 7,
            ActionState::Spray(_) => 8,
            ActionState::InsertText => 9,
            ActionState::Line(_) => 10,
            ActionState::Curve => 11,
            ActionState::Rectangle(_) => 12,
            ActionState::Polygon(_) => 13,
            ActionState::Ellipse(_) => 14,
            ActionState::RoundedRectangle(_) => 15
        }
    }
}

impl TryFrom<u32> for ActionState {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ActionState::FreeFormSelect),
            1 => Ok(ActionState::Select(Default::default())),
            2 => Ok(ActionState::Eraser(Default::default())),
            3 => Ok(ActionState::PaintBucket(Default::default())),
            4 => Ok(ActionState::ColorPicker(Default::default())),
            5 => Ok(ActionState::Magnifier),
            6 => Ok(ActionState::Pencil(Default::default())),
            7 => Ok(ActionState::Brush(Default::default())),
            8 => Ok(ActionState::Spray(Default::default())),
            9 => Ok(ActionState::InsertText),
            10 => Ok(ActionState::Line(Default::default())),
            11 => Ok(ActionState::Curve),
            12 => Ok(ActionState::Rectangle(Default::default())),
            13 => Ok(ActionState::Polygon(Default::default())),
            14 => Ok(ActionState::Ellipse(Default::default())),
            15 => Ok(ActionState::RoundedRectangle(Default::default())),
            _ => Err(()),
        }
    }
}

impl Deref for ActionState {
    type Target = ActionState;

    fn deref(&self) -> &Self::Target {
        &self
    }
}

impl DerefMut for ActionState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self
    }
}
