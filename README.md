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

## Building and running locally

To compile and run the game without having to build a release package, run:

```shell
$ cargo run --release
```

(Building/running in release mode is highly recommended for playing the game;
the non-release build enables some otherwise-distracting features intended for
development/debugging, and also there's one particular late-game puzzle that
runs quite slowly in debug mode.)

Note that invoking the built binary directly via `./target/release/syzygy` does
not work on some platforms, because it confuses the game's resource-loading
code.  Always run via `cargo` or from an installed release package.

## Building a release package

### Mac OS X

1. Install the [SDL2 framework](https://www.libsdl.org/download-2.0.php) under
   `/Library/Frameworks/`.
1. Install [`cargo-bundle`](https://crates.io/crates/cargo-bundle) (version
   0.4.0 or later).
1. Run `cargo bundle --release`.
1. Fix the binary's rpath:

    ```shell
    $ install_name_tool -add_rpath "@executable_path/../Frameworks" \
        target/release/bundle/osx/System\ Syzygy.app/Contents/MacOS/syzygy
    ```

1. Strip the binary:

    ```shell
    $ strip target/release/bundle/osx/System\ Syzygy.app/Contents/MacOS/syzygy
    ```

1. (Optional) Remove i386 architecture from the bundled SDL2 framework:

   ```shell
   $ BUNDLE="target/release/bundle/osx/System Syzygy.app"; \
       FILE="${BUNDLE}/Contents/Frameworks/SDL2.framework/Versions/A/SDL2"; \
       lipo "${FILE}" -remove i386 -output "${FILE}"
   ```

1. (Optional) Sign the app bundle:

   ```shell
   $ codesign --deep --force --verbose \
       --sign "3rd Party Mac Developer Application" \
       --entitlements "data/osx/entitlements.plist" \
       target/release/bundle/osx/System\ Syzygy.app
   $ codesign --verify -vvvv target/release/bundle/osx/System\ Syzygy.app
   ```

1. (Optional) Create signed installer package:

   ```shell
   $ productbuild --component "target/release/bundle/osx/System Syzygy.app" \
       /Applications --sign "3rd Party Mac Developer Installer" \
       "target/release/bundle/osx/System Syzygy.pkg"
   ```

### Debian Linux

1. Install SDL2:

    ```shell
    $ sudo apt-get install libsdl2-dev
    ```

1. Install [`cargo-bundle`](https://crates.io/crates/cargo-bundle) (version
   0.4.0 or later).
1. Run `cargo build --release`.
1. Strip the binary:

    ```shell
    $ strip target/release/syzygy
    ```

4. Run `cargo bundle --release --format=deb`.

### Windows

1. Follow
   [these instructions](https://github.com/Rust-SDL2/rust-sdl2#windows-msvc)
   for installing SDL2 where Rust can find it.
1. Run `cargo build --release`.
1. Copy `SDL2.dll` to `target/release/SDL2.dll`.
1. To create the zip archive, open `target/release/` in Windows Explorer,
   select both `syzygy.exe` and `SDL2.dll`, right-click, and select "Send to >
   Compressed (zipped) folder".
