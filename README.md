# Hushcrumbs

[![Crates.io](https://img.shields.io/crates/v/hushcrumbs?color=blue
)](https://crates.io/crates/hushcrumbs)
[![Coverage](https://img.shields.io/badge/Coverage-Report-purple)](https://enigmacurry.github.io/hushcrumbs/coverage/master/)
[![Matrix chat](https://img.shields.io/badge/Matrix-Join_Chat-%234fb99a)](https://matrix.to/#/#blog.rymcg.tech:enigmacurry.com)

Hushcrumbs is another secrets manager. Its job is to centrally store
files that are linked throughout your filesystem. It can ingest any
file from any path, moving the file to a central repository, and
creating a symlink in its original path. This lets you colocate the
symlinks (eg. `.env` files) amongst your various project directories,
no matter where they might live. One major benefit of storing all of
your secrets in a centralized directory, is that it makes it trivial
to wipe all of them, en masse. Additionally, `git` will never commit
the contents of any symlink, but it can still track its location (by
relative or absolute path).

**Hushcrumbs does not perform any encryption at rest** (Nb. you must
completely trust your own system, as the secrets are always
unencryped; any process on your system can read the secrets file in
plain text, assuming it has filesystem permission to do so!), however,
this tool does (or will) have the ability to produce encrypted backups
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

[Download the latest release for your platform.](https://github.com/EnigmaCurry/hushcrumbs/releases)

Or install via cargo ([crates.io/crates/hushcrumbs](https://crates.io/crates/hushcrumbs)):

```
cargo install hushcrumbs
```

### Tab completion

To install tab completion support, put this in your `~/.bashrc` (assuming you use Bash):

```
### Bash completion for hushcrumbs (Put this in ~/.bashrc)
source <(hushcrumbs completions bash)
```

If you don't like to type out the full name `hushcrumbs`, you can make
a shorter alias (`h`), as well as enable tab completion for the alias
(`h`):

```
### Alias hushcrumbs as h (Put this in ~/.bashrc):
alias h=hushcrumbs
complete -F _hushcrumbs -o bashdefault -o default h
```

Completion for Zsh and/or Fish has also been implemented, but the
author has not tested this:

```
### Zsh completion for hushcrumbs (Put this in ~/.zshrc):
autoload -U compinit; compinit; source <(hushcrumbs completions zsh)

### Fish completion for hushcrumbs (Put this in ~/.config/fish/config.fish):
hushcrumbs completions fish | source
```


## Usage

```
$ hushcrumbs

Usage: hushcrumbs [OPTIONS] [COMMAND]

Commands:
  init         Creates a new backup directory
  deinit       Restores all original files and unconfigures the backup directory
  add          Adds a file to the backup and creates a symlink
  restore      Restores backup files
  rm           Removes a file from the backup [aliases: remove]
  ls           Lists backups or files in a backup [aliases: list]
  commit       Commits a backup (placeholder)
  push         Pushes a backup (placeholder)
  completions  Generates shell completions script (tab completion)
  help         Print this message or the help of the given subcommand(s)

Options:
  -c, --config <CONFIG_FILE>  Sets the path to the global config file [default: /home/ryan/.config/hushcrumbs/config.ron]
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

See [DEVELOPMENT.md](DEVELOPMENT.md)
