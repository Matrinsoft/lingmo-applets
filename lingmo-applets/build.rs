use std::fs;
use xdgen::{App, Context, FluentString};

fn main() {
    let ctx = Context::new("../i18n/", "desktop_entries").unwrap();

    [
        (
            "com.lingmoos.LingmoAppList",
            "lingmo-app-list",
            "lingmo-app-list-comment",
            "lingmo-app-list-keywords",
        ),
        (
            "com.lingmoos.LingmoAppletA11y",
            "lingmo-applet-a11y",
            "lingmo-applet-a11y-comment",
            "lingmo-applet-a11y-keywords",
        ),
        (
            "com.lingmoos.LingmoAppletAudio",
            "lingmo-applet-audio",
            "lingmo-applet-audio-comment",
            "lingmo-applet-audio-keywords",
        ),
        (
            "com.lingmoos.LingmoAppletBattery",
            "lingmo-applet-battery",
            "lingmo-applet-battery-comment",
            "lingmo-applet-battery-keywords",
        ),
        (
            "com.lingmoos.LingmoAppletBluetooth",
            "lingmo-applet-bluetooth",
            "lingmo-applet-bluetooth-comment",
            "lingmo-applet-bluetooth-keywords",
        ),
        (
            "com.lingmoos.LingmoAppletInputSources",
            "lingmo-applet-input-sources",
            "lingmo-applet-input-sources-comment",
            "lingmo-applet-input-sources-keywords",
        ),
        (
            "com.lingmoos.LingmoAppletMinimize",
            "lingmo-applet-minimize",
            "lingmo-applet-minimize-comment",
            "lingmo-applet-minimize-keywords",
        ),
        (
            "com.lingmoos.LingmoAppletNetwork",
            "lingmo-applet-network",
            "lingmo-applet-network-comment",
            "lingmo-applet-network-keywords",
        ),
        (
            "com.lingmoos.LingmoAppletNotifications",
            "lingmo-applet-notifications",
            "lingmo-applet-notifications-comment",
            "lingmo-applet-notifications-keywords",
        ),
        (
            "com.lingmoos.LingmoAppletPower",
            "lingmo-applet-power",
            "lingmo-applet-power-comment",
            "lingmo-applet-power-keywords",
        ),
        (
            "com.lingmoos.LingmoAppletStatusArea",
            "lingmo-applet-status-area",
            "lingmo-applet-status-area-comment",
            "lingmo-applet-status-area-keywords",
        ),
        (
            "com.lingmoos.LingmoAppletTiling",
            "lingmo-applet-tiling",
            "lingmo-applet-tiling-comment",
            "lingmo-applet-tiling-keywords",
        ),
        (
            "com.lingmoos.LingmoAppletTime",
            "lingmo-applet-time",
            "lingmo-applet-time-comment",
            "lingmo-applet-time-keywords",
        ),
        (
            "com.lingmoos.LingmoAppletWorkspaces",
            "lingmo-applet-workspaces",
            "lingmo-applet-workspaces-comment",
            "lingmo-applet-workspaces-keywords",
        ),
        (
            "com.lingmoos.LingmoPanelAppButton",
            "lingmo-panel-app-button",
            "lingmo-panel-app-button-comment",
            "lingmo-panel-app-button-keywords",
        ),
        (
            "com.lingmoos.LingmoPanelLauncherButton",
            "lingmo-panel-launcher-button",
            "lingmo-panel-launcher-button-comment",
            "lingmo-panel-launcher-button-keywords",
        ),
        (
            "com.lingmoos.LingmoPanelWorkspacesButton",
            "lingmo-panel-workspaces-button",
            "lingmo-panel-workspaces-button-comment",
            "lingmo-panel-workspaces-button-keywords",
        ),
    ]
    .into_iter()
    .map(|(id, name, comment, keywords)| {
        let template_path = ["../", name, "/data/", id, ".desktop"].concat();

        let app = App::new(FluentString(name))
            .comment(FluentString(comment))
            .keywords(FluentString(keywords));

        (id, app.expand_desktop(&template_path, &ctx).unwrap())
    })
    .for_each(|(id, contents)| {
        let parent = "../target/xdgen/";
        fs::create_dir_all(parent).unwrap();
        fs::write([parent, id, ".desktop"].concat().as_str(), contents).unwrap();
    });
}

