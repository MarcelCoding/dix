//! Printing diffs for humans to stdout.

use std::{
  fmt::{
    self,
    Write as _,
  },
  fs,
  io::{
    self,
    Write as _,
  },
  path::Path,
};

use yansi::Paint as _;

use crate::{
  DiffReport,
  RenderOptions,
  write_diff_report,
  write_hidden_note,
};

struct WriteFmt<W: io::Write>(W);

impl<W: io::Write> fmt::Write for WriteFmt<W> {
  fn write_str(&mut self, string: &str) -> fmt::Result {
    self.0.write_all(string.as_bytes()).map_err(|_| fmt::Error)
  }
}

/// Prints `report`, the diff between `old_path` and `new_path`, for humans to
/// stdout, followed by a note on the package diffs `options` hid.
///
/// # Returns
///
/// `Ok(())` once the diff is printed.
///
/// # Errors
///
/// Returns an error if the output cannot be written.
pub fn print_diff(
  old_path: &Path,
  new_path: &Path,
  report: &DiffReport,
  options: RenderOptions,
) -> misstep::Result<()> {
  let hidden = display_diff(old_path, new_path, report, options)?;
  write_hidden_note(&mut io::stdout().lock(), hidden, "--full")?;
  Ok(())
}

/// Renders `report`, the diff between `old_path` and `new_path`, for humans to
/// stdout.
///
/// # Returns
///
/// The number of package diffs hidden by `options`.
///
/// # Errors
///
/// Returns an error if the output cannot be written.
pub fn display_diff(
  old_path: &Path,
  new_path: &Path,
  report: &DiffReport,
  options: RenderOptions,
) -> misstep::Result<usize> {
  let mut out = WriteFmt(io::stdout());

  tracing::info!("rendering diff report");

  writeln!(
    out,
    "{arrows} {old}",
    arrows = "<<<".bold(),
    old = fs::canonicalize(old_path)
      .unwrap_or_else(|_| old_path.to_path_buf())
      .display(),
  )?;
  writeln!(
    out,
    "{arrows} {new}",
    arrows = ">>>".bold(),
    new = fs::canonicalize(new_path)
      .unwrap_or_else(|_| new_path.to_path_buf())
      .display(),
  )?;

  let hidden = write_diff_report(&mut out, report, options)?;

  tracing::info!("diff report rendered");

  Ok(hidden)
}
