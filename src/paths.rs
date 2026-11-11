//! Diffing closures given as store paths or snapshot files.

use std::{
  cell::LazyCell,
  fs,
  io::{
    self,
    BufRead as _,
  },
  path::Path,
};

use misstep::ResultExt as _;

use crate::{
  DiffReport,
  SnapshotDocument,
  Store,
  diff_store_snapshots,
};

/// Diffs `old_path` and `new_path`, each a store path or snapshot file.
///
/// # Returns
///
/// The diff from `old_path` to `new_path`.
///
/// # Errors
///
/// Returns an error if a path cannot be loaded as a store path or snapshot.
pub fn diff_paths(
  old_path: &Path,
  new_path: &Path,
) -> misstep::Result<DiffReport> {
  // Only opened if an argument is not a snapshot file.
  let store: LazyCell<Store> = LazyCell::new(Store::open);
  let old = load_snapshot(old_path, &store)?;
  let new = load_snapshot(new_path, &store)?;
  Ok(diff_store_snapshots(&old.snapshot, &new.snapshot))
}

/// Reads the snapshot in `path` if it is a JSON file, and queries the store
/// path `path` resolves to in `store` otherwise.
///
/// # Returns
///
/// The snapshot read from, or queried for, `path`.
///
/// # Errors
///
/// Returns an error if `path` is a JSON file that is not a valid snapshot, or
/// the store cannot be queried.
fn load_snapshot(
  path: &Path,
  store: &LazyCell<Store>,
) -> misstep::Result<SnapshotDocument> {
  if !is_json_file(path)? {
    return store.query_snapshot_document(path);
  }

  tracing::debug!(path = %path.display(), "reading snapshot");
  let input = fs::read_to_string(path)?;
  SnapshotDocument::from_json(&input)
    .with_context(|| format!("failed to read snapshot '{}'", path.display()))
}

/// Checks whether `path` is a file holding a JSON object, by its first
/// non-whitespace byte.
fn is_json_file(path: &Path) -> io::Result<bool> {
  if !path.is_file() {
    return Ok(false);
  }
  let mut reader = io::BufReader::new(fs::File::open(path)?);
  Ok(reader.fill_buf()?.trim_ascii_start().starts_with(b"{"))
}
