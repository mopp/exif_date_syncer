extern crate rexiv2;
extern crate chrono;
extern crate filetime;


mod sync_error;


use chrono::DateTime;
use chrono::TimeZone;
use chrono::Timelike;
use filetime::FileTime;
use std::cmp;
use std::env;
use std::fs;
use sync_error::SyncError;


static TARGET_EXIF_TAGS:[&'static str; 3]  = [
    "Exif.Image.DateTime",          // The date and time of image creation. In Exif standard, it is the date and time the file was changed.
    "Exif.Photo.DateTimeOriginal",  // The date and time when the original image data was generated. For a digital still camera the date and time the picture was taken are recorded.
    "Exif.Photo.DateTimeDigitized", // The date and time when the image was stored as digital data.
];

static EXIF_DATE_FORMAT: &'static str = "%Y:%m:%d %H:%M:%S";



fn main()
{
    for filepath in env::args().skip(1) {
        println!("{}", filepath);
        match sync_date(filepath.as_str()) {
            Ok(written_date_str) => {
                println!("  {}", written_date_str);
            },
            Err(e) => {
                println!("  {}", e);
            }
        }
    }
}


fn sync_date(filepath: &str) -> Result<String, SyncError>
{
    let metadata_fs   = try!(fs::metadata(filepath));
    let metadata_exif = try!(rexiv2::Metadata::new_from_path(filepath));

    if metadata_exif.supports_exif() == false {
        return Err(SyncError::NotExifFormat);
    }

    // Obtain the data struct.
    let mtime             = FileTime::from_last_modification_time(&metadata_fs);
    let secs              = mtime.seconds_relative_to_1970();
    let nanos             = mtime.nanoseconds();
    let datetime_modified = chrono::Local.timestamp(secs as i64, nanos);

    // Find the oldest datetime of the image file.
    let oldest_datetime: DateTime<chrono::Local> =
        match metadata_exif.has_exif() {
            true  => {
                let datetimes = TARGET_EXIF_TAGS.iter()
                    .map(|tag| metadata_exif.get_tag_string(tag).ok())
                    .flat_map(|x| x.and_then(|date_str| chrono::Local.datetime_from_str(date_str.as_str(), EXIF_DATE_FORMAT).ok()))
                    .collect::<Vec<_>>();

                cmp::min(datetimes.into_iter().min(), Some(datetime_modified)).unwrap_or(datetime_modified)
            },
            false => {
                // No exif metadata.
                // Then, the modified date is written into the exif fields.
                datetime_modified
            },
        };

    // Set exif file tags.
    let datetime_str    = oldest_datetime.format(EXIF_DATE_FORMAT).to_string();
    for tag in TARGET_EXIF_TAGS.iter() {
        try!(metadata_exif.set_tag_string(tag, datetime_str.as_str()));
    }
    try!(metadata_exif.save_to_file(filepath));

    // Set the last access and modification times of the file.
    let ft = FileTime::from_seconds_since_1970(oldest_datetime.timestamp() as u64, oldest_datetime.nanosecond());
    try!(filetime::set_file_times(filepath, ft, ft));


    Ok(datetime_str)
}
