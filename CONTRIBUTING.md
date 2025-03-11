# Contributing to nHale

Thank you for considering contributing to nHale! We welcome contributions from everyone, regardless of your level of experience.

## How to Contribute

### Development Process

We use GitHub to host code, track issues and feature requests, and accept pull requests.

### Pull Requests

1. Fork the repository and create your branch from `main`.
2. If you've added code that should be tested, add tests.
3. Ensure the test suite passes.
4. Update documentation as needed.
5. Make sure your code follows the project's style guide.

## Commit Message Guidelines

We follow the [Conventional Commits](https://www.conventionalcommits.org/) specification for our commit messages. This leads to more readable messages that are easy to follow when looking through the project history and enables automatic versioning.

### Commit Message Format

Each commit message consists of a **header**, a **body**, and a **footer**. The header has a special format that includes a **type**, a **scope**, and a **subject**:

```
<type>(<scope>): <subject>
<BLANK LINE>
<body>
<BLANK LINE>
<footer>
```

The **header** is mandatory, while the **scope**, **body**, and **footer** are optional.

### Type

Must be one of the following:

* **feat**: A new feature
* **fix**: A bug fix
* **docs**: Documentation only changes
* **style**: Changes that do not affect the meaning of the code (white-space, formatting, etc)
* **refactor**: A code change that neither fixes a bug nor adds a feature
* **perf**: A code change that improves performance
* **test**: Adding missing tests or correcting existing tests
* **chore**: Changes to the build process or auxiliary tools and libraries such as documentation generation

### Scope

The scope should be the name of the module affected (e.g., `embedding`, `extraction`, `crypto`, etc.).

### Subject

The subject contains a succinct description of the change:

* Use the imperative, present tense: "change" not "changed" nor "changes"
* Don't capitalize the first letter
* No dot (.) at the end

### Body

The body should include the motivation for the change and contrast this with previous behavior.

### Footer

The footer should contain any information about **Breaking Changes** and is also the place to reference GitHub issues that this commit **Closes**.

### Examples

```
feat(embedding): add new JPG steganography algorithm

Implement a new DCT-based steganography algorithm for JPG files that improves robustness against compression.

Closes #123
```

```
fix(extraction): resolve buffer overflow in PDF extraction

When extracting data from large PDF files, the buffer would overflow causing memory corruption.
This fix implements proper bounds checking and buffer allocation.

Fixes #456
```

## Versioning

nHale follows [Semantic Versioning 2.0.0](https://semver.org/). In summary:

- **MAJOR** version increments for incompatible API changes
- **MINOR** version increments for backward-compatible functionality additions
- **PATCH** version increments for backward-compatible bug fixes

## Getting Started

1. Fork the repository on GitHub
2. Clone your fork locally
   ```bash
   git clone https://github.com/YOUR-USERNAME/nhale.git
   cd nhale
   ```
3. Create a new branch for your feature or bugfix
   ```bash
   git checkout -b feat/your-feature-name
   ```
4. Make your changes and commit using the conventional commit format
5. Push your branch and submit a pull request

## License

By contributing, you agree that your contributions will be licensed under the project's MIT License. 