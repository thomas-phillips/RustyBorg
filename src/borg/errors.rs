use super::super::util;
use borgbackup::errors::CreateError;

#[derive(Debug)]
pub enum ArchiveError {
    EpochTimeError,
    ArchiveCreateError(CreateError),
}

pub fn parse_archive_error(archive_error: ArchiveError) {
    match archive_error {
        ArchiveError::EpochTimeError => {
            util::log_print("Error retriving SystemTime since 1970!", util::LogLevel::Info)
        }
        ArchiveError::ArchiveCreateError(create_err) => {
            util::log_print(&format!("{:?}", create_err), util::LogLevel::Error)
        }
    }
}
