# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.0](https://github.com/tversteeg/sprite-gen/compare/sprite-v0.3.0...sprite-v0.4.0) - 2024-01-14

### Added
- non-functional save sheet button

### Fixed
- *(deps)* update rust crate rfd to 0.12.1

### Other
- *(deps)* update swatinem/rust-cache action to v2.7.3
- *(deps)* update swatinem/rust-cache action to v2.7.2
- *(deps)* update swatinem/rust-cache action to v2.7.1
- *(ci)* use updated deployment
- *(deps)* update swatinem/rust-cache action to v2.7.1
- Release
- fix versions for release
- [**breaking**] rewrite with custom drawing ([#33](https://github.com/tversteeg/sprite-gen/pull/33))
- Update Rust crate serde to 1.0.147
- Add renovate.json
- Update itertools requirement from 0.9.0 to 0.10.0
- Update rand requirement from 0.7.3 to 0.8.0
- Fix lib version in README.md
- Update minifb requirement from 0.16.0 to 0.17.0
- Partially update to druid 0.6
- Bump version to 0.2.5
- Update format & remove rand in favor of randomize
- Update to new druid updates & move menu bar to buttons
- Use unstable/latest version of druid
- Update itertools requirement from 0.8.2 to 0.9.0
- Amount of results is now configurable
- Bump lib version to 0.1.9
- Merge branch 'master' of github.com:tversteeg/sprite-gen
- Update rand dependency
- Fix clippy warning
- Attempt to open file
- Allow drawing while dragging instead of clicking
- (cargo-release) start next development iteration 0.2.5-alpha.0
- (cargo-release) version 0.2.4
- Fix bug & update example image
- Colored panel is hidden when option is disabled
- Add gui elements for all sprite-gen options
- Move widgets & appstate to new file
- Fix copying of grid
- (cargo-release) start next development iteration 0.2.4-alpha.0
- (cargo-release) version 0.2.3
- Implement copy to clipboard button
- Bump sprite version to 0.1.8
- Bump sprite version to 0.1.7
- Implement Debug for MaskValue
- Add export button
- Change build status badge in CI
- Remove Linux MUSL target from CI
- (cargo-release) start next development iteration 0.2.3-alpha.0
- (cargo-release) version 0.2.2
- Implement new-file & save-file functionality
- Merge branch 'master' of github.com:tversteeg/sprite-gen
- Make selector a constant
- Fix CI dependencies for compile & clippy
- Install druid dependencies in CI
- Remove border from image
- (cargo-release) start next development iteration 0.2.2-alpha.0
- (cargo-release) version 0.2.1
- Change colors and upload new example image
- Add mirror X & mirror Y options
- Add option to change the render scale
- (cargo-release) start next development iteration 0.2.1-alpha.0
- (cargo-release) version 0.2.0
- First working version using Druid
- Expose i8 as a function in MaskValue
- (cargo-release) version 0.1.5
- Add MaskValue type
- Add MaskValue type
- Drawing grid is now functional
- Render grid
- Fix clippy warning
- Render background square for the grid
- Add custom druid widget for the grid
- Remove unused 'use' warnings
- Start creating a more complete gui with druid
- Fix clippy warnings for lib & example
- Fix badge URL in README.md
- Replace CI badge in README.md
- Replace Travis CI with GitHub CI
- Update dependencies
- Bump minifb version to 0.15.0
- Update minifb requirement from 0.13 to 0.14
- Apply cargo fmt
- Better run instructions
- Set Rust to 2018 version
- Update minifb requirement from 0.12 to 0.13
- Update minifb requirement from 0.11 to 0.12
- Merge branch 'master' into dependabot/cargo/blit-0.5
- Update minifb requirement from 0.10 to 0.11
- Merge branch 'master' of github.com:tversteeg/sprite-gen
- Bump Cargo version to 0.1.6
- Fix library conflict
- Bump both versions
- Update example & fix small 1bit error
- Implement colorization algorithm
- Add documentation and setup structure for adding colors
- Add documentation
- Bump library version to 0.1.2
- Implement mirroring in all directions
- Bump Cargo version to 0.1.4
- Make dynamic drawing of new sprites possible
- Make mask drawable
- Bump version to 0.1.3
- Add drawing buttons with new direct-gui version and redraw sprites on spacebar
- Update blit to 0.4
- Attempt to use a 'direct-gui' for dynamically creating the mask
- Wildcard dependencies are needed for Cargo
- Fix README.md, make unused library calls unimplemented
- Small Cargo fixes
- Add needed version for Cargo upload
- Split executable from example
- Fix algorithm, add mirror_x reflection and draw_grid function
- Add CI setup
- Implement basic version of the algorithm and show it in a simple example
- Setup base structure
