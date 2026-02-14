# Activation checklist

This file confirms automation has been configured and activated.

- Dependabot: configured
- Mergify: config added in `.mergify.yml` (install app to activate)
- CodeQL: workflow added
- cargo-chef + actions/cache: CI configured
- sccache validation workflow: added

If you installed Mergify in the repository, `.mergify.yml` rules will be active after merging PR #17.