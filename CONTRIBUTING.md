# Contributing to nsp-cli

First off, thank you for considering contributing to `nsp-cli`! We appreciate all contributions, whether they are bug fixes, new features, or improvements to the documentation.

Please take a moment to review these guidelines to ensure a smooth process for all contributors.

## How to Contribute

### 1. Fork the Repository

To get started, fork the repository to your GitHub account and clone it locally:
```bash
git clone https://github.com/YOUR_USERNAME/nsp-cli.git
cd nsp-cli
```
### 2. Create a New Branch

Before making changes, create a new branch to isolate your work:
```bash
git checkout -b your-branch-name
```
### 3. Make Your Changes

Make your changes or additions to the codebase. This could include:

- Bug fixes
- New features
- Documentation improvements
- Code cleanups

### 4. Test Your Changes

Run the tests to make sure everything works as expected:
```bash
cargo test
```
If your changes require new tests, be sure to add them and verify that they pass.

### 5. Commit Your Changes

Once you're happy with your changes, commit them to your branch:
```bash
git add .
git commit -m "Description of your changes"
```
### 6. Push Your Changes

Push your changes to your fork:
```bash
git push origin your-branch-name
```
### 7. Create a Pull Request

Go to the repository on GitHub and open a pull request (PR). In your PR, describe the changes you made, and if applicable, explain why they are needed or how they improve the project. Make sure to reference any related issues by using #issue_number.

## Pull Request Guidelines

We ask that you follow these guidelines when submitting a pull request:

- **Keep it focused**: Each pull request should represent one logical change or addition. If you’re working on multiple features or fixes, open separate pull requests.
- **Provide a clear description**: Summarize what your PR does and why it’s necessary.
- **Follow code style**: Please follow the existing coding conventions. We aim to keep the codebase clean and consistent.
- **Write tests**: If your change introduces new functionality or fixes a bug, add appropriate tests where possible.
- **Update documentation**: If your change affects the way the program is used, update the `README.md` or any relevant documentation files.

## Reporting Issues

If you encounter a bug or unexpected behavior, feel free to open an issue in the GitHub Issues section. When creating an issue, please include:

- A clear and descriptive title.
- Steps to reproduce the issue (if applicable).
- Expected vs actual behavior.
- Environment details (e.g., operating system, Rust version, etc.).

## Code of Conduct

By participating in this project, you agree to abide by the [Code of Conduct](CODE_OF_CONDUCT.md). This includes:

- Respecting others' opinions and ideas.
- Being inclusive and welcoming to all participants.
- Not engaging in discriminatory, harassing, or offensive behavior.

## License

By contributing to this project, you agree that your contributions will be licensed under the MIT License, as stated in the [LICENSE](LICENSE) file.

---

Thank you for your interest in contributing to `nsp-cli`! We look forward to your contributions and improving the project together.
