use rexiv2;
use std::convert::From;
use std::error;
use std::fmt;
use std::io;


#[derive(Debug)]
pub enum SyncError {
    Io(io::Error),
    Exif(rexiv2::Rexiv2Error),
    NotExifFormat,
}


impl error::Error for SyncError {
    fn description(&self) -> &str {
        match *self {
            SyncError::Io(ref e)      => e.description(),
            SyncError::Exif(ref e)    => e.description(),
            SyncError::NotExifFormat  => "This format does not support exif."
        }
    }


    fn cause(&self) -> Option<&error::Error> {
        match *self {
            SyncError::Io(ref e)     => Some(e),
            SyncError::Exif(ref e)   => Some(e),
            SyncError::NotExifFormat => None,
        }
    }
}


impl fmt::Display for SyncError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SyncError::Io(ref e)     => write!(f, "{}", e),
            SyncError::Exif(ref e)   => write!(f, "{}", e),
            SyncError::NotExifFormat => write!(f, "{}", (self as &error::Error).description()),
        }
    }
}


impl From<io::Error> for SyncError {
    fn from(error: io::Error) -> SyncError
    {
        SyncError::Io(error)
    }
}


impl From<rexiv2::Rexiv2Error> for SyncError {
    fn from(error: rexiv2::Rexiv2Error) -> SyncError
    {
        SyncError::Exif(error)
    }
}
