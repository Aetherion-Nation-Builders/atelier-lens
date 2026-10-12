//! The Nation and project links (Help menu, About dialog, top-bar Nation button). Command ids keep
//! their upstream names (`app.discord` opens the Nation, `app.artcraft` the Atelier suite).

/// The app's slug on the Nation site and its repository name (`atelier-lens`).
pub const APP: &str = "lens";
/// The Aetherion Nation.
pub const DISCORD: &str = "https://www.aetherionnnation.com";
/// Atelier, the Nation's creative suite.
pub const WEBSITE: &str = "https://www.aetherionnnation.com/atelier";
/// This app's page on the Nation site.
pub const APP_PAGE: &str = "https://www.aetherionnnation.com/atelier/lens";
/// This app's source repository.
pub const GITHUB: &str = "https://github.com/Aetherion-Nation-Builders/atelier-lens";

/// The user documentation (docs/ in the repository).
pub const HELP: &str = "https://github.com/Aetherion-Nation-Builders/atelier-lens/tree/main/docs";

/// (UI command id, menu label, URL) for each link, in Help-menu order.
pub const LINKS: &[(&str, &str, &str)] = &[
    ("app.help", "Atelier Lens Help", HELP),
    ("app.discord", "Join the Nation…", DISCORD),
    ("app.website", "Atelier Lens Website", APP_PAGE),
    ("app.github", "Atelier Lens on GitHub", GITHUB),
    ("app.artcraft", "Atelier Website", WEBSITE),
    ("app.feedback", "Send Feedback…", FEEDBACK),
];

/// Where feedback and bug reports go.
pub const FEEDBACK: &str = "https://github.com/Aetherion-Nation-Builders/atelier-lens/issues/new";

/// The URL behind a link command id.
pub fn url_of(cmd: &str) -> Option<&'static str> {
    LINKS.iter().find(|(id, _, _)| *id == cmd).map(|(_, _, u)| *u)
}

/// Open `url` in the user's browser (through the host's `open_url` service).
pub fn open(app: &mut crate::LightcraftApp, url: &str) -> Result<serde_json::Value, String> {
    let open = app.services.open_url.as_mut().ok_or("can't open links here")?;
    open(url)?;
    Ok(serde_json::json!({ "url": url }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn links_are_https_and_follow_the_atelier_scheme() {
        for (id, label, url) in LINKS {
            assert!(url.starts_with("https://"), "{id}");
            assert!(!label.is_empty());
            assert_eq!(url_of(id), Some(*url));
        }
        assert!(WEBSITE.starts_with(DISCORD));
        assert_eq!(APP_PAGE, format!("{WEBSITE}/{APP}"));
        assert!(GITHUB.ends_with(&format!("/Aetherion-Nation-Builders/atelier-{APP}")));
        assert!(HELP.starts_with(GITHUB) && FEEDBACK.starts_with(GITHUB));
    }
}
