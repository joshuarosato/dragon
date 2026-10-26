use std::path::{Path, PathBuf};

use gtk::{gdk, gio};
use gtk::prelude::*;

use crate::drag;

const UI: &str = include_str!("../data/window.ui");
const CSS: &str = include_str!("../data/style.css");

pub fn build_window(app: &adw::Application, paths: &[PathBuf], keep_open: bool) {
    install_css();

    let builder = gtk::Builder::from_string(UI);
    let window: adw::ApplicationWindow = builder
        .object("window")
        .expect("window.ui is missing #window");
    let card: gtk::Box = builder
        .object("drag_card")
        .expect("window.ui is missing #drag_card");
    let icon: gtk::Image = builder
        .object("file_icon")
        .expect("window.ui is missing #file_icon");
    let title: gtk::Label = builder
        .object("title_label")
        .expect("window.ui is missing #title_label");
    let subtitle: gtk::Label = builder
        .object("subtitle_label")
        .expect("window.ui is missing #subtitle_label");

    window.set_application(Some(app));

    populate_file_summary(paths, &icon, &title, &subtitle);
    card.set_tooltip_text(Some(&tooltip_for_paths(paths)));
    card.set_cursor_from_name(Some("grab"));

    let provider = drag::provider_for_paths(paths);
    let source = gtk::DragSource::new();
    source.set_actions(gdk::DragAction::COPY);
    source.set_content(Some(&provider));

    if !keep_open {
        let window_weak = window.downgrade();
        source.connect_drag_begin(move |_source, drag| {
            let window_weak = window_weak.clone();
            drag.connect_dnd_finished(move |_drag| {
                if let Some(window) = window_weak.upgrade() {
                    window.close();
                }
            });
        });
    }

    card.add_controller(source);

    let keys = gtk::EventControllerKey::new();
    let window_weak = window.downgrade();
    keys.connect_key_pressed(move |_controller, key, _keycode, _state| {
        if key == gdk::Key::Escape {
            if let Some(window) = window_weak.upgrade() {
                window.close();
            }
            gtk::glib::Propagation::Stop
        } else {
            gtk::glib::Propagation::Proceed
        }
    });
    window.add_controller(keys);

    window.present();
}

fn install_css() {
    let provider = gtk::CssProvider::new();
    provider.load_from_data(CSS);

    if let Some(display) = gdk::Display::default() {
        gtk::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}

fn populate_file_summary(
    paths: &[PathBuf],
    icon: &gtk::Image,
    title: &gtk::Label,
    subtitle: &gtk::Label,
) {
    if paths.len() == 1 {
        let path = &paths[0];
        let file = gio::File::for_path(path);

        if let Ok(info) = file.query_info(
            "standard::display-name,standard::icon,standard::size",
            gio::FileQueryInfoFlags::NONE,
            gio::Cancellable::NONE,
        ) {
            title.set_label(info.display_name().as_str());
            if let Some(gicon) = info.icon() {
                icon.set_from_gicon(&gicon);
            }

            let parent = path
                .parent()
                .map(|p| p.display().to_string())
                .unwrap_or_default();
            let size = human_size(info.size().max(0) as u64);
            subtitle.set_label(&format!("{size}  •  {parent}"));
            return;
        }

        title.set_label(&display_name(path));
        subtitle.set_label(
            &path
                .parent()
                .map(|p| p.display().to_string())
                .unwrap_or_default(),
        );
        return;
    }

    icon.set_icon_name(Some("folder-documents-symbolic"));
    title.set_label(&format!("{} files", paths.len()));

    let preview = paths
        .iter()
        .take(3)
        .map(|path| display_name(path))
        .collect::<Vec<_>>()
        .join(", ");
    let extra = paths.len().saturating_sub(3);
    if extra > 0 {
        subtitle.set_label(&format!("{preview} + {extra} more"));
    } else {
        subtitle.set_label(&preview);
    }
}

fn display_name(path: &Path) -> String {
    path.file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.display().to_string())
}

fn tooltip_for_paths(paths: &[PathBuf]) -> String {
    paths
        .iter()
        .map(|path| path.display().to_string())
        .collect::<Vec<_>>()
        .join("\n")
}

fn human_size(bytes: u64) -> String {
    const UNITS: [&str; 5] = ["B", "KiB", "MiB", "GiB", "TiB"];

    if bytes < 1024 {
        return format!("{bytes} B");
    }

    let mut value = bytes as f64;
    let mut unit = 0usize;
    while value >= 1024.0 && unit < UNITS.len() - 1 {
        value /= 1024.0;
        unit += 1;
    }

    if value >= 10.0 {
        format!("{value:.0} {}", UNITS[unit])
    } else {
        format!("{value:.1} {}", UNITS[unit])
    }
}
