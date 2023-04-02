use fatfs;
use core2;
use derive_more::{From, Into};

/// An adapter (wrapper type) that implements traits required by the [`fatfs`] crate
/// for any I/O device that wants to be usable by [`fatfs`].
///
/// To meet [`fatfs`]'s requirements, the underlying I/O stream must be able to 
/// read, write, and seek while tracking its current offset. 
/// We use traits from the [`core2`] crate to meet these requirements, 
/// thus, the given `IO` parameter must implement those [`core2`] traits.
///
/// For example, this allows one to access a FAT filesystem 
/// by reading from or writing to a storage device.
pub struct FatFsAdapter<IO>(IO);
impl<IO> FatFsAdapter<IO> {
    pub fn new(io: IO) -> FatFsAdapter<IO> { FatFsAdapter(io) }
}
/// This tells the `fatfs` crate that our read/write/seek functions
/// may return errors of the type [`FatFsIoErrorAdapter`],
/// which is a simple wrapper around [`core2::io::Error`].
impl<IO> fatfs::IoBase for FatFsAdapter<IO> {
    type Error = FatFsIoErrorAdapter;
}

impl<IO> fatfs::Read for FatFsAdapter<IO> where IO: core2::io::Read {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        self.0.read(buf).map_err(Into::into)
    }
}

impl<IO> fatfs::Write for FatFsAdapter<IO> where IO: core2::io::Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        self.0.write(buf).map_err(Into::into)
    }
    fn flush(&mut self) -> Result<(), Self::Error> {
        self.0.flush().map_err(Into::into)
    }
}
impl<IO> fatfs::Seek for FatFsAdapter<IO> where IO: core2::io::Seek {
    fn seek(&mut self, pos: fatfs::SeekFrom) -> Result<u64, Self::Error> {
        let core2_pos = match pos {
            fatfs::SeekFrom::Start(s)   => core2::io::SeekFrom::Start(s),
            fatfs::SeekFrom::Current(c) => core2::io::SeekFrom::Current(c),
            fatfs::SeekFrom::End(e)     => core2::io::SeekFrom::End(e),
        };
        self.0.seek(core2_pos).map_err(Into::into)
    }
}

/// This struct exists to enable us to implement the [`fatfs::IoError`] trait
/// for the [`core2::io::Error`] trait.
/// 
/// This is required because Rust prevents implementing foreign traits for foreign types.
#[derive(Debug, From, Into)]
pub struct FatFsIoErrorAdapter(core2::io::Error);
impl fatfs::IoError for FatFsIoErrorAdapter {
    fn is_interrupted(&self) -> bool {
        self.0.kind() == core2::io::ErrorKind::Interrupted
    }
    fn new_unexpected_eof_error() -> Self {
        FatFsIoErrorAdapter(core2::io::ErrorKind::UnexpectedEof.into())
    }
    fn new_write_zero_error() -> Self {
        FatFsIoErrorAdapter(core2::io::ErrorKind::WriteZero.into())
    }
}