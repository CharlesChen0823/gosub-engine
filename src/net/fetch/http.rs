/// While fetching encompasses more than just HTTP, it borrows a number of concepts from HTTP and applies these to resources obtained via other means (e.g., data URLs).
///
/// An HTTP tab or space is U+0009 TAB or U+0020 SPACE.
///
/// HTTP whitespace is U+000A LF, U+000D CR, or an HTTP tab or space.
///
/// HTTP whitespace is only useful for specific constructs that are reused outside the context of HTTP headers (e.g., MIME types). For HTTP header values, using HTTP tab or space is preferred, and outside that context ASCII whitespace is preferred. Unlike ASCII whitespace this excludes U+000C FF.
///
/// An HTTP newline byte is 0x0A (LF) or 0x0D (CR).
///
/// An HTTP tab or space byte is 0x09 (HT) or 0x20 (SP).
///
/// An HTTP whitespace byte is an HTTP newline byte or HTTP tab or space byte.
///
/// To collect an HTTP quoted string from a string input, given a position variable position and an optional boolean extract-value (default false):
///
/// Let positionStart be position.
///
/// Let value be the empty string.
///
/// Assert: the code point at position within input is U+0022 (").
///
/// Advance position by 1.
///
/// While true:
///
/// Append the result of collecting a sequence of code points that are not U+0022 (") or U+005C (\) from input, given position, to value.
///
/// If position is past the end of input, then break.
///
/// Let quoteOrBackslash be the code point at position within input.
///
/// Advance position by 1.
///
/// If quoteOrBackslash is U+005C (\), then:
///
/// If position is past the end of input, then append U+005C (\) to value and break.
///
/// Append the code point at position within input to value.
///
/// Advance position by 1.
///
/// Otherwise:
///
/// Assert: quoteOrBackslash is U+0022 (").
///
/// Break.
///
/// If extract-value is true, then return value.
///
/// Return the code points from positionStart to position, inclusive, within input.

/// A method is a byte sequence that matches the method token production.
///
/// A CORS-safelisted method is a method that is `GET`, `HEAD`, or `POST`.
///
/// A forbidden method is a method that is a byte-case-insensitive match for `CONNECT`, `TRACE`, or `TRACK`. [HTTPVERBSEC1], [HTTPVERBSEC2], [HTTPVERBSEC3]
///
/// To normalize a method, if it is a byte-case-insensitive match for `DELETE`, `GET`, `HEAD`, `OPTIONS`, `POST`, or `PUT`, byte-uppercase it.
pub enum Method {}

/// A header list is a list of zero or more headers. It is initially « ».
pub struct HeaderMap {}

impl HeaderMap {
    /// To get a structured field value given a header name name and a string type from a header list list, run these steps. They return null or a structured field value.
    ///
    /// Assert: type is one of "dictionary", "list", or "item".
    ///
    /// Let value be the result of getting name from list.
    ///
    /// If value is null, then return null.
    ///
    /// Let result be the result of parsing structured fields with input_string set to value and header_type set to type.
    ///
    /// If parsing failed, then return null.
    ///
    /// Return result.
    fn get_field_value(&self) {}

    /// To set a structured field value given a tuple (header name name, structured field value structuredValue), in a header list list:

    /// Let serializedValue be the result of executing the serializing structured fields algorithm on structuredValue.
    ///
    /// Set (name, serializedValue) in list.
    fn set_field_value(&self) {}
    /// A header list list contains a header name name if list contains a header whose name is a byte-case-insensitive match for name.

    /// To get a header name name from a header list list, run these steps. They return null or a header value.
    ///
    /// If list does not contain name, then return null.
    ///
    /// Return the values of all headers in list whose name is a byte-case-insensitive match for name, separated from each other by 0x2C 0x20, in order.
    fn get_header_name(&self) {}
    ///
    /// To get, decode, and split a header name name from header list list, run these steps. They return null or a list of strings.
    ///
    /// Let value be the result of getting name from list.
    ///
    /// If value is null, then return null.
    ///
    /// Return the result of getting, decoding, and splitting value.
    fn get_special_header_name(&self) {}
}

/// A status is an integer in the range 0 to 999, inclusive.
///
/// Various edge cases in mapping HTTP/1’s status-code to this concept are worked on in issue #1156.
///
/// A null body status is a status that is 101, 103, 204, 205, or 304.
///
/// An ok status is a status in the range 200 to 299, inclusive.
///
/// A redirect status is a status that is 301, 302, 303, 307, or 308.
pub struct Statue {}

pub struct Body {}
