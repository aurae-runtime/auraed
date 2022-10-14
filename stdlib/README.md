# The Aurae Standard Library

Read the original [whitepaper](https://docs.google.com/document/d/1dA591eipsgWeAlaSwbYNQtAQaES243IIqXPAfKhJSjU/edit#heading=h.vknhjb3d4yfc).

### Subsystem Documentation

- [Subsystem Specification](https://github.com/aurae-runtime/api/tree/main/spec#aurae-api-specification)
    - [Proposing a New Subsystem](https://github.com/aurae-runtime/api/tree/main/spec#proposing-a-new-subsystem)
    - [Requesting a Change (RFC) to an existing subsystem](https://github.com/aurae-runtime/api/tree/main/spec#requesting-a-change-rfc-to-an-existing-subsystem)

### API Convention

Generally follow [this style guide](https://developers.google.com/protocol-buffers/docs/style) in the proto files.

It is short, but the main points are:

- Files should be named `lower_snake_case.proto`
- Files should be ordered in the following manner

```proto
// AURAE LICENSE HEADER

syntax = "proto3";

package lower_snake_case_package_name;

// imports sorted alphabetically
import "path/to/dependency.proto";
import "path/to/other.proto";

// file options

// everything else

``` 
- Services should be named `UpperCamelCase` (aka PascalCase)
- Service methods should be named `UpperCamelCase`
- Messages should be named `UpperCamelCase`
- Field names, including `oneof` and extension names, should be `snake_case`
- `repeated` fields should have pluralized names
- Enums should be named `UpperCamelCase`
- Enum variants should be `SCREAMING_SNAKE_CASE`
- (Suggested) Enums should NOT be nested, and their variants should be prefixed with the enum's name
```proto
enum FooBar {
  FOO_BAR_UNSPECIFIED = 0;
  FOO_BAR_FIRST_VALUE = 1;
  FOO_BAR_SECOND_VALUE = 2;
}
``` 
- (Suggested) The zero value enum variants should have the suffix `UNSPECIFIED`

---

An additional convention that is meant to reduce the likelihood of future breaking changes and ease the creation of macros for generating code:

- rpc methods (e.g., `MyMethod`) should have dedicated request and response messages named `MyMethodRequest` and `MyMethodResponse`