# secrets

This is another secrets manager.

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
