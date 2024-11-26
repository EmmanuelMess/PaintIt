use crate::line::LineState;
use crate::pencil::PencilState;
use crate::rectangle::RectangleState;
use crate::spray::SprayState;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ActionState {
    FreeFormSelect,
    Select,
    Eraser,
    PaintBucket,
    ColorPicker,
    Magnifier,
    Pencil(PencilState),
    Bucket,
    Spray(SprayState),
    InsertText,
    Line(LineState),
    Curve,
    Rectangle(RectangleState),
    Polygon,
    Ellipse,
    RoundedRectangle,
}

#[macro_export]
macro_rules! specify_state {
    ( $action:ident, $state:ident, $expr:expr ) => {
        match $action {
            ActionState::FreeFormSelect => panic!("State not found"),
            ActionState::Select => panic!("State not found"),
            ActionState::Eraser => panic!("State not found"),
            ActionState::PaintBucket => panic!("State not found"),
            ActionState::ColorPicker => panic!("State not found"),
            ActionState::Magnifier => panic!("State not found"),
            ActionState::Pencil(ref mut $state) => $expr,
            ActionState::Bucket => panic!("State not found"),
            ActionState::Spray(ref mut $state) => $expr,
            ActionState::InsertText => panic!("State not found"),
            ActionState::Line(ref mut $state) => $expr,
            ActionState::Curve => panic!("State not found"),
            ActionState::Rectangle(ref mut $state) => $expr,
            ActionState::Polygon => panic!("State not found"),
            ActionState::Ellipse => panic!("State not found"),
            ActionState::RoundedRectangle => panic!("State not found"),
        }
    };
}

impl Into<u32> for ActionState {
    fn into(self) -> u32 {
        match self {
            ActionState::FreeFormSelect => 0,
            ActionState::Select => 1,
            ActionState::Eraser => 2,
            ActionState::PaintBucket => 3,
            ActionState::ColorPicker => 4,
            ActionState::Magnifier => 5,
            ActionState::Pencil(_) => 6,
            ActionState::Bucket => 7,
            ActionState::Spray(_) => 8,
            ActionState::InsertText => 9,
            ActionState::Line(_) => 10,
            ActionState::Curve => 11,
            ActionState::Rectangle(_) => 12,
            ActionState::Polygon => 13,
            ActionState::Ellipse => 14,
            ActionState::RoundedRectangle => 15
        }
    }
}

impl TryFrom<u32> for ActionState {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ActionState::FreeFormSelect),
            1 => Ok(ActionState::Select),
            2 => Ok(ActionState::Eraser),
            3 => Ok(ActionState::PaintBucket),
            4 => Ok(ActionState::ColorPicker),
            5 => Ok(ActionState::Magnifier),
            6 => Ok(ActionState::Pencil(Default::default())),
            7 => Ok(ActionState::Bucket),
            8 => Ok(ActionState::Spray(Default::default())),
            9 => Ok(ActionState::InsertText),
            10 => Ok(ActionState::Line(Default::default())),
            11 => Ok(ActionState::Curve),
            12 => Ok(ActionState::Rectangle(Default::default())),
            13 => Ok(ActionState::Polygon),
            14 => Ok(ActionState::Ellipse),
            15 => Ok(ActionState::RoundedRectangle),
            _ => Err(()),
        }
    }
}