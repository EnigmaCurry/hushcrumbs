# Hushcrumbs

Hushcrumbs is another secrets manager. Its job is to manage disperse
files, living anywhere in your filesystem, and moving those files into
a central repository. It then replaces the original files with
symlinks to the central location. This lets you colocate the symlinks
(eg. `.env` files) amongst your various project directories, wherever
they might live. One major benefit of storing all of your secrets in a
centralized directory, is that it makes it trivial to wipe them all,
en masse. Additionally, `git` will never commit the contents of any
symlink, but can still track its location (by relative or absolute
path).

**This tool does not perform any encryption at rest** (Nb. you must
completely trust your own system, as the secrets are always
unencryped: any process on your system, assuming it has appropriate
permission, can read the secrets file in plain text!), however, this
tool does (or will) have the ability to produce encrypted backups
which you may want to store offsite, and this tool will also
facilitate future restoration of those original files, from such an
encrypted backup. 

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
git clone git@github.com:EnigmaCurry/hushcrumbs.git \
  ~/git/vendor/enigmacurry/hushcrumbs
cd ~/git/vendor/enigmacurry/hushcrumbs
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

### Build release binary

```
just build --release
```

### Create development alias

```
## Add this to ~/.bashrc or equivalent:
alias hushcrumbs='just -f ~/git/vendor/enigmacurry/hushcrumbs/Justfile run'
alias h=hushcrumbs
```

Now you can run `hushcrumbs`, or simply `h`, from any directory, with
any arguments, and it will automatically rebuild from source, and then
run it with those args.

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

### Clippy

```
just clippy
just clippy --fix
```

### Release (Github actions)

#### Bump release version and push new branch

The `bump-version` target will automatically update the version number
in Cargo.toml, Cargo.lock, and README.md as suggested by git-cliff.
This creates a new branch named `release-{VERSION}`, and automatically
checks it out. You just need to `git push` the branch:

```
just bump-version
# ... automatically checks out a new branch named release-{VERSION}

git push
```

#### Make a new PR with the changeset

Branch protection is enabled, all changesets must come in the form of
a Pull Request. On GitHub, create a new Pull Request for the
`release-{VERSION}` branch into the master branch.

#### Merge the PR and tag the release

Once the PR is merged, update your local repo, and run the release
target:

```
git checkout master
git pull
just release
```

New binaries will be automatically built by github actions, and a new
packaged release will be posted.
