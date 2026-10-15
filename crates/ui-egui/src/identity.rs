//! Who this app is: the product name and where it lives. Atelier Lens is built on LightCraft
//! (storytold/lightcraft, MIT OR Apache-2.0). The code keeps its upstream names (crates, command
//! ids, the library and `.lcpreset` formats) so upstream work keeps merging cleanly; only what a person sees
//! is renamed, through the constants here and the catalog rewrite in [`rewrite`]. The places that
//! carry the name outside this crate are listed in `docs/brand.md`.

/// The product name as shown in the title bar, menus, dialogs and the start screen.
pub const PRODUCT: &str = "Atelier Lens";
/// The short name, for places where the suite name is already in view.
pub const SHORT: &str = "Lens";
/// The suite this app belongs to.
pub const SUITE: &str = "Atelier";
/// Who makes it.
pub const VENDOR: &str = "Aetherion Nation";
/// The upstream project this app is built on, as named in its own strings and catalogs.
pub const UPSTREAM: &str = "LightCraft";
/// The sibling app Photo ▸ Edit In opens: upstream calls it PhotoCraft; in Atelier it is
/// Atelier Pigment. [`rewrite`] renames it too, so the Edit In labels keep their translations.
pub const SIBLING_UPSTREAM: &str = "PhotoCraft";
/// The sibling app's product name (its macOS bundle is `Atelier Pigment.app`).
pub const SIBLING: &str = "Atelier Pigment";

/// Rewrites an upstream string so it names this product. Applied to every translation catalog
/// entry as it is read, so the UI follows the brand in every language without editing the
/// catalogs (which would make every upstream catalog change a merge conflict).
pub fn rewrite(s: &str) -> String {
    // "a LightCraft backup" reads "an Atelier Lens backup".
    s.replace(&format!(" a {UPSTREAM}"), &format!(" an {PRODUCT}")).replace(UPSTREAM, PRODUCT).replace(SIBLING_UPSTREAM, SIBLING)
}

/// Like [`rewrite`], borrowing when there is nothing to change (the common case).
pub fn rewrite_cow(s: &str) -> std::borrow::Cow<'_, str> {
    if s.contains(UPSTREAM) || s.contains(SIBLING_UPSTREAM) { std::borrow::Cow::Owned(rewrite(s)) } else { std::borrow::Cow::Borrowed(s) }
}

/// Upstream strings the brand layer retires: anything naming the upstream community and site.
/// Their UI strings are replaced by ours (`brand.tsv`), so their catalog rows are skipped
/// on load; keeping them would leave translations for strings nothing shows.
pub fn retired(s: &str) -> bool {
    ["ArtCraft", "Discord", "getartcraft", "discord.gg"].iter().any(|w| s.contains(w))
}

/// Our own UI strings, with their translations: `brand.tsv`. The catalogs are upstream
/// files, so our strings live beside them and are added to each language's catalog as it loads.
/// The file's `product` line is the rename [`rewrite`] applies; a test keeps it equal to
/// [`PRODUCT`].
const BRAND_TSV: &str = include_str!("brand.tsv");

/// `(code, source, translation)` for every line of `brand.tsv` (the `product` line included).
fn brand_lines() -> impl Iterator<Item = (&'static str, &'static str, &'static str)> {
    BRAND_TSV.lines().filter(|l| !l.trim().is_empty() && !l.starts_with('#')).filter_map(|l| {
        let mut cols = l.split('\t');
        match (cols.next(), cols.next(), cols.next(), cols.next()) {
            (Some(code), Some(src), Some(tr), None) if !src.is_empty() && !tr.is_empty() => Some((code, src, tr)),
            _ => None,
        }
    })
}

/// The brand strings for a language (English for a language the file does not cover).
pub fn strings(code: &str) -> Vec<(&'static str, &'static str)> {
    let of = |c: &str| brand_lines().filter(|(l, _, _)| *l == c).map(|(_, s, t)| (s, t)).collect::<Vec<_>>();
    let own = of(code);
    if own.is_empty() { of("en") } else { own }
}

