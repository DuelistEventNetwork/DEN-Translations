// Generates rust code using src/translation.rs types and data/*.toml files.
use quote::{ToTokens, quote};
use serde::Deserialize;
use std::fs;
use std::path::Path;
use syn::{Expr, LitStr};

#[derive(Deserialize)]
struct LocationEntry {
    id: u32,
    text: String,
}

#[derive(Deserialize)]
struct Location {
    entries: Vec<LocationEntry>,
}

#[derive(Deserialize)]
struct GoodsEntry {
    id: u32,
    name: String,
    info: String,
    description: String,
}

#[derive(Deserialize)]
struct Goods {
    entries: Vec<GoodsEntry>,
}

#[derive(Deserialize)]
struct MenuEntry {
    id: u32,
    text: String,
}

#[derive(Deserialize)]
struct Menu {
    entries: Vec<MenuEntry>,
}

#[derive(Deserialize)]
struct ActionButtonsEntry {
    id: u32,
    text: String,
}

#[derive(Deserialize)]
struct ActionButtons {
    entries: Vec<ActionButtonsEntry>,
}

#[derive(Deserialize)]
struct EventTextEntry {
    id: u32,
    text: String,
}

#[derive(Deserialize)]
struct EventText {
    entries: Vec<EventTextEntry>,
}

#[derive(Deserialize)]
struct SystemEntry {
    id: u32,
    text: String,
}

#[derive(Deserialize)]
struct System {
    entries: Vec<SystemEntry>,
}

#[derive(Deserialize)]
struct Translation {
    location: Location,
    goods: Goods,
    menu: Menu,
    action_buttons: ActionButtons,
    event_text: EventText,
    system: System,
}

fn escape_string(s: &str) -> String {
    s.replace("\\", "\\\\")
        .replace("\"", "\\\"")
        .replace("\n", "\\n")
}

fn generate_location(location: &Location) -> Expr {
    let entries = location
        .entries
        .iter()
        .map(|entry| {
            let id = entry.id;
            let text = LitStr::new(&escape_string(&entry.text), proc_macro2::Span::call_site());
            quote! { LocationEntry { id: #id, text: #text } }
        })
        .collect::<Vec<_>>();
    syn::parse2(quote! { Location { entries: &[ #(#entries),* ] } }).unwrap()
}

fn generate_goods(goods: &Goods) -> Expr {
    let entries = goods
        .entries
        .iter()
        .map(|entry| {
            let id = entry.id;
            let name = LitStr::new(&escape_string(&entry.name), proc_macro2::Span::call_site());
            let info = LitStr::new(&escape_string(&entry.info), proc_macro2::Span::call_site());
            let description = LitStr::new(
                &escape_string(&entry.description),
                proc_macro2::Span::call_site(),
            );
            quote! { GoodsEntry { id: #id, name: #name, info: #info, description: #description } }
        })
        .collect::<Vec<_>>();
    syn::parse2(quote! { Goods { entries: &[ #(#entries),* ] } }).unwrap()
}

fn generate_menu(menu: &Menu) -> Expr {
    let entries = menu
        .entries
        .iter()
        .map(|entry| {
            let id = entry.id;
            let text = LitStr::new(&escape_string(&entry.text), proc_macro2::Span::call_site());
            quote! { MenuEntry { id: #id, text: #text } }
        })
        .collect::<Vec<_>>();
    syn::parse2(quote! { Menu { entries: &[ #(#entries),* ] } }).unwrap()
}

fn generate_action_buttons(action_buttons: &ActionButtons) -> Expr {
    let entries = action_buttons
        .entries
        .iter()
        .map(|entry| {
            let id = entry.id;
            let text = LitStr::new(&escape_string(&entry.text), proc_macro2::Span::call_site());
            quote! { ActionButtonsEntry { id: #id, text: #text } }
        })
        .collect::<Vec<_>>();
    syn::parse2(quote! { ActionButtons { entries: &[ #(#entries),* ] } }).unwrap()
}

fn generate_event_text(event_text: &EventText) -> Expr {
    let entries = event_text
        .entries
        .iter()
        .map(|entry| {
            let id = entry.id;
            let text = LitStr::new(&escape_string(&entry.text), proc_macro2::Span::call_site());
            quote! { EventTextEntry { id: #id, text: #text } }
        })
        .collect::<Vec<_>>();
    syn::parse2(quote! { EventText { entries: &[ #(#entries),* ] } }).unwrap()
}

fn generate_system(system: &System) -> Expr {
    let entries = system
        .entries
        .iter()
        .map(|entry| {
            let id = entry.id;
            let text = LitStr::new(&escape_string(&entry.text), proc_macro2::Span::call_site());
            quote! { SystemEntry { id: #id, text: #text } }
        })
        .collect::<Vec<_>>();
    syn::parse2(quote! { System { entries: &[ #(#entries),* ] } }).unwrap()
}

fn generate_translation(trans: &Translation) -> Expr {
    let location = generate_location(&trans.location);
    let goods = generate_goods(&trans.goods);
    let menu = generate_menu(&trans.menu);
    let action_buttons = generate_action_buttons(&trans.action_buttons);
    let event_text = generate_event_text(&trans.event_text);
    let system = generate_system(&trans.system);
    syn::parse2(quote! {
        Translation {
            location: #location,
            goods: #goods,
            menu: #menu,
            action_buttons: #action_buttons,
            event_text: #event_text,
            system: #system,
        }
    })
    .unwrap()
}

fn main() {
    println!("cargo:rerun-if-changed=data/");
    let dest_path = Path::new("src").join("generated.rs");
    let mut output = "use crate::translation::*;\n\n".to_string();

    let languages = vec![
        ("ENGLISH", "data/english.toml"),
        ("BRAZILIAN_PORTUGUESE", "data/brazilian_portuguese.toml"),
        ("CHINESE_SIMPLIFIED", "data/chinese_simplified.toml"),
        ("CHINESE_TRADITIONAL", "data/chinese_traditional.toml"),
        ("FRENCH", "data/french.toml"),
        ("GERMAN", "data/german.toml"),
        ("ITALIAN", "data/italian.toml"),
        ("JAPANESE", "data/japanese.toml"),
        ("KOREAN", "data/korean.toml"),
        ("POLISH", "data/polish.toml"),
        ("RUSSIAN", "data/russian.toml"),
        ("SPANISH_EU", "data/spanish_eu.toml"),
        ("SPANISH_LATIN", "data/spanish_latin.toml"),
        ("THAI", "data/thai.toml"),
    ];

    for (name, file) in languages {
        let content = fs::read_to_string(file).unwrap();
        let trans: Translation = toml::from_str(&content).unwrap();
        let code = generate_translation(&trans);
        let code_str = code.to_token_stream().to_string();
        output.push_str(&format!("pub static {name}: Translation = {code_str};\n\n"));
    }

    fs::write(&dest_path, output).unwrap();
}
