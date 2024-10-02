# Contributing to Cliq

Thank you for your interest in contributing to Cliq! Contributions are welcome and appreciated.

## Table of Contents

- [How Can I Contribute?](#how-can-i-contribute)
  - [Reporting Bugs](#reporting-bugs)
  - [Suggesting Features](#suggesting-features)
  - [Improving Documentation](#improving-documentation)
  - [Contributing Code](#contributing-code)
- [Setting Up the Development Environment](#setting-up-the-development-environment)
- [Creating a Pull Request](#creating-a-pull-request)
- [License](#license)

## How Can I Contribute?

### Reporting Bugs

If you find a bug, please report it by creating a new issue. Be sure to include:

- A clear description of the bug.
- Steps to reproduce the bug.
- Expected and actual behavior.
- Any relevant screenshots or error messages.
- Your environment (OS, Rust version, etc.).

### Suggesting Features

We’re open to new ideas! To suggest a feature:

- Check if the feature is already being discussed in the [issues](https://github.com/santhosh-chinnasamy/cliq/issues).
- If not, open a new issue with a clear and detailed description of the feature.
- Discuss how the feature would benefit the project and its users.

### Improving Documentation

Contributions to the documentation are always welcome. If you spot an area for improvement, feel free to:

- Suggest changes by creating an issue.
- Submit a pull request with your changes.

### Contributing Code

#### Before You Start

- Check the [open issues](https://github.com/santhosh-chinnasamy/cliq/issues) to see if someone else is already working on a similar fix or feature.
- If not, create a new issue or comment on an existing one to express your interest in working on it.
- Wait for approval from a maintainer before starting work on major features.

#### Guidelines

- Keep your code clean and readable.
- Follow Rust best practices and ensure the code compiles without errors.
- Make sure to include tests for your changes, when applicable.

## Setting Up the Development Environment

1. **Clone the repository:**

   ```bash
   git clone https://github.com/santhosh-chinnasamy/cliq.git
   cd cliq
   ```

2. **Ensure you have Rust installed:**

   You can install Rust using [rustup](https://rustup.rs/):

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

3. **Install dependencies:**

   If the project has any external dependencies, you can install them using `cargo`:

   ```bash
   cargo build
   ```

4. **Run tests:** (WIP #16)

   To ensure everything is working as expected:

   ```bash
   cargo test
   ```

## Creating a Pull Request

- Fork the repository.
- Create a new branch:
  ```bash
  git checkout -b your-feature-branch
  ```
- Make your changes, ensuring that your code is properly tested and formatted.
- Update the README.md with details of changes to the interface, if applicable.
- Update the CHANGELOG.md with notes on your changes.
- Commit your changes:
  ```bash
  git commit -m "Add description of the changes"
  ```
- Push the branch:
  ```bash
  git push origin your-feature-branch
  ```
- Open a pull request from your fork’s branch to the `main` branch of the Cliq repository.
- Describe your changes clearly in the pull request and link any related issues.
- The PR will be merged once you have the sign-off of at least one other developer/maintainer.

## License

By contributing to Cliq, you agree that your contributions will be licensed under the [MIT License](./LICENSE).
