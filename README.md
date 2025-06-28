# Resource Monitor: Lightweight Terminal-Based Dashboard in Rust

![GitHub Repo Stars](https://img.shields.io/github/stars/GabrielCosta14/resource-monitor?style=social) ![GitHub Release](https://img.shields.io/github/release/GabrielCosta14/resource-monitor.svg) ![GitHub Issues](https://img.shields.io/github/issues/GabrielCosta14/resource-monitor) ![GitHub Forks](https://img.shields.io/github/forks/GabrielCosta14/resource-monitor) 

## Overview

Resource Monitor is a lightweight, terminal-based resource dashboard built in Rust. It offers a clean and efficient way to monitor system resources without requiring elevated privileges. This tool is designed to work seamlessly on contemporary macOS versions, from Monterey to Sonoma.

You can download the latest release from the [Releases section](https://github.com/GabrielCosta14/resource-monitor/releases). Simply choose the appropriate file for your system, download it, and execute it in your terminal.

## Features

- **Unprivileged Access**: Operate without requiring administrative rights.
- **Cross-Platform Compatibility**: Specifically designed for macOS (Monterey → Sonoma).
- **Real-Time Monitoring**: Get instant updates on CPU, memory, and disk usage.
- **User-Friendly Interface**: Navigate easily with a terminal-based UI.
- **Lightweight**: Minimal resource consumption, making it ideal for everyday use.

## Installation

To install Resource Monitor, follow these steps:

1. Visit the [Releases section](https://github.com/GabrielCosta14/resource-monitor/releases).
2. Download the latest version for your macOS.
3. Open your terminal.
4. Navigate to the directory where you downloaded the file.
5. Run the file using the command:

   ```bash
   ./resource-monitor
   ```

## Usage

Once you have installed Resource Monitor, you can start monitoring your system resources. Simply run the command:

```bash
./resource-monitor
```

The dashboard will display real-time data on CPU, memory, and disk usage. Use the keyboard shortcuts to navigate through different sections:

- **Arrow Keys**: Navigate through the options.
- **Q**: Quit the application.

## Dependencies

Resource Monitor relies on several Rust libraries to function effectively:

- **Ratatui**: A Rust library for building terminal user interfaces.
- **Crossterm**: A cross-platform library for handling terminal I/O.
- **Sysinfo**: A Rust library for gathering system information.

Make sure you have Rust installed on your system. You can install Rust by following the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).

## Development

If you're interested in contributing to Resource Monitor, follow these steps:

1. Fork the repository.
2. Create a new branch for your feature or bug fix.
3. Make your changes and commit them with clear messages.
4. Push your branch to your forked repository.
5. Open a pull request to the main repository.

### Code Structure

The code is organized into several modules:

- **main.rs**: The entry point of the application.
- **ui.rs**: Handles the terminal user interface.
- **monitor.rs**: Contains functions for monitoring system resources.
- **utils.rs**: Utility functions used throughout the application.

### Testing

To ensure the reliability of Resource Monitor, we encourage you to write tests for your code. You can run the tests using the following command:

```bash
cargo test
```

## Issues

If you encounter any bugs or have feature requests, please check the [Issues section](https://github.com/GabrielCosta14/resource-monitor/issues) of the repository. You can report a new issue or contribute to existing discussions.

## Community

Join our community of users and developers. Share your experiences, ask questions, and get help with Resource Monitor. You can connect with us through:

- **GitHub Discussions**: Engage with other users and developers.
- **Social Media**: Follow us for updates and news.

## License

Resource Monitor is licensed under the MIT License. You are free to use, modify, and distribute this software as long as you include the original license.

## Acknowledgments

- **Ratatui**: For providing a powerful framework for terminal UI development.
- **Crossterm**: For enabling cross-platform terminal handling.
- **Sysinfo**: For allowing easy access to system information.

## Resources

- [Ratatui Documentation](https://docs.rs/ratatui)
- [Crossterm Documentation](https://docs.rs/crossterm)
- [Sysinfo Documentation](https://docs.rs/sysinfo)

## Support

If you have questions or need assistance, feel free to reach out through the [Issues section](https://github.com/GabrielCosta14/resource-monitor/issues) or join our community discussions.

You can also download the latest version from the [Releases section](https://github.com/GabrielCosta14/resource-monitor/releases) and start using Resource Monitor today!