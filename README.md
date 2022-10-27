# JSON

This is a simple command line JSON parser that reads JSON from standard input
and prints it out with agreeable formatting. The following snippet demonstrates
usage of the tool.

```shell
$ echo '{ "hello": "world", "example": [1, 2, 3, true, false, null] }' | json
{
  "hello": "world",
  "example": [
    1,
    2,
    3,
    true,
    false,
    null
  ]
}
```

## Installation

The tool can be downloaded from the [Releases](https://github.com/msmoiz/json/releases)
page. It can also be installed from source using the Rust toolchain, available
[here](https://www.rust-lang.org/tools/install). Once the Rust toolchain is
set up, run the following command from within the root directory of this
project.

```shell
cargo install --path .
```

## Limitations

The parsing functionality is not available as a standalone library, which makes
it of little use to other software for the time being. In addition, the parsing
is done using unoptimized recursion which will lead to stack overflow in the
case of oversized input. Further, the JSON value construct that is returned by
the parser is traversable but otherwise not particularly ergonomic to use.
