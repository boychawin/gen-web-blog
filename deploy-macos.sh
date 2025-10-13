#!/bin/bash

# GenWebBlog macOS-Only Release Deployment Script
# This script automates the process of creating a macOS-focused release
# Changes: removed awk usage for version math; use IFS/read and shell arithmetic
# to avoid awk quoting/compatibility issues on macOS.

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Allow skipping remote/git operations (useful for CI or dry-run)
# Set SKIP_REMOTE=1 to skip pushing, tagging and interactive prompts
SKIP_REMOTE="${SKIP_REMOTE:-0}"

# Helper functions
info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

success() {
    echo -e "${GREEN}✅ $1${NC}"
}

warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

error() {
    echo -e "${RED}❌ $1${NC}"
    exit 1
}

# Check if we're in the right directory
if [[ ! -f "Cargo.toml" ]] || [[ ! -f "app.toml" ]]; then
    error "This script must be run from the root of the GenWebBlog project"
fi

# Check if we're on macOS
if [[ "$(uname -s)" != "Darwin" ]]; then
    error "This script is designed to run on macOS only"
fi

# Get current version from Cargo.toml (prefer the package section)
CURRENT_VERSION=$(awk 'BEGIN{pkg=0} /^\[package\]/{pkg=1;next} pkg && /^[[:space:]]*version[[:space:]]*=/{match($0,/version[[:space:]]*=[[:space:]]*"(.*)"/,m); if(m[1]!=""){print m[1]; exit}}' Cargo.toml)
if [[ -z "$CURRENT_VERSION" ]]; then
    error "Failed to determine current version from Cargo.toml"
fi
# Strip any pre-release/build metadata for simple numeric bumping (e.g., 1.2.3-beta -> 1.2.3)
VERSION_CORE="${CURRENT_VERSION%%[-+]*}"
info "Current version: $CURRENT_VERSION (core: $VERSION_CORE)"

# Parse version into components (major.minor.patch) from the core semver part
IFS='.' read -r MAJOR MINOR PATCH <<< "$VERSION_CORE"

# Ensure numeric defaults
MAJOR=${MAJOR:-0}
MINOR=${MINOR:-0}
PATCH=${PATCH:-0}

# Validate numeric parts (fallback to 0 if not numeric)
re='^[0-9]+$'
if ! [[ $MAJOR =~ $re ]]; then MAJOR=0; fi
if ! [[ $MINOR =~ $re ]]; then MINOR=0; fi
if ! [[ $PATCH =~ $re ]]; then PATCH=0; fi

PATCH_NEXT=$((PATCH + 1))
MINOR_NEXT=$((MINOR + 1))
MINOR_BUMP_VERSION="${MAJOR}.${MINOR_NEXT}.0"
PATCH_BUMP_VERSION="${MAJOR}.${MINOR}.${PATCH_NEXT}"

# Check for uncommitted changes
if [[ -n $(git status --porcelain) ]]; then
    warning "You have uncommitted changes:"
    git status --short
    echo ""
    if [[ "${SKIP_REMOTE:-}" == "1" || "${SKIP_REMOTE:-}" == "true" ]]; then
        warning "SKIP_REMOTE is set; skipping interactive commit/push steps."
    else
        read -p "Do you want to commit these changes? [y/N] " -n 1 -r
        echo
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            # Get commit message
            echo "Enter commit message (or press Enter for default):"
            read -r COMMIT_MSG
            if [[ -z "$COMMIT_MSG" ]]; then
                COMMIT_MSG="🍎 macOS-focused release

- Simplified GitHub Actions for macOS builds only
- Updated installation scripts for macOS priority
- Fixed Rust toolchain compatibility issues
- Better error handling for unsupported platforms"
            fi

            info "Staging all changes..."
            git add .

            info "Committing changes..."
            git commit -m "$COMMIT_MSG"
            success "Changes committed successfully"
        else
            error "Please commit your changes before creating a release"
        fi
    fi
