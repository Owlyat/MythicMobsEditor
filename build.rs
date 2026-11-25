fn main() {
    // `rust-i18n` embeds the locale files at compile time. Without these
    // directives, editing a locale file does not make Cargo rebuild the crate,
    // so the running binary keeps the old translations.
    println!("cargo:rerun-if-changed=locales");
    println!("cargo:rerun-if-changed=locales/en.json");
    println!("cargo:rerun-if-changed=locales/fr.json");
}
