### Split2ops Software Changelog

**Version: 1.0.0**

#### Analytics Module
- **Added**: New `analytics_menu.rs` file providing an analytics menu.
- **Features**: View Packet File functionality to read lines, detect IP addresses using regex, and display lines with robust error handling.

#### Data Speed Module
- **Cleaned Up**: Removed unused import `self` in `ds_menu.rs`.

#### Administrative Menu
- **Updated**: Integrated the `analytics_menu` option into the administrative menu in `admin_menu.rs`.
- **Cleaned Up**: Removed unused import `self` in `admin_menu.rs`.

#### Packet Capture Module
- **Updated**: Enhanced `set_capture_duration` function for consistent duration handling in `pc_menu.rs`.
- **Added**: Safer global capture duration management.
- **Improved**: Enhanced packet capture command execution and argument handling.
- **Feature**: Added functionality to print the location of the exported packet file.
- **Enhanced**: Initial check for existing export files upon menu start.
- **Improved**: Enhanced error handling and user feedback mechanisms.

#### Core Modules
- **Compatibility**: Ensured `bootup.rs`, `lib.rs`, `block_all.rs`, `data_speeds.rs`, `permissions.rs`, `menu.rs` are compatible with new updates.
- **Testing**: Verified no new warnings or errors post-updates.

#### Main Entry Point
- **Updated**: Ensured `main.rs` correctly references new and updated modules.

#### Dependency Management
- **Added**: New dependency on the `regex` crate for IP address detection in `Cargo.toml`.
- **Updated**: General dependency management and version alignment.

### Summary
- **Enhancements**: Streamlined packet capture and export processes, integrated analytics for packet data, improved error handling, and user feedback.
- **Cleanups**: Removed unused imports, organized and improved code consistency.
- **New Features**: Added analytics capabilities, extended functionalities in packet capture.
4364