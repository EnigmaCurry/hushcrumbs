PACKAGE_NAME=hushcrumbs

.PHONY: all build install clean deps reinstall

all: build

deps:
	@command -v python3 >/dev/null 2>&1 || { echo "❌ python3 is not installed."; exit 1; }
	@command -v poetry >/dev/null 2>&1 || { echo "❌ poetry is not installed."; exit 1; }
	@command -v pipx >/dev/null 2>&1 || { echo "❌ pipx is not installed."; exit 1; }
	@echo "✅ All dependencies are installed."

build: deps
	poetry build

install: build
	@echo "🔍 Installing latest wheel with pipx..."
	@pipx install "$$(ls -t dist/$(PACKAGE_NAME)-*.whl | head -n 1)"

reinstall: clean install

uninstall: deps
	pipx uninstall hushcrumbs

test: build
	poetry run pytest

clean:
	@echo "🧹 Cleaning up build artifacts..."
	@rm -rf build/ dist/ *.egg-info
