# Translations

The app uses [`rust-i18n`](https://crates.io/crates/rust-i18n). Locale files are
embedded into the binary at compile time by `i18n!("locales", fallback = "en")`
in `src/main.rs`.

## Files

- `en.json` — the English/source strings. Keys are the English text itself.
- `fr.json` — French translations. Any key missing here falls back to English.

Both files use the flat `_version: 1` format:

```json
{
  "_version": 1,
  "Preview": "Aperçu"
}
```

## Adding or updating a string

1. Wrap UI text in `t!("Some English text")`, or call `crate::i18n::tr(...)`
   for dynamic strings (mechanic/trigger/targeter descriptions and enum labels).
2. Add the English key to `locales/en.json` (value = the same English text).
3. Add the translated key/value to `locales/fr.json`. If you leave it out, the
   English text is shown as a fallback.
4. Run `cargo test`. The tests verify that `en.json` and `fr.json` have the same
   keys and that every mechanic, trigger and targeter has an entry.

## Adding a language

1. Copy `locales/en.json` to `locales/<code>.json` and translate the values.
2. Add `(<code>, "<Display Name>")` to `LANGUAGES` in `src/i18n.rs`.
3. `cargo build` — the new locale file is embedded automatically.

## Notes

- Keys are matched exactly, including punctuation and line breaks. Do not
  "clean up" the English key text without updating the code that emits it.
- Short syntax values (e.g. `@PlayersInRadius`, `~onAttack`, `ADD`, `RED`) are
  intentionally not translated: they are written to the MythicMobs YAML.
- `fr.json` currently contains English placeholders for most strings; replace
  the values with French text as translations become available.
