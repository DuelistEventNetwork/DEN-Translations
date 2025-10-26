// Generates rust code using src/translation.rs types and data/*.toml files.
use proc_macro2::TokenStream;
use quote::quote;
use serde::Deserialize;
use std::fs;
use std::path::Path;
use syn::{Expr, LitInt};

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

fn generate_utf16_data(s: &str) -> Vec<LitInt> {
    s.encode_utf16()
        .map(|u| LitInt::new(&u.to_string(), proc_macro2::Span::call_site()))
        .chain(std::iter::once(LitInt::new(
            "0",
            proc_macro2::Span::call_site(),
        )))
        .collect()
}

fn generate_location(location: &Location) -> Expr {
    let entries = location
        .entries
        .iter()
        .map(|entry| {
            let id = entry.id;
            let utf16 = generate_utf16_data(&entry.text);
            quote! { LocationEntry { id: #id, text: Utf16String { data: &[ #(#utf16),* ] } } }
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
            let name_utf16 = generate_utf16_data(&entry.name);
            let info_utf16 = generate_utf16_data(&entry.info);
            let description_utf16 = generate_utf16_data(&entry.description);
            quote! { GoodsEntry { id: #id, name: Utf16String { data: &[ #(#name_utf16),* ] }, info: Utf16String { data: &[ #(#info_utf16),* ] }, description: Utf16String { data: &[ #(#description_utf16),* ] } } }
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
            let utf16 = generate_utf16_data(&entry.text);
            quote! { MenuEntry { id: #id, text: Utf16String { data: &[ #(#utf16),* ] } } }
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
            let utf16 = generate_utf16_data(&entry.text);
            quote! { ActionButtonsEntry { id: #id, text: Utf16String { data: &[ #(#utf16),* ] } } }
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
            let utf16 = generate_utf16_data(&entry.text);
            quote! { EventTextEntry { id: #id, text: Utf16String { data: &[ #(#utf16),* ] } } }
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
            let utf16 = generate_utf16_data(&entry.text);
            quote! { SystemEntry { id: #id, text: Utf16String { data: &[ #(#utf16),* ] } } }
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

/// Attempt to format the tokens with prettyplease,
/// first by parsing as a file, then as a block,
/// otherwise just returns the tokens as a string.
pub fn pretty_parse(tokens: TokenStream) -> String {
    use syn::File;
    match syn::parse2::<File>(tokens.clone()) {
        Ok(file) => prettyplease::unparse(&file),
        Err(_) => {
            // ok its not a file, lets try again putting the tokens in a function
            match syn::parse2::<File>(quote::quote! {
              fn deleteme(){
                  #tokens
              }
            }) {
                Ok(file) => {
                    let mut str = prettyplease::unparse(&file);
                    // remove the outer function
                    str = str.replace("fn deleteme() {\n", "");
                    if let Some(pos) = str.rfind("\n}") {
                        str.replace_range(pos..pos + 3, "");
                    }
                    // remove the function indent
                    str = str
                        .lines()
                        .map(|line| if line.len() >= 4 { &line[4..] } else { line })
                        .collect::<Vec<_>>()
                        .join("\n");
                    str
                }
                Err(_) =>
                // still cant parse, just return the tokens as a string
                {
                    tokens.to_string()
                }
            }
        }
    }
}

fn main() {
    println!("cargo:rerun-if-changed=data/");
    let dest_path = Path::new("src").join("generated.rs");

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
    let mut output = TokenStream::new();

    let header = quote! {
        use crate::translation::*;
    };
    output.extend(header);
    for (name, file) in languages {
        let content = fs::read_to_string(file).unwrap();
        let trans: Translation = toml::from_str(&content).unwrap();
        let code = generate_translation(&trans);
        let name_ident = syn::Ident::new(name, proc_macro2::Span::call_site());
        let decl = quote! {
            pub static #name_ident: Translation = #code;
        };
        output.extend(decl);
    }
    fs::write(&dest_path, pretty_parse(output)).unwrap();
}