fi

# Ask for version bump
echo ""
info "Current version: v$CURRENT_VERSION"
echo "Release options:"
echo "1) Patch release (bug fixes) - $PATCH_BUMP_VERSION"
echo "2) Minor release (new features) - $MINOR_BUMP_VERSION"
echo "3) Keep current version"
echo "4) Custom version"

# If SKIP_REMOTE is set, default to keeping current version to avoid interactive input
if [[ "${SKIP_REMOTE:-}" == "1" || "${SKIP_REMOTE:-}" == "true" ]]; then
    info "SKIP_REMOTE is set — defaulting to keep current version (non-interactive mode)."
    VERSION_OPTION=3
else
    read -p "Select option [1-4]: " -n 1 -r VERSION_OPTION
    echo ""
fi

case $VERSION_OPTION in
    1)
        NEW_VERSION="$PATCH_BUMP_VERSION"
        ;;
    2)
        NEW_VERSION="$MINOR_BUMP_VERSION"
        ;;
    3)
        NEW_VERSION="$CURRENT_VERSION"
        ;;
    4)
        if [[ "${SKIP_REMOTE:-}" == "1" || "${SKIP_REMOTE:-}" == "true" ]]; then
            error "Custom version requires interactive input. Run without SKIP_REMOTE=1 to set custom version."
        fi
        read -p "Enter new version (e.g., 1.5.2): " NEW_VERSION
        ;;
    *)
        error "Invalid option selected"
        ;;
esac

info "Target version: v$NEW_VERSION"

# Update version if needed
if [[ "$NEW_VERSION" != "$CURRENT_VERSION" ]]; then
    info "Updating version in Cargo.toml..."

    # Update version manually if cargo-set-version not available
    if command -v cargo-set-version >/dev/null 2>&1; then
        cargo set-version "$NEW_VERSION"
    else
        # Manual version update without relying on sed -i (more portable)
        # Replace the first version line found in the [package] section only.
        awk -v ver="$NEW_VERSION" '
          BEGIN{pkg=0}
          /^\[package\]/{pkg=1; print; next}
          pkg && /^[[:space:]]*version[[:space:]]*=/{print "version = \"" ver "\""; pkg=0; next}
          {print}
        ' Cargo.toml > Cargo.toml.tmp && mv Cargo.toml.tmp Cargo.toml
        info "Updated version manually (cargo-edit not available)"
    fi

    # Update Cargo.lock
    cargo check --quiet

    # Commit version change (skip if SKIP_REMOTE is set)
    if [[ "${SKIP_REMOTE:-}" == "1" || "${SKIP_REMOTE:-}" == "true" ]]; then
        warning "SKIP_REMOTE is set; skipping git add/commit for version bump."
    else
        git add Cargo.toml Cargo.lock
        git commit -m "🔖 Bump version to v$NEW_VERSION"
        success "Version updated to v$NEW_VERSION"
    fi
fi

# Quick local build test
info "Testing local build..."
if ! cargo build --quiet; then
    error "Local build failed. Please fix before releasing."
fi
success "Local build successful"

# Run basic tests
info "Running tests..."
if ! cargo test --quiet; then
    warning "Some tests failed."
    if [[ "${SKIP_REMOTE:-}" == "1" || "${SKIP_REMOTE:-}" == "true" ]]; then
        warning "SKIP_REMOTE is set; continuing despite test failures (non-interactive mode)."
    else
        warning "Continue anyway? [y/N]"
        read -p "" -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            error "Fix tests before releasing"
        fi
    fi
else
    success "Tests passed"
fi

# Format code
info "Formatting code..."
cargo fmt
success "Code formatted"

# Push changes to GitHub
if [[ "${SKIP_REMOTE:-}" == "1" || "${SKIP_REMOTE:-}" == "true" ]]; then
    warning "SKIP_REMOTE is set; skipping push to remote (origin)."
