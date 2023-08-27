# PyLychee

<!--
![PyPI](https://img.shields.io/pypi/v/pylychee)
![PyPI - Python Version](https://img.shields.io/pypi/pyversions/pylychee)
![GitHub](https://img.shields.io/github/license/jb--/pylychee)
-->

PyLychee is an unofficial Python wrapper for [Lychee](https://github.com/lycheeverse/lychee), a fast, async, stream-based link checker written in Rust. It allows you to use Lychee's functionality in your Python projects.

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
from pylychee import Lychee

# Initialize the Lychee wrapper
lychee = Lychee()

# Check links in a local file
result = lychee.check_file("README.md")

# Check links on a website
result = lychee.check_website("https://example.com")

# Check links in a remote file
result = lychee.check_remote_file("https://raw.githubusercontent.com/lycheeverse/lychee/master/README.md")

# Print the result
print(result)
```

## Credits

This Python wrapper is built on top of the fantastic [Lychee](https://github.com/lycheeverse/lychee) project. All credits for the link checking functionality go to the original authors and contributors of Lychee.

## License

PyLychee is licensed under the MIT License. See the [LICENSE](LICENSE) file for more information.

Lychee is licensed under either of

- Apache License, Version 2.0, (LICENSE-APACHE or
  https://www.apache.org/licenses/LICENSE-2.0)
- MIT license (LICENSE-MIT or https://opensource.org/licenses/MIT)

at your option.
