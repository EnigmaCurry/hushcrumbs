# hushcrumbs

Hushcrumbs is another secrets manager. Its job is to manage disperse
files anywhere in your filesystem, moving them into a central
repository, and replacing the original files with symlinks to the
central location. This lets you colocate symlinks to your secrets (eg.
`.env` files) amongst your project directories, wherever they might
live.

**This tool does not perform any encryption at rest** (Nb. you must
completely trust your own filesystem permissions: any process on your
system, assuming it has appropriate permission, can read the secrets
file in plain text!), however, this tool does have the ability to
produce encrypted backups, and can also facilitate future restoration
from such an encrypted backup, and makes it easy to wipe all secrets
en masse, because they are all stored in a central directory.

## STATUS: EXPERIMENTAL

```
THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
```


## Development

These instructions are specific to Fedora; minor adjustments for your
platform may be required.

### Install host dependencies

```
sudo dnf install git openssh rustup
sudo dnf group install "C Development Tools and Libraries" "Development Tools"
```

### Install rust and cargo

```
rustup-init ## just press enter when prompted for default selection
. "$HOME/.cargo/env"
```

### Clone source repository

```
git clone git@github.com:EnigmaCurry/secrets.git \
  ~/git/vendor/enigmacurry/secrets
cd ~/git/vendor/enigmacurry/secrets
```

### Install development dependencies

```
cargo install just
just deps
```

### Build and run development app

```
just run help
just run [ARGS ...]
```

### Create development alias

```
## Add this to ~/.bashrc or equivalent:
alias secrets='just -f ~/git/vendor/enigmacurry/secrets/Justfile run'
```

Now you can run `secrets` from any directory, with any arguments, and it
will automatically rebuild from source and run it.

## Testing

This project has incomplete testing.

### Run tests

```
# Run all tests:
just test

# Run a single test:
just test test_cli_help

# Verbose logging (which normally would be hidden for passing tests)
just test-verbose test_cli_help

# Auto run tests on source change:
just test-watch
```

