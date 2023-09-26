# LychPy

LychPy is an unofficial Python wrapper for [Lychee](https://github.com/lycheeverse/lychee), a fast, async, stream-based link checker written in Rust. It allows you to use Lychee's functionality in your Python projects.

> :warning: **Important Notice:** This project is still in development, and not ready for usage. Please check back later.

![Screencast](assets/screencast.svg)

## Table of Contents

- [Installation](#installation)
- [Usage](#usage)
- [Credits](#credits)
- [License](#license)

## Installation

```bash
pip install lychpy
```

## Usage

```python
Python 3.7.17 (default, Jun 14 2023, 09:26:35) 
[GCC 9.3.1 20200408 (Red Hat 9.3.1-2)] on linux
Type "help", "copyright", "credits" or "license" for more information.
>>> import lychpy
>>> lychpy.check(["https://bing.com", "https://google.com"])
{'https://bing.com': ✔ [200] https://bing.com/, 'https://google.com': ✔ [200] https://google.com/}
>>> url, result = lychpy.check(["https://bing.com"]).popitem()
>>> result.is_excluded
False
>>> result.is_failure
False
>>> result.is_success
True
>>> result.is_timeout
False
>>> result.is_unsupported
False
>>> result.status
'200'
>>> result.url
'https://bing.com/'
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
