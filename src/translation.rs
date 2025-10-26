pub struct LocationEntry {
    pub id: u32,
    pub text: &'static str,
}

pub struct GoodsEntry {
    pub id: u32,
    pub name: &'static str,
    pub info: &'static str,
    pub description: &'static str,
}

pub struct MenuEntry {
    pub id: u32,
    pub text: &'static str,
}

pub struct ActionButtonsEntry {
    pub id: u32,
    pub text: &'static str,
}

pub struct ActionButtons {
    pub entries: &'static [ActionButtonsEntry],
}

pub struct EventTextEntry {
    pub id: u32,
    pub text: &'static str,
}

pub struct EventText {
    pub entries: &'static [EventTextEntry],
}

pub struct SystemEntry {
    pub id: u32,
    pub text: &'static str,
}

pub struct System {
    pub entries: &'static [SystemEntry],
}

pub struct Menu {
    pub entries: &'static [MenuEntry],
}

pub struct Location {
    pub entries: &'static [LocationEntry],
}
pub struct Goods {
    pub entries: &'static [GoodsEntry],
}

pub struct Translation {
    pub location: Location,
    pub goods: Goods,
    pub menu: Menu,
    pub action_buttons: ActionButtons,
    pub event_text: EventText,
    pub system: System,
}
