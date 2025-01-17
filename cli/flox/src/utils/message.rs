use std::fmt::Display;
use std::io::Write;

use flox_rust_sdk::models::manifest::PackageToInstall;
/// Write a message to stderr.
///
/// This is a wrapper around `eprintln!` that can be further extended
/// to include logging, word wrapping, ANSI filtereing etc.
fn print_message(v: impl Display) {
    eprintln!("{v}");
}

fn print_message_to_buffer(out: &mut impl Write, v: impl Display) {
    writeln!(out, "{v}").unwrap();
}

/// alias for [print_message]
pub(crate) fn plain(v: impl Display) {
    print_message(v);
}
pub(crate) fn error(v: impl Display) {
    print_message(std::format_args!("❌ ERROR: {v}"));
}
pub(crate) fn created(v: impl Display) {
    print_message(std::format_args!("✨ {v}"));
}
/// double width character, add an additional space for alignment
pub(crate) fn deleted(v: impl Display) {
    print_message(std::format_args!("🗑️  {v}"));
}
pub(crate) fn updated(v: impl Display) {
    print_message(std::format_args!("✅ {v}"));
}
/// double width character, add an additional space for alignment
pub(crate) fn warning(v: impl Display) {
    print_message(std::format_args!("⚠️  {v}"));
}

/// double width character, add an additional space for alignment
pub(crate) fn warning_to_buffer(out: &mut impl Write, v: impl Display) {
    print_message_to_buffer(out, std::format_args!("⚠️  {v}"));
}

pub(crate) fn package_installed(pkg: &PackageToInstall, environment_description: &str) {
    updated(format!(
        "'{}' installed to environment {environment_description}",
        pkg.id()
    ));
}
