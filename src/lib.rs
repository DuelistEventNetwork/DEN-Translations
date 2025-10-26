mod generated;
mod translation;

pub use generated::*;
pub use translation::*;

#[repr(u32)]
pub enum LanguageId {
    JpnJP = 0,
    EngUS = 1,
    FraFR = 2,
    SpaES = 3,
    ItaIT = 4,
    DeuDE = 5,
    KorKR = 6,
    ZhoTW = 7,
    ZhoCN = 8,
    PolPL = 9,
    RusRU = 10,
    PorBR = 11,
    SpaAR = 12,
    ThaTH = 13,
    AraAE = 14,
}

impl LanguageId {
    pub fn from_steam_api(steam_api_lang: &'static str) -> Self {
        // Taken directly from Elden Ring's implementation
        match steam_api_lang {
            "brazilian" => LanguageId::PorBR,
            "bulgarian" => LanguageId::EngUS,
            "czech" => LanguageId::EngUS,
            "danish" => LanguageId::EngUS,
            "dutch" => LanguageId::EngUS,
            "english" => LanguageId::EngUS,
            "finnish" => LanguageId::EngUS,
            "french" => LanguageId::FraFR,
            "german" => LanguageId::DeuDE,
            "greek" => LanguageId::EngUS,
            "hungarian" => LanguageId::EngUS,
            "italian" => LanguageId::ItaIT,
            "japanese" => LanguageId::JpnJP,
            "koreana" => LanguageId::KorKR,
            "norwegian" => LanguageId::EngUS,
            "polish" => LanguageId::PolPL,
            "portugues" => LanguageId::EngUS,
            "romanian" => LanguageId::EngUS,
            "russian" => LanguageId::RusRU,
            "schinese" => LanguageId::ZhoCN,
            "spanish" => LanguageId::SpaES,
            "swedish" => LanguageId::EngUS,
            "tchinese" => LanguageId::ZhoTW,
            "thai" => LanguageId::ThaTH,
            "turkish" => LanguageId::EngUS,
            "ukrainian" => LanguageId::EngUS,
            "latam" => LanguageId::SpaAR,
            "vietnamese" => LanguageId::EngUS,
            "arabic" => LanguageId::AraAE,
            _ => LanguageId::EngUS,
        }
    }

    pub fn to_translation_bank(&self) -> &'static Translation {
        match self {
            LanguageId::JpnJP => &generated::JAPANESE,
            LanguageId::EngUS => &generated::ENGLISH,
            LanguageId::FraFR => &generated::FRENCH,
            LanguageId::SpaES => &generated::SPANISH_EU,
            LanguageId::ItaIT => &generated::ITALIAN,
            LanguageId::DeuDE => &generated::GERMAN,
            LanguageId::KorKR => &generated::KOREAN,
            LanguageId::ZhoTW => &generated::CHINESE_TRADITIONAL,
            LanguageId::ZhoCN => &generated::CHINESE_SIMPLIFIED,
            LanguageId::PolPL => &generated::POLISH,
            LanguageId::RusRU => &generated::RUSSIAN,
            LanguageId::PorBR => &generated::BRAZILIAN_PORTUGUESE,
            LanguageId::SpaAR => &generated::SPANISH_LATIN,
            LanguageId::ThaTH => &generated::THAI,
            // No Arabic translation available yet
            LanguageId::AraAE => &generated::ENGLISH,
        }
    }
}