else
    BRANCH=$(git rev-parse --abbrev-ref HEAD)
    info "Pushing changes to GitHub branch: $BRANCH"
    git push origin "$BRANCH"
    success "Changes pushed to $BRANCH branch"
fi

# Create and push tag
TAG_NAME="v$NEW_VERSION"
info "Creating tag: $TAG_NAME"

if [[ "${SKIP_REMOTE:-}" == "1" || "${SKIP_REMOTE:-}" == "true" ]]; then
    warning "SKIP_REMOTE is set; skipping tag creation and remote push."
else
    # Check if tag already exists
    if git tag -l | grep -q "^$TAG_NAME$"; then
        warning "Tag $TAG_NAME already exists"
        read -p "Do you want to delete and recreate it? [y/N] " -n 1 -r
        echo
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            git tag -d "$TAG_NAME"
            git push origin :refs/tags/"$TAG_NAME"
            info "Deleted existing tag"
        else
            error "Tag already exists. Aborting."
        fi
    fi

    # Create tag with message
    cat > /tmp/tag_message << EOF
GenWebBlog v$NEW_VERSION (macOS Release)

🍎 macOS-Focused Release

This release prioritizes macOS compatibility and stability:

✅ Native Apple Silicon (M1/M2/M3) support
✅ Intel Mac compatibility
✅ Optimized build process
✅ Improved installation experience
✅ Better error handling

📦 Installation (macOS):
curl -fsSL https://raw.githubusercontent.com/boychawin/gen-web-blog/main/install.sh | bash

🔧 For other platforms:
cargo install --git https://github.com/boychawin/gen-web-blog

⚡ Quick Start:
genwebblog init my-blog
cd my-blog
genwebblog start

🌟 Features:
- Lightning-fast static site generation
- Built-in multilingual support (Thai, English, Japanese)
- SEO optimization
- One-command deployment to GitHub Pages
- Beautiful responsive themes
- Markdown-first content management
EOF

    git tag -a "$TAG_NAME" -F /tmp/tag_message
    rm /tmp/tag_message

    info "Pushing tag to GitHub..."
    git push origin "$TAG_NAME"
    success "Tag $TAG_NAME pushed successfully"
fi

# Wait for GitHub to process
sleep 3

echo ""
success "🎉 macOS Release deployment completed!"
echo ""
info "GitHub Actions will now:"
echo "  • Build binaries for macOS (Intel + Apple Silicon)"
echo "  • Run comprehensive tests on macOS"
echo "  • Create GitHub release with download links"
echo ""
info "Monitor the progress at:"
echo "  https://github.com/boychawin/gen-web-blog/actions"
echo ""
info "macOS users can install with:"
echo "  curl -fsSL https://raw.githubusercontent.com/boychawin/gen-web-blog/main/install.sh | bash"
echo ""
info "Other platforms can use:"
echo "  cargo install --git https://github.com/boychawin/gen-web-blog"
echo ""
info "Release page will be available at:"
echo "  https://github.com/boychawin/gen-web-blog/releases/tag/$TAG_NAME"

# Optional: Open GitHub Actions page
if command -v open >/dev/null 2>&1; then
    if [[ "${SKIP_REMOTE:-}" == "1" || "${SKIP_REMOTE:-}" == "true" ]]; then
        info "SKIP_REMOTE is set; not opening browser."
    else
        read -p "Open GitHub Actions page in browser? [y/N] " -n 1 -r
        echo
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            open "https://github.com/boychawin/gen-web-blog/actions"
        fi
    fi
fi

echo ""
success "🚀 Ready for macOS users to download and install!"
success "Build will complete in 5-10 minutes."

# Show next steps
echo ""
info "📝 Next steps:"
echo "1. Wait for GitHub Actions to complete"
echo "2. Test the installation on different Macs"
echo "3. Share with macOS users first"
echo "4. Add Linux/Windows support later"
echo ""
info "🎯 Focus: Deliver excellent macOS experience first!"
