use hbb_common::regex::Regex;
use std::ops::Deref;

mod ar;
mod be;
mod bg;
mod ca;
mod cn;
mod cs;
mod da;
mod de;
mod el;
mod en;
mod eo;
mod es;
mod et;
mod eu;
mod fa;
mod fr;
mod he;
mod hr;
mod hu;
mod id;
mod it;
mod ja;
mod ko;
mod kz;
mod lt;
mod lv;
mod nb;
mod nl;
mod pl;
mod ptbr;
mod ro;
mod ru;
mod sc;
mod sk;
mod sl;
mod sq;
mod sr;
mod sv;
mod th;
mod tr;
mod tw;
mod uk;
mod vi;
mod ta;
mod ge;
mod fi;

pub const LANGS: &[(&str, &str)] = &[
    ("en", "English"),
    ("it", "Italiano"),
    ("fr", "Fran莽ais"),
    ("de", "Deutsch"),
    ("nl", "Nederlands"),
    ("nb", "Norsk bokm氓l"),
    ("zh-cn", "绠€浣撲腑鏂?),
    ("zh-tw", "绻侀珨涓枃"),
    ("pt", "Portugu锚s"),
    ("es", "Espa帽ol"),
    ("et", "Eesti keel"),
    ("eu", "Euskara"),
    ("hu", "Magyar"),
    ("bg", "袘褗谢谐邪褉褋泻懈"),
    ("be", "袘械谢邪褉褍褋泻邪褟"),
    ("ru", "袪褍褋褋泻懈泄"),
    ("sk", "Sloven膷ina"),
    ("id", "Indonesia"),
    ("cs", "膶e拧tina"),
    ("da", "Dansk"),
    ("eo", "Esperanto"),
    ("tr", "T眉rk莽e"),
    ("vi", "Ti岷縩g Vi峄噒"),
    ("pl", "Polski"),
    ("ja", "鏃ユ湰瑾?),
    ("ko", "頃滉淡鞏?),
    ("kz", "覛邪蟹邪覜"),
    ("uk", "校泻褉邪褩薪褋褜泻邪"),
    ("fa", "賮丕乇爻蹖"),
    ("ca", "Catal脿"),
    ("el", "螘位位畏谓喂魏维"),
    ("sv", "Svenska"),
    ("sq", "Shqip"),
    ("sr", "Srpski"),
    ("th", "喔犩覆喔┼覆喙勦笚喔?),
    ("sl", "Sloven拧膷ina"),
    ("ro", "Rom芒n膬"),
    ("lt", "Lietuvi懦"),
    ("lv", "Latvie拧u"),
    ("ar", "丕賱毓乇亘賷丞"),
    ("he", "注讘专讬转"),
    ("hr", "Hrvatski"),
    ("sc", "Sardu"),
    ("ta", "喈む喈苦喁?),
    ("ge", "醿メ儛醿犪儣醿ａ儦醿?),
    ("fi", "Suomi"),
];

#[cfg(not(any(target_os = "android", target_os = "ios")))]
pub fn translate(name: String) -> String {
    let locale = sys_locale::get_locale().unwrap_or_default();
    translate_locale(name, &locale)
}

pub fn translate_locale(name: String, locale: &str) -> String {
    let locale = locale.to_lowercase();
    let mut lang = hbb_common::config::LocalConfig::get_option("lang").to_lowercase();
    if lang.is_empty() {
        // zh_CN on Linux, zh-Hans-CN on mac, zh_CN_#Hans on Android
        if locale.starts_with("zh") {
            lang = (if locale.contains("tw") {
                "zh-tw"
            } else {
                "zh-cn"
            })
            .to_owned();
        }
    }
    if lang.is_empty() {
        lang = locale
            .split("-")
            .next()
            .map(|x| x.split("_").next().unwrap_or_default())
            .unwrap_or_default()
            .to_owned();
    }
    let lang = lang.to_lowercase();
    let m = match lang.as_str() {
        "fr" => fr::T.deref(),
        "zh-cn" => cn::T.deref(),
        "it" => it::T.deref(),
        "zh-tw" => tw::T.deref(),
        "de" => de::T.deref(),
        "nb" => nb::T.deref(),
        "nl" => nl::T.deref(),
        "es" => es::T.deref(),
        "et" => et::T.deref(),
        "eu" => eu::T.deref(),
        "hu" => hu::T.deref(),
        "ru" => ru::T.deref(),
        "eo" => eo::T.deref(),
        "id" => id::T.deref(),
        "br" => ptbr::T.deref(),
        "pt" => ptbr::T.deref(),
        "tr" => tr::T.deref(),
        "cs" => cs::T.deref(),
        "da" => da::T.deref(),
        "sk" => sk::T.deref(),
        "vi" => vi::T.deref(),
        "pl" => pl::T.deref(),
        "ja" => ja::T.deref(),
        "ko" => ko::T.deref(),
        "kz" => kz::T.deref(),
        "uk" => uk::T.deref(),
        "fa" => fa::T.deref(),
        "fi" => fi::T.deref(),
        "ca" => ca::T.deref(),
        "el" => el::T.deref(),
        "sv" => sv::T.deref(),
        "sq" => sq::T.deref(),
        "sr" => sr::T.deref(),
        "th" => th::T.deref(),
        "sl" => sl::T.deref(),
        "ro" => ro::T.deref(),
        "lt" => lt::T.deref(),
        "lv" => lv::T.deref(),
        "ar" => ar::T.deref(),
        "bg" => bg::T.deref(),
        "be" => be::T.deref(),
        "he" => he::T.deref(),
        "hr" => hr::T.deref(),
        "sc" => sc::T.deref(),
        "ta" => ta::T.deref(),
        "ge" => ge::T.deref(),
        _ => en::T.deref(),
    };
    let (name, placeholder_value) = extract_placeholder(&name);
    let replace = |s: &&str| {
        let mut s = s.to_string();
        if let Some(value) = placeholder_value.as_ref() {
            s = s.replace("{}", &value);
        }
        if !crate::is_rustdesk() {
            if s.contains("RustDesk")
                && !name.starts_with("upgrade_rustdesk_server_pro")
                && name != "powered_by_me"
            {
                let app_name = crate::get_app_name();
                if !app_name.contains("RustDesk") {
                    s = s.replace("RustDesk", &app_name);
                } else {
                    // https://github.com/rustdesk/rustdesk-server-pro/issues/845
                    // If app_name contains "RustDesk" (e.g., "RustDesk-Admin"), we need to avoid
                    // replacing "RustDesk" within the already-substituted app_name, which would
                    // cause duplication like "RustDesk-Admin" -> "RustDesk-Admin-Admin".
                    //
                    // app_name only contains alphanumeric and hyphen.
                    const PLACEHOLDER: &str = "#A-P-P-N-A-M-E#";
                    if !s.contains(PLACEHOLDER) {
                        s = s.replace(&app_name, PLACEHOLDER);
                        s = s.replace("RustDesk", &app_name);
                        s = s.replace(PLACEHOLDER, &app_name);
                    } else {
                        // It's very unlikely to reach here.
                        // Skip replacement to avoid incorrect result.
                    }
                }
            }
        }
        s
    };
    if let Some(v) = m.get(&name as &str) {
        if !v.is_empty() {
            return replace(v);
        }
    }
    if lang != "en" {
        if let Some(v) = en::T.get(&name as &str) {
            if !v.is_empty() {
                return replace(v);
            }
        }
    }
    replace(&name.as_str())
}

// Matching pattern is {}
// Write {value} in the UI and {} in the translation file
//
// Example:
// Write in the UI: translate("There are {24} hours in a day")
// Write in the translation file: ("There are {} hours in a day", "{} hours make up a day")
fn extract_placeholder(input: &str) -> (String, Option<String>) {
    if let Ok(re) = Regex::new(r#"\{(.*?)\}"#) {
        if let Some(captures) = re.captures(input) {
            if let Some(inner_match) = captures.get(1) {
                let name = re.replace(input, "{}").to_string();
                let value = inner_match.as_str().to_string();
                return (name, Some(value));
            }
        }
    }
    (input.to_string(), None)
}

mod test {
    #[test]
    fn test_extract_placeholders() {
        use super::extract_placeholder as f;

        assert_eq!(f(""), ("".to_string(), None));
        assert_eq!(
            f("{3} sessions"),
            ("{} sessions".to_string(), Some("3".to_string()))
        );
        assert_eq!(f(" } { "), (" } { ".to_string(), None));
        // Allow empty value
        assert_eq!(
            f("{} sessions"),
            ("{} sessions".to_string(), Some("".to_string()))
        );
        // Match only the first one
        assert_eq!(
            f("{2} times {4} makes {8}"),
            ("{} times {4} makes {8}".to_string(), Some("2".to_string()))
        );
    }
}

