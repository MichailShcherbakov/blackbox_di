pub fn get_token<T>() -> String {
    return std::any::type_name::<T>().to_string();
}
