//! Helper functions for the ONNX Runtime bindings.

/// Converts a vector of `std::ffi::CString` to a vector of raw C string pointers.
pub(crate) fn c_strings_to_pointers(
    strings: &[std::ffi::CString],
) -> Vec<*const std::os::raw::c_char> {
    strings.iter().map(|string| string.as_ptr()).collect()
}

/// Convert an iterator of string-like objects into a vector of `CString`s.
pub(crate) fn iter_to_c_strings<Iter>(strings: Iter) -> Vec<std::ffi::CString>
where
    Iter: Iterator<Item: AsRef<str>>,
{
    strings
        .map(|string| std::ffi::CString::new(string.as_ref()).unwrap())
        .collect()
}

pub(crate) fn keys_to_c_strings<T: AsRef<str>, U>(strings: &[(T, U)]) -> Vec<std::ffi::CString> {
    iter_to_c_strings(strings.iter().map(|(string, _)| string.as_ref()))
}

pub(crate) fn values_to_c_strings<T, U: AsRef<str>>(strings: &[(T, U)]) -> Vec<std::ffi::CString> {
    iter_to_c_strings(strings.iter().map(|(_, string)| string.as_ref()))
}
