rootdir := ''
prefix := '/usr'
clean := '0'
debug := '0'
vendor := '0'
target := if debug == '1' { 'debug' } else { 'release' }
vendor_args := if vendor == '1' { '--frozen --offline' } else { '' }
debug_args := if debug == '1' { '' } else { '--release' }
cargo_args := vendor_args + ' ' + debug_args
targetdir := env('CARGO_TARGET_DIR', 'target')
sharedir := rootdir + prefix + '/share'
iconsdir := sharedir + '/icons/hicolor'
prefixdir := prefix + '/bin'
bindir := rootdir + prefixdir
libdir := rootdir + prefix + '/lib'
default-schema-target := sharedir / 'lingmo'
lingmo-applets-bin := prefixdir / 'lingmo-applets'
metainfo := 'com.lingmoos.LingmoApplets.metainfo.xml'
metainfo-src := 'data' / metainfo
metainfo-dst := clean(rootdir / prefix) / 'share' / 'metainfo' / metainfo

default: build-release

# Compiles with debug profile
build-debug *args:
    cargo build {{ args }}

# Compiles with release profile
build-release *args: (build-debug '--release' args)

# Compile with a vendored tarball
build-vendored *args:
    @just vendor-extract
    cargo build --release {{ args }} --frozen --offline

_link_applet name:
    ln -sf {{ lingmo-applets-bin }} {{ bindir }}/{{ name }}

_install_icons name:
    find {{ name }}/'data'/'icons' -type f -exec echo {} \; | rev | cut -d'/' -f-3 | rev | xargs -d '\n' -I {} install -Dm0644 {{ name }}/'data'/'icons'/{} {{ iconsdir }}/{}

_install_default_schema name:
    find {{ name }}/'data'/'default_schema' -type f -exec echo {} \; | rev | cut -d'/' -f-3 | rev | xargs -d '\n' -I {} install -Dm0644 {{ name }}/'data'/'default_schema'/{} {{ default-schema-target }}/{}

_install_desktop path:
    install -Dm0644 {{ path }} {{ sharedir }}/applications/{{ file_name(path) }}

_install_bin name:
    install -Dm0755 {{ targetdir }}/{{ target }}/{{ name }} {{ bindir }}/{{ name }}

_install_applet id name: (_install_icons name) (_install_desktop 'target/xdgen/' + id + '.desktop') (_link_applet name)

_install_button id name: (_install_icons name) (_install_desktop 'target/xdgen/' + id + '.desktop')

_install_metainfo:
    install -Dm0644 {{ metainfo-src }} {{ metainfo-dst }}

_install_status_notifier_watcher:
    sed "s|@bindir@|{{ prefixdir }}|" lingmo-applet-status-area/data/dbus-1/com.lingmoos.LingmoStatusNotifierWatcher.service.in > lingmo-applet-status-area/data/dbus-1/com.lingmoos.LingmoStatusNotifierWatcher.service
    install -Dm0644 lingmo-applet-status-area/data/dbus-1/com.lingmoos.LingmoStatusNotifierWatcher.service {{ sharedir }}/dbus-1/services/com.lingmoos.LingmoStatusNotifierWatcher.service
    sed "s|@bindir@|{{ prefixdir }}|" lingmo-applet-status-area/data/com.lingmoos.LingmoStatusNotifierWatcher.service.in > lingmo-applet-status-area/data/com.lingmoos.LingmoStatusNotifierWatcher.service
    install -Dm0644 lingmo-applet-status-area/data/com.lingmoos.LingmoStatusNotifierWatcher.service {{ libdir }}/systemd/user/com.lingmoos.LingmoStatusNotifierWatcher.service

_install_secret_agent_policy:
    install -Dm0644 lingmo-applet-network/data/dbus-1/system.d/com.lingmoos.LingmoSettings.Applet.NetworkManager.SecretAgent.conf {{ sharedir }}/dbus-1/system.d/com.lingmoos.LingmoSettings.Applet.NetworkManager.SecretAgent.conf

# Installs files into the system
install: (_install_bin 'lingmo-applets') (_link_applet 'lingmo-panel-button') (_install_applet 'com.lingmoos.LingmoAppList' 'lingmo-app-list') (_install_default_schema 'lingmo-app-list') (_install_applet 'com.lingmoos.LingmoAppletA11y' 'lingmo-applet-a11y') (_install_applet 'com.lingmoos.LingmoAppletAudio' 'lingmo-applet-audio') (_install_applet 'com.lingmoos.LingmoAppletInputSources' 'lingmo-applet-input-sources') (_install_applet 'com.lingmoos.LingmoAppletBattery' 'lingmo-applet-battery') (_install_applet 'com.lingmoos.LingmoAppletBluetooth' 'lingmo-applet-bluetooth') (_install_applet 'com.lingmoos.LingmoAppletMinimize' 'lingmo-applet-minimize') (_install_applet 'com.lingmoos.LingmoAppletNetwork' 'lingmo-applet-network') (_install_applet 'com.lingmoos.LingmoAppletNotifications' 'lingmo-applet-notifications') (_install_applet 'com.lingmoos.LingmoAppletPower' 'lingmo-applet-power') (_install_applet 'com.lingmoos.LingmoAppletStatusArea' 'lingmo-applet-status-area') (_install_applet 'com.lingmoos.LingmoAppletTiling' 'lingmo-applet-tiling') (_install_applet 'com.lingmoos.LingmoAppletTime' 'lingmo-applet-time') (_install_applet 'com.lingmoos.LingmoAppletWorkspaces' 'lingmo-applet-workspaces') (_install_button 'com.lingmoos.LingmoPanelAppButton' 'lingmo-panel-app-button') (_install_button 'com.lingmoos.LingmoPanelLauncherButton' 'lingmo-panel-launcher-button') (_install_button 'com.lingmoos.LingmoPanelWorkspacesButton' 'lingmo-panel-workspaces-button') _install_metainfo _install_status_notifier_watcher _install_secret_agent_policy

# Vendor Cargo dependencies locally
vendor:
	mkdir -p .cargo
	cargo vendor --locked 2>/dev/null | awk '/^\[/{p=1} p' > .cargo/config
	if ! grep -q 'directory' .cargo/config 2>/dev/null; then
	echo '[source.crates-io]' >> .cargo/config
	echo 'replace-with = "vendored-sources"' >> .cargo/config
	echo '' >> .cargo/config
	echo '[source.vendored-sources]' >> .cargo/config
	echo 'directory = "vendor"' >> .cargo/config
	fi
	grep '^source = "git+' Cargo.lock | sed 's/source = "//;s/"$//' | sort -u | while read src; do \
	echo "[source.\"$src\"]"; \
	echo 'replace-with = "vendored-sources"'; \
	echo ""; \
	done >> .cargo/config
	tar pcf vendor.tar vendor .cargo/config
	rm -rf vendor

# Extracts vendored dependencies
[private]
vendor-extract:
    rm -rf vendor
    tar pxf vendor.tar

# Bump cargo version, create git commit, and create tag
tag version:
    find -type f -name Cargo.toml -exec sed -i '0,/^version/s/^version.*/version = "{{ version }}"/' '{}' \; -exec git add '{}' \;
    cargo check
    cargo clean
    dch -D noble -v {{ version }}
    git add Cargo.lock debian/changelog
