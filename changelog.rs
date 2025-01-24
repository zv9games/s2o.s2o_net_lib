### Program Name: s2o_net_lib 0.1

**Description:**
s2o_net_lib is a comprehensive network tool designed to test data speeds, capture packets, and set firewall rules. This tool provides essential functionalities for network diagnostics and management.

**Files:**
- uac.ps1: PowerShell script for running an executable with administrator privileges.
- Cargo.toml: Configuration file specifying dependencies and metadata for the Rust project.
- build.rs: Build script for setting up the environment and linking necessary DLLs.
- main.rs: Main entry point for the application, initializing logging and the application state.
- logging.rs: Module for managing logging, including initialization and logging functions.
- gui_engine.rs: Module for rendering the graphical user interface (GUI), including menu settings and state.
- initialization.rs: Module for initializing the application, setting DLL paths, and checking for elevated privileges.
- app_state.rs: Module for managing the application state using an enum and shared state structure.
- s_menu.rs: Module for rendering the security menu and managing security-related actions.
- p_menu.rs: Module for rendering the program menu and managing program-related actions.
- pc_menu.rs: Module for rendering the packet capture menu and handling packet capture logic.
- pcnc.rs: Module for managing the packet capture network capture interface.
- ds_menu.rs: Module for rendering the data speed menu and managing data speed-related actions.
- ns_menu.rs: Module for rendering the network settings menu and managing network settings-related actions.
- packet_sniffer.rs: Module for interfacing with the packet sniffer DLL, including starting/stopping the sniffer and capturing packets.
