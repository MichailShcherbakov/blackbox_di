use syn::{spanned::Spanned, Error, Path, Type};

pub(crate) fn get_path_from_type(type_: &Type) -> Result<Path, Error> {
    if let Type::Path(path_type) = type_ {
        Ok(path_type.path.clone())
    } else {
        Err(Error::new(type_.span(), "Unsupported type"))
    }
}
