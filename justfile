# bash/justfile

# https://just.systems/man/en/chapter_1.html

# ------------------------------------------------------------------------------
# CONFIG

set shell := ["bash", "-uc"]

# ------------------------------------------------------------------------------
# VARIABLES

NODE_VERSION := "v16.13.2"

################################################################################
# RECIPES
################################################################################

# DEFAULT RECIPE WHEN USER DOES NOT GIVE A SPECIFIC RECIPE

@_default:
    echo "View file 'justfile' to see internals of any recipe."
    just --list --unsorted

# ------------------------------------------------------------------------------
# BASIC RECIPES

# Initialize the repo. Necessary after initial git-clone.
init:
    #!/bin/bash
    # this is a bash script, not a justfile recipe
    # it checks for rust, wasm compile target, wasm-pack, node, npm, and npx
    TASKS=()

    if ! command -v rustc &> /dev/null || ! command -v cargo &> /dev/null; then
        TASKS+=("install rust")
    else
        if ! rustup target list | grep "wasm32-unknown-unknown .*installed" &> /dev/null; then
            TASKS+=("install wasm32-unknown-unknown via: rustup target add wasm32-unknown-unknown")
        fi

        if ! command -v wasm-pack &> /dev/null; then
            TASKS+=("install wasm-pack")
        fi
    fi

    if ! command -v node &> /dev/null; then
        TASKS+=("install node")
    else
        if ! [[ $(node -v) == "{{NODE_VERSION}}" ]]; then
            TASKS+=("need node {{NODE_VERSION}} via: nvm install {{NODE_VERSION}} && nvm use")
        fi

        # TODO: update js version to something more recent, then uncomment pnpm lines
        # prefer pnpm over npm -- for more info: https://pnpm.io/
        # if ! command -v pnpm &> /dev/null; then
            if ! command -v npm &> /dev/null; then
                TASKS+=("install pnpm or npm")
            else
                npm install
            fi
        # else
            # pnpm install
        # fi

        if ! command -v npx &> /dev/null; then
            TASKS+=("install npx")
        fi
    fi

    if [ ${#TASKS[@]} -gt 0 ]; then
        echo ""
        echo "######################################################################"
        echo "#                      'just init' final output                      #"
        echo "######################################################################"
        echo ""
        echo -e "\033[1;31mWARNING: Repo is not yet fully initialized.\033[0m"
        echo ""
        echo "User must complete these tasks as appropriate for user's OS & configs."
        echo ""
        echo "TASKS:"
        for ((i = 0; i < ${#TASKS[@]}; i++)); do
            echo "- ${TASKS[$i]}"
        done
    fi

# Clean up the repo by removing all generated files.
@clean:
    rm -rf dist pkg

# ------------------------------------------------------------------------------
# LINTING

# Lint all code.
@lint: lint-js lint-rs

# Only lint (and fix) the JavaScript code via prettier.
@lint-js:
    npx prettier \
        $(git diff --name-only --diff-filter=ACM) \
        $(git diff --cached --name-only --diff-filter=ACM) \
        --write --ignore-unknown

# Only lint the Rust code via Clippy.
@lint-rs:
    cargo clippy --all-targets --all-features -- -D warnings

# ------------------------------------------------------------------------------
# RUN LOCAL SERVERS

# Run the development server, which has hot-reloading and debug-mode.
@start-dev: clean
    webpack-dev-server --open -d
alias dev := start-dev

# Run the distribution server, which does NOT have hot-reloading or debug-mode.
@start-dist: build
    python3 -m http.server 8000 --directory dist

# ------------------------------------------------------------------------------
# BUILD & DEPLOY

# Build the deployable distribution.
@build: clean
    webpack

# Manually publish the app to GitHub Pages.
@publish: build && clean
    gh-pages -d dist

# ------------------------------------------------------------------------------
# TESTING

# Run all tests.
test: test-rs test-wasm test-node

# Run Rust tests.
@test-rs:
    cargo test

# Run headless Wasm tests. Optionally give specific browsers; ex: just test-wasm chrome safari
test-wasm *BROWSERS='node chrome firefox safari':
    #!/bin/bash
    BROWSER_ARG=$(echo {{BROWSERS}} | sed 's/[^ ]* */--&/g')
    wasm-pack test --headless $BROWSER_ARG

# Run Wasm tests with Node.js. Optionally give specific tests; ex: just test-node tier_0 tier_1
test-node *TESTS:
    #!/bin/bash
    if [ -z "{{TESTS}}" ]; then
        WASM_BINDGEN_TEST_TIMEOUT=60 wasm-pack test --node
    else
        TEST_ARG=$(echo {{TESTS}} | sed 's/[^ ]* */--test &/g')
        WASM_BINDGEN_TEST_TIMEOUT=60 wasm-pack test --node -- $TEST_ARG
    fi