use crate::Wrapper;

use quick_error::quick_error;
use std::ops::{Deref, DerefMut};
use std::path::PathBuf;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Filesystem(path: PathBuf, action: &'static str, err: ::std::io::Error) {
            cause(err)
            description(err.description())
            display("error {} \"{}\": {}", action, path.display(), err)
        }

        Filesystem2(
            src: PathBuf,
            dst: PathBuf,
            action: &'static str,
            err: ::std::io::Error
        ) {
            cause(err)
            description(err.description())
            display(
                "error {} \"{}\" to \"{}\": {}",
                action,
                src.display(),
                dst.display(),
                err
            )
        }
    }
}

impl Deref for Error {
    type Target = ::std::io::Error;

    fn deref(&self) -> &Self::Target {
        match *self {
            Error::Filesystem(_, _, ref err) => err,
            Error::Filesystem2(_, _, _, ref err) => err,
        }
    }
}

impl DerefMut for Error {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match *self {
            Error::Filesystem(_, _, ref mut err) => err,
            Error::Filesystem2(_, _, _, ref mut err) => err,
        }
    }
}

impl Wrapper<::std::io::Error> for Error {
    fn into_inner(self) -> ::std::io::Error {
        match self {
            Error::Filesystem(_, _, err) => err,
            Error::Filesystem2(_, _, _, err) => err,
        }
    }
}

pub type Result<T> = ::std::result::Result<T, Error>;
