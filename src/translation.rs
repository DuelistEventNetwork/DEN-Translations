use std::ptr::NonNull;

pub struct Utf16String {
    pub data: &'static [u16],
}

impl Utf16String {
    /// Safety
    ///
    /// Please don't mutate the data :3
    pub fn as_nonnull(&self) -> NonNull<u16> {
        unsafe { NonNull::new(self.data.as_ptr() as *mut u16).unwrap_unchecked() }
    }
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
