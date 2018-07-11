# System Syzygy

*System Syzygy* is a puzzle game, in the style of Cliff Johnson's
[*The Fool's Errand*](http://fools-errand.com/02-FE/index.htm) and
[*3 in Three*](http://www.fools-errand.com/04-3T/index.htm), and of Andrew
Plotkin's [*System's Twilight*](http://www.eblong.com/zarf/twilight.html).

https://mdsteele.games/syzygy/

## License

*System Syzygy* is licensed under the GNU GPL, version 3.  *System Syzygy* is
free software: you can redistribute it and/or modify it under the terms of the
GNU General Public License as published by the Free Software Foundation, either
version 3 of the License, or (at your option) any later version.

*System Syzygy* is distributed in the hope that it will be useful, but WITHOUT
ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
FOR A PARTICULAR PURPOSE.  See the GNU General Public License for more details.

The complete license can be found in the LICENSE file.

## Building a release

### Mac OS X

1. Install the [SDL2 framework](https://www.libsdl.org/download-2.0.php) under
   `/Library/Frameworks/`.
1. Install [`cargo-bundle`](https://github.com/burtonageo/cargo-bundle) from
   head (version 0.3.0 as published on crates.io is still missing some needed
   bugfixes).
1. Run `cargo bundle --release`.
1. Fix the binary's rpath:

    ```shell
    $ install_name_tool -add_rpath "@executable_path/../Frameworks" \
        target/release/bundle/osx/System\ Syzygy.app/Contents/MacOS/syzygy
    ```

1. Add the SDL2 framework to the app bundle:

    ```shell
    $ mkdir target/release/bundle/osx/System\ Syzygy.app/Contents/Frameworks
    $ cp -R /Library/Frameworks/SDL2.framework \
        target/release/bundle/osx/System\ Syzygy.app/Contents/Frameworks/
    ```

1. Strip the binary:

    ```shell
    $ strip target/release/bundle/osx/System\ Syzygy.app/Contents/MacOS/syzygy
    ```

1. (Optional) Sign the app bundle:

   ```shell
   $ codesign --deep --verbose --sign "<identity>" \
       target/release/bundle/osx/System\ Syzygy.app
   $ codesign --verify -vvvv target/release/bundle/osx/System\ Syzygy.app
   ```

### Debian Linux

1. Install SDL2:

    ```shell
    $ sudo apt-get install libsdl2-dev
    ```

1. Install [`cargo-bundle`](https://github.com/burtonageo/cargo-bundle) from
   head (version 0.3.0 as published on crates.io is still missing some needed
   bugfixes).
1. Run `cargo build --release`.
1. Strip the binary:

    ```shell
    $ strip target/release/syzygy
    ```

4. Run `cargo bundle --release --format=deb`.
