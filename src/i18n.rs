/// Languages shipped with the app, as `(locale code, display name)`.
pub const LANGUAGES: &[(&str, &str)] = &[("en", "English"), ("fr", "Français")];

// `rust-i18n` reads the locale files in a proc macro, which Cargo does not
// always track. These `include_str!` calls make Cargo rebuild the crate when a
// locale file changes, so edits are not silently left out of the binary.
const _: &str = include_str!("../locales/en.json");
const _: &str = include_str!("../locales/fr.json");

/// Translate a runtime string using its English text as the lookup key.
///
/// `rust-i18n` falls back to the key itself when no translation exists, so
/// untranslated strings keep showing English until a translation is added.
pub fn tr(text: impl Into<String>) -> String {
    let text = text.into();
    t!(text.as_str()).to_string()
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use rust_i18n::Backend;
    use strum::{IntoEnumIterator, VariantArray};

    use crate::skill::{
        mechanics::Mechanic,
        targeters::{
            MetaEntityTarget, MetaLocationTarget, MultiLocationTarget, MultyEntity,
            SingleEntityTarget, SingleLocationTarget, Targeter, ThreatTableTarget,
        },
        triggers::Trigger,
    };

    #[test]
    fn translation_lookup_and_fallback() {
        rust_i18n::set_locale("fr");
        assert_eq!(crate::i18n::tr("__i18n_test__"), "Test français");
        assert_eq!(crate::i18n::tr("A key with no translation"), "A key with no translation");

        // A representative field tooltip (the kind shown by on_hover_ui).
        let hover = "The slot to be selected for the action. Slot starts at 0, so if a villager has 3 trades, the middle trade would be slot 1";
        assert_ne!(crate::i18n::tr(hover), hover, "hover string was not translated");

        // A multi-line hover string: these are the ones most likely to
        // mismatch because of newline/backslash escaping.
        let multiline = "Sets the base value of the mob's max health attribute.\nMythic doesn't have any limitations on max health but Spigot, however, caps the max health at 2048.\nThis can easily be changed in spigot's configuration file, server_root\\spigot.yml.\nWhenever the mob is holding or wearing an item with attribute modifiers will also affect the total max health.";
        assert_ne!(crate::i18n::tr(multiline), multiline, "multi-line hover string was not translated");

        rust_i18n::set_locale("en");
        assert_eq!(crate::i18n::tr("__i18n_test__"), "English test");
        assert_eq!(
            crate::i18n::tr("MythicMobs Skill Editor"),
            "MythicMobs Skill Editor"
        );
    }

    fn keys(locale: &str) -> HashSet<String> {
        crate::_rust_i18n_backend()
            .messages_for_locale(locale)
            .unwrap_or_default()
            .into_iter()
            .map(|(key, _)| key.into_owned())
            .collect()
    }

    #[test]
    fn english_and_french_have_the_same_keys() {
        assert_eq!(keys("en"), keys("fr"));
    }

    #[test]
    fn every_mechanic_trigger_and_targeter_has_a_key() {
        let en = keys("en");

        for mechanic in Mechanic::iter() {
            let desc: String = mechanic.get_desc().into();
            assert!(en.contains(&desc), "missing mechanic description: {desc}");
            let label: String = mechanic.get_fields().into();
            assert!(en.contains(&label), "missing mechanic label: {label}");
        }

        for trigger in Trigger::VARIANTS {
            let desc: String = trigger.get_desc().into();
            assert!(en.contains(&desc), "missing trigger description: {desc}");
            let label: String = trigger.get_fields().into();
            assert!(en.contains(&label), "missing trigger label: {label}");
        }

        for targeter in Targeter::iter() {
            let desc = targeter.get_desc();
            assert!(en.contains(desc), "missing targeter description: {desc}");
        }

        fn assert_descs<T: VariantArray>(
            en: &HashSet<String>,
            items: &[T],
            desc: impl Fn(&T) -> &'static str,
            what: &str,
        ) {
            for item in items {
                let text = desc(item);
                assert!(en.contains(text), "missing {what} description: {text}");
            }
        }

        assert_descs(&en, SingleEntityTarget::VARIANTS, |v| v.get_desc(), "single entity targeter");
        assert_descs(&en, MultyEntity::VARIANTS, |v| v.get_desc(), "multi entity targeter");
        assert_descs(&en, ThreatTableTarget::VARIANTS, |v| v.get_desc(), "threat table targeter");
        assert_descs(&en, SingleLocationTarget::VARIANTS, |v| v.get_desc(), "single location targeter");
        assert_descs(&en, MultiLocationTarget::VARIANTS, |v| v.get_desc(), "multi location targeter");
        assert_descs(&en, MetaEntityTarget::VARIANTS, |v| v.get_desc(), "meta entity targeter");
        assert_descs(&en, MetaLocationTarget::VARIANTS, |v| v.get_desc(), "meta location targeter");
    }
}

