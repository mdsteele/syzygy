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

1. Install [`cargo-bundle`](https://github.com/burtonageo/cargo-bundle).
2. Run `cargo bundle --release`.
3. Fix the binary's rpath:

    ```shell
    $ install_name_tool -add_rpath "@executable_path/../Frameworks" \
        target/release/bundle/osx/System\ Syzygy.app/Contents/MacOS/syzygy
    ```

4. Add the SDL2 framework to the app bundle:

    ```shell
    $ mkdir target/release/bundle/osx/System\ Syzygy.app/Contents/Frameworks
    $ cp -R /Library/Frameworks/SDL2.framework \
        target/release/bundle/osx/System\ Syzygy.app/Contents/Frameworks/
    ```

5. Strip the binary:

    ```shell
    $ strip target/release/bundle/osx/System\ Syzygy.app/Contents/MacOS/syzygy
    ```

6. Compress the app bundle:

    ```shell
    $ ditto -c -k --keepParent target/release/bundle/osx/System\ Syzygy.app \
        ./System-Syzygy-v1.0.0-Mac.zip
    ```
