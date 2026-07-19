//! Хранение JWT-токена CLI.

use std::fs;
use std::io;
use std::path::Path;

/// Читает JWT-токен из файла.
///
/// # Errors
///
/// Возвращает ошибку, если файл существует, но его не удалось прочитать.
pub(crate) fn read_token(path: &Path) -> anyhow::Result<Option<String>> {
    match fs::read_to_string(path) {
        Ok(content) => {
            let token = content.trim().to_string();

            if token.is_empty() {
                Ok(None)
            } else {
                Ok(Some(token))
            }
        }
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(error.into()),
    }
}

/// Записывает JWT-токен в файл.
///
/// # Errors
///
/// Возвращает ошибку, если токен не удалось записать.
pub(crate) fn write_token(path: &Path, token: &str) -> anyhow::Result<()> {
    fs::write(path, token.trim())?;

    Ok(())
}
