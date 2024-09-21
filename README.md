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


## Install

[Download the latest release for your platform.](https://github.com/EnigmaCurry/dry_console/releases)

Or install via cargo ([crates.io/crates/hushcrumbs](https://crates.io/crates/hushcrumbs)):

```
cargo install hushcrumbs
```


## Usage

```
$ hushcrumbs

Usage: hushcrumbs [OPTIONS] [COMMAND]

Commands:
  init     Creates a new backup directory
  deinit   Restores all original files and unconfigures the backup directory
  add      Adds a file to the backup and creates a symlink
  restore  Restores backup files
  rm       Removes a file from the backup [aliases: remove]
  ls       Lists backups or files in a backup [aliases: list]
  commit   Commits a backup (placeholder)
  push     Pushes a backup (placeholder)
  help     Print this message or the help of the given subcommand(s)

Options:
  -c, --config <CONFIG_FILE>  Sets the path to the global config file. [default: /home/ryan/.config/hushcrumbs/config.ron]
      --log <LEVEL>           Sets the log level, overriding the RUST_LOG environment variable. [possible values: trace, debug, info, warn, error]
  -v                          Sets the log level to debug.
      --no-confirm            Disables all interactive confirmation (careful!)
  -h, --help                  Print help
  -V, --version               Print version

```

### Initialize a new backup with a name and location

```
## hushcrumbs init <BACKUP_NAME> <PATH>
## Example:
hushcrumbs init test /tmp/test
```

This will create a new backup named `test` at the path `/tmp/test`.
(The name of the backup and the name of directory are independent of
each other.)

### List all backups

```
hushcrumbs ls
```

This will print a list of the backups that have been initialized:

```
 Backup Name | Backup Path 
-------------+-------------
 test        | /tmp/test 
```

### Add files to the named backup

```
## hushcrumbs add <BACKUP_NAME> <PATH>
## Example:
touch /tmp/hello.txt
hushcrumbs add test /tmp/hello.txt
```

The path `/tmp/hello.txt` is moved to the `/tmp/test` backup folder,
and a new symlink is created which points to it at the original path
`/tmp/hello.txt`.

### List all files in the named backup

```
## hushcrumbs ls <BACKUP_NAME>
## Example:
hushcrumbs list test
```

This will list all of the files contained in the `test` backup:

```
 Local files contained in backup (test): 
-----------------------------------------
 /tmp/hello.txt 
```

### Remove a file from the backup

```
## hushcrumbs rm <BACKUP_NAME> <PATH>
## Example:
hushcrumbs rm test /tmp/hello.txt
```

This removes the symlink, and replaces the original file contents back
in place at `/tmp/hello.txt`, then the file is removed from the
backup.

If the symlink has already been deleted, and you now wish to remove it
from the backup without restoring it, use the `--delete` flag:

```
## To permanently delete the file AND the backup of it:
## hushcrumbs rm <BACKUP_NAME> <PATH> --delete
## Example:
hushcrumbs rm test /tmp/hello.txt --delete
```

This is a destructive operation, so there is an interactive
confirmation required to proceed. If you are doing this unattended,
you may also add the `--no-confirm` option to disable the confirmation
prompt.
 
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
