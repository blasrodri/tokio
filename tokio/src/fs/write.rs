use crate::fs::asyncify;

use std::{io, path::Path};

/// Creates a future that will open a file for writing and write the entire
/// contents of `contents` to it.
///
/// This is the async equivalent of [`std::fs::write`][std].
///
/// This operation is blocking. However, tokio runs it in a
/// background thread pool dedicated to blocking operations, using
/// [`spawn_blocking`].
///
/// [`spawn_blocking`]: crate::task::spawn_blocking

///
/// [std]: fn@std::fs::write
///
/// # Examples
///
/// ```no_run
/// use tokio::fs;
///
/// # async fn dox() -> std::io::Result<()> {
/// fs::write("foo.txt", b"Hello world!").await?;
/// # Ok(())
/// # }
/// ```
pub async fn write(path: impl AsRef<Path>, contents: impl AsRef<[u8]>) -> io::Result<()> {
    let path = path.as_ref().to_owned();
    let contents = contents.as_ref().to_owned();

    asyncify(move || std::fs::write(path, contents)).await
}
