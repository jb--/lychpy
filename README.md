# LychPy

LychPy is an unofficial Python wrapper for [Lychee](https://github.com/lycheeverse/lychee), a fast, async, stream-based link checker written in Rust. It allows you to use Lychee's functionality in your Python projects.

> :warning: **Important Notice:** This project is still in development, and not ready for usage. Please check back later.

## Table of Contents

- [Installation](#installation)
- [Usage](#usage)
- [Credits](#credits)
- [License](#license)

## Installation

TBD.
Please ensure that you have the Lychee binary installed on your system. Follow the [installation instructions](https://github.com/lycheeverse/lychee#installation) provided by the Lychee project.

## Usage

```python
import lychpy

result = lychpy.check(["https://example.com"])

# Print the result
print(result)
```

## Credits

This Python wrapper is built on top of the [Lychee](https://github.com/lycheeverse/lychee) project. All credits for the link checking functionality go to the original authors and contributors of Lychee.

## License

LychPy is licensed under the MIT License. See the [LICENSE](LICENSE) file for more information.

Lychee is licensed under either of

- Apache License, Version 2.0, (LICENSE-APACHE or
  https://www.apache.org/licenses/LICENSE-2.0)
- MIT license (LICENSE-MIT or https://opensource.org/licenses/MIT)

at your option.
