use static_toml::static_toml;

static_toml! {
    pub static ENGLISH = include_toml!("data/english.toml");
    pub static BRAZILIAN_PORTUGUESE = include_toml!("data/brazilian_portuguese.toml");
    pub static CHINESE_SIMPLIFIED = include_toml!("data/chinese_simplified.toml");
    pub static CHINESE_TRADITIONAL = include_toml!("data/chinese_traditional.toml");
    pub static FRENCH = include_toml!("data/french.toml");
    pub static GERMAN = include_toml!("data/german.toml");
    pub static ITALIAN = include_toml!("data/italian.toml");
    pub static JAPANESE = include_toml!("data/japanese.toml");
    pub static KOREAN = include_toml!("data/korean.toml");
    pub static POLISH = include_toml!("data/polish.toml");
    pub static RUSSIAN = include_toml!("data/russian.toml");
    pub static SPANISH_EU = include_toml!("data/spanish_eu.toml");
    pub static SPANISH_LATIN = include_toml!("data/spanish_latin.toml");
    pub static THAI = include_toml!("data/thai.toml");
}

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
}
