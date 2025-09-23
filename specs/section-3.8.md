### 3.8. Type Annotation

A type annotation is a prefix to any Node Name (Section 3.2) or Value (Section 3.7) that
includes a *suggestion* of what type the value is *intended* to be treated as,
or as a *context-specific elaboration* of the more generic type the node name
indicates.

Type annotations are written as a set of `(` and `)` with a single
String (Section 3.9) in it. It may contain Whitespace after the `(` and before
the `)`, and may be separated from its target by Whitespace.

KDL does not specify any restrictions on what implementations might do with
these annotations. They are free to ignore them, or use them to make decisions
about how to interpret a value.

Additionally, the following type annotations MAY be recognized by KDL parsers
and, if used, SHOULD interpret these types as follows:

#### 3.8.1. Reserved Type Annotations for Numbers Without Decimals:

Signed integers of various sizes (the number is the bit size):

- `i8`
- `i16`
- `i32`
- `i64`
- `i128`

Unsigned integers of various sizes (the number is the bit size):

- `u8`
- `u16`
- `u32`
- `u64`
- `u128`

Platform-dependent integer types, both signed and unsigned:

- `isize`
- `usize`

#### 3.8.2. Reserved Type Annotations for Numbers With Decimals:

IEEE 754 floating point numbers, both single (32) and double (64) precision:

- `f32`
- `f64`

IEEE 754-2008 decimal floating point numbers

- `decimal64`
- `decimal128`

#### 3.8.3. Reserved Type Annotations for Strings:

- `date-time`: ISO8601 date/time format.
- `time`: "Time" section of ISO8601.
- `date`: "Date" section of ISO8601.
- `duration`: ISO8601 duration format.
- `decimal`: IEEE 754-2008 decimal string format.
- `currency`: ISO 4217 currency code.
- `country-2`: ISO 3166-1 alpha-2 country code.
- `country-3`: ISO 3166-1 alpha-3 country code.
- `country-subdivision`: ISO 3166-2 country subdivision code.
- `email`: RFC5322 email address.
- `idn-email`: RFC6531 internationalized email address.
- `hostname`: RFC1123 internet hostname (only ASCII segments)
- `idn-hostname`: RFC5890 internationalized internet hostname
(only `xn--`-prefixed ASCII "punycode" segments, or non-ASCII segments)
- `ipv4`: RFC2673 dotted-quad IPv4 address.
- `ipv6`: RFC2373 IPv6 address.
- `url`: RFC3986 URI.
- `url-reference`: RFC3986 URI Reference.
- `irl`: RFC3987 Internationalized Resource Identifier.
- `irl-reference`: RFC3987 Internationalized Resource Identifier Reference.
- `url-template`: RFC6570 URI Template.
- `uuid`: RFC4122 UUID.
- `regex`: Regular expression. Specific patterns may be implementation-dependent.
- `base64`: A Base64-encoded string, denoting arbitrary binary data.

#### 3.8.4. Examples

```kdl
node (u8)123
node prop=(regex).*
(published)date "1970-01-01"
(contributor)person name="Foo McBar"
```