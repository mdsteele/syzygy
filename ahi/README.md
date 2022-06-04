# ahi

Rust library for encoding/decoding ASCII Hex Image (.ahi) and ASCII Hex Font
(.ahf) files.

## The AHI format

ASCII Hex Image (AHI) is a simple text-based format for storing collections of
small, 16-color images.  It is intended for storing sprites for games or other
graphical applications, in a way that makes changes to image files result in
(semi-)human-readable VCS diffs.

A typical .ahi file looks like this:

```text
ahi0 w20 h5 n2

0000000000000FFF0000
FFFFFFFFFFFFFF11FF00
F11111111111111111FF
FFFFFFFFFFFFFF11FF00
0000000000000FFF0000

0000FFF0000000000000
00FF11FFFFFFFFFFFFFF
FF11111111111111111F
00FF11FFFFFFFFFFFFFF
0000FFF0000000000000
```

The top-level crate documentation has more details about the format spec.

## The AHF format

ASCII Hex Font (AHF) is a variation on the AHI file format, meant for storing
16-color bitmap fonts as an ASCII text file.

A typical .ahf file looks like this:

```text
ahf0 h6 b5 n2

def w4 s5
1111
1001
1001
1001
1001
1111

'A' w5 s6
01110
10001
11111
10001
10001
00000

'g' w4 s5
0000
0111
1001
0111
0001
0110
```

The top-level crate documentation has more details about the format spec.

## License

_AHI_ is licensed under the GNU GPL, version 3.  _AHI_ is free software: you
can redistribute it and/or modify it under the terms of the GNU General Public
License as published by the Free Software Foundation, either version 3 of the
License, or (at your option) any later version.

_AHI_ is distributed in the hope that it will be useful, but WITHOUT ANY
WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A
PARTICULAR PURPOSE.  See the GNU General Public License for more details.

The complete license can be found in the LICENSE file.
