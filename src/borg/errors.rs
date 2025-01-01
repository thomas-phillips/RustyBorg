use super::super::util::exiterr_with_message;
use borgbackup::errors::CreateError;

#[derive(Debug)]
pub enum ArchiveError {
    EpochTimeError,
    ArchiveCreateError(CreateError),
}

pub fn parse_archive_error(archive_error: ArchiveError){
    match archive_error {
        ArchiveError::EpochTimeError => {
            exiterr_with_message(1, "Error retriving SystemTime since 1970!")
        }
        ArchiveError::ArchiveCreateError(create_err) => {
            exiterr_with_message(1, &format!("{:?}", create_err))
        }
    }
}
