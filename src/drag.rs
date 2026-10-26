use std::path::PathBuf;

use gtk::{gdk, gio, glib};
use gtk::gio::prelude::*;
use gtk::glib::value::ToValue;

const URI_LIST_MIME: &str = "text/uri-list";

/// Build a content provider that offers both GDK's native file-list value and
/// an explicit text/uri-list representation for broad interoperability.
pub fn provider_for_paths(paths: &[PathBuf]) -> gdk::ContentProvider {
    let files: Vec<gio::File> = paths.iter().map(|path| gio::File::for_path(path)).collect();

    let file_list = gdk::FileList::from_array(&files);
    let native = gdk::ContentProvider::for_value(&file_list.to_value());

    let uri_list = files
        .iter()
        .map(|file| file.uri().to_string())
        .collect::<Vec<_>>()
        .join("\r\n")
        + "\r\n";
    let bytes = glib::Bytes::from(uri_list.as_bytes());
    let uri_provider = gdk::ContentProvider::for_bytes(URI_LIST_MIME, &bytes);

    gdk::ContentProvider::new_union(&[native, uri_provider])
}