/// One language's upstream catalog as Atelier shows it: every key and translation rewritten to
/// name this product, the retired rows dropped, and our own strings added.
pub fn apply(code: &str, catalog: std::collections::BTreeMap<String, String>) -> std::collections::BTreeMap<String, String> {
    let mut out: std::collections::BTreeMap<String, String> =
        catalog.into_iter().filter(|(key, _)| !retired(key)).map(|(key, value)| (rewrite(&key), rewrite(&value))).collect();
    for (key, value) in strings(code) {
        out.insert(key.to_string(), value.to_string());
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_upstream_name_is_rewritten_wherever_it_appears() {
        assert_eq!(rewrite("About LightCraft"), "About Atelier Lens");
        assert_eq!(rewrite("LightCraft — LightCraft"), "Atelier Lens — Atelier Lens");
        assert_eq!(rewrite("Lightroom"), "Lightroom");
        assert_eq!(rewrite("not a LightCraft backup"), "not an Atelier Lens backup");
        assert_eq!(rewrite("Edit in PhotoCraft"), "Edit in Atelier Pigment");
        assert!(matches!(rewrite_cow("PhotoCraft"), std::borrow::Cow::Owned(_)));
        assert!(retired("Join the ArtCraft Discord…") && retired("ArtCraft Website") && retired("LightCraft on getartcraft.com"));
        assert!(!retired("Join the Nation") && !retired("Save As…"));
    }

    #[test]
    fn the_names_hang_together() {
        assert!(PRODUCT.starts_with(SUITE));
        assert!(PRODUCT.ends_with(SHORT));
        assert_ne!(PRODUCT, UPSTREAM);
    }

    #[test]
    fn the_product_line_matches_the_constants() {
        let product = brand_lines().find(|(code, _, _)| *code == "product");
        assert_eq!(product, Some(("product", UPSTREAM, PRODUCT)));
    }

    #[test]
    fn every_line_parses_and_every_language_covers_the_english_set() {
        let lines = BRAND_TSV.lines().filter(|l| !l.trim().is_empty() && !l.starts_with('#')).count();
        assert_eq!(lines, brand_lines().count(), "every non-comment line of brand.tsv has three columns");
        let en: Vec<&str> = strings("en").iter().map(|(k, _)| *k).collect();
        assert!(en.len() >= 4);
        let codes: std::collections::BTreeSet<&str> = brand_lines().map(|(c, _, _)| c).filter(|c| *c != "product" && *c != "en").collect();
        assert!(codes.len() >= 9, "{codes:?}");
        for code in codes {
            let keys: Vec<&str> = strings(code).iter().map(|(k, _)| *k).collect();
            assert_eq!(keys, en, "{code}");
            for (k, v) in strings(code) {
                assert_eq!(v.matches('…').count(), k.matches('…').count(), "{code}: {k:?} -> {v:?}");
            }
        }
        assert_eq!(strings("xx"), strings("en"));
    }

    #[test]
    fn the_build_script_applies_the_same_rename_to_the_format_catalogs() {
        let build = include_str!("../build.rs");
        let rule = format!(r#"s.replace(" a {UPSTREAM}", " an {PRODUCT}").replace("{UPSTREAM}", "{PRODUCT}")"#);
        assert!(build.contains(&rule), "build.rs must carry {rule}");
    }

    #[test]
    fn catalogs_name_the_product_and_carry_our_strings() {
        let mut upstream = std::collections::BTreeMap::new();
        upstream.insert("About LightCraft".to_string(), "Über LightCraft".to_string());
        upstream.insert("Join the ArtCraft Discord…".to_string(), "Dem ArtCraft-Discord beitreten…".to_string());
        let catalog = apply("de", upstream);
        assert_eq!(catalog.get("About Atelier Lens").map(String::as_str), Some("Über Atelier Lens"));
        assert!(catalog.keys().all(|k| !retired(k) || k.contains("built on LightCraft")), "{catalog:?}");
        assert_eq!(catalog.get("Join the Nation…").map(String::as_str), Some("Der Nation beitreten…"));
    }
}
