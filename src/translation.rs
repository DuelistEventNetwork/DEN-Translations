pub struct Utf16String {
    pub data: &'static [u16],
}

pub struct LocationEntry {
    pub id: u32,
    pub text: Utf16String,
}

pub struct GoodsEntry {
    pub id: u32,
    pub name: Utf16String,
    pub info: Utf16String,
    pub description: Utf16String,
}

pub struct MenuEntry {
    pub id: u32,
    pub text: Utf16String,
}

pub struct LineHelpEntry {
    pub id: u32,
    pub text: Utf16String,
}

pub struct DialoguesEntry {
    pub id: u32,
    pub text: Utf16String,
}

pub struct ActionButtonsEntry {
    pub id: u32,
    pub text: Utf16String,
}

pub struct ActionButtons {
    pub entries: &'static [ActionButtonsEntry],
}

pub struct EventTextEntry {
    pub id: u32,
    pub text: Utf16String,
}

pub struct EventText {
    pub entries: &'static [EventTextEntry],
}

pub struct TalkEventText {
    pub entries: &'static [EventTextEntry],
}

/// `TutorialBody`, the text a tutorial toast shows. Keyed by the
/// `TUTORIAL_PARAM_ST` row's `textId`.
pub struct TutorialBody {
    pub entries: &'static [EventTextEntry],
}

pub struct SystemEntry {
    pub id: u32,
    pub text: Utf16String,
}

pub struct System {
    pub entries: &'static [SystemEntry],
}

pub struct Menu {
    pub entries: &'static [MenuEntry],
}

pub struct LineHelp {
    pub entries: &'static [LineHelpEntry],
}

pub struct Dialogues {
    pub entries: &'static [DialoguesEntry],
}

pub struct Location {
    pub entries: &'static [LocationEntry],
}
pub struct Goods {
    pub entries: &'static [GoodsEntry],
}

pub enum CustomSegment {
    Literal(&'static [u16]),
    Arg(usize),
}

pub struct CustomEntry {
    pub text: &'static [u16],
    pub segments: &'static [CustomSegment],
}

pub struct Translation {
    pub location: Location,
    pub goods: Goods,
    pub menu: Menu,
    pub line_help: LineHelp,
    pub dialogues: Dialogues,
    pub action_buttons: ActionButtons,
    pub event_text: EventText,
    pub talk_event_text: TalkEventText,
    pub tutorial_body: TutorialBody,
    pub system: System,
    pub custom: crate::generated::Custom,
}
