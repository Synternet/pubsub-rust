<!-- omit in toc -->
# Contributing to Syntropy PubSub Rust 

First off, thanks for taking the time to contribute! ❤️

All types of contributions are encouraged and valued. See the [Table of Contents](#table-of-contents) for different ways to help and details about how this project handles them. Please make sure to read the relevant section before making your contribution. It will make it a lot easier for us maintainers and smooth out the experience for all involved. The community looks forward to your contributions. 🎉

> And if you like the project, but just don't have time to contribute, that's fine. There are other easy ways to support the project and show your appreciation, which we would also be very happy about:
> - Star the project
> - Tweet about it
> - Refer this project in your project's readme
> - Mention the project at local meetups and tell your friends/colleagues

<!-- omit in toc -->
## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [I Have a Question](#i-have-a-question)
- [I Want To Contribute](#i-want-to-contribute)
  - [Reporting Bugs](#reporting-bugs)
  - [Suggesting Enhancements](#suggesting-enhancements)
  - [Your First Code Contribution](#your-first-code-contribution)
  - [Improving The Documentation](#improving-the-documentation)
- [Styleguides](#styleguides)
  - [Commit Messages](#commit-messages)
- [Join The Project Team](#join-the-project-team)


## Code of Conduct

This project and everyone participating in it is governed by the
[Syntropy PubSub Rust Code of Conduct](https://github.com/SyntropyNet/pubsub-rustblob/master/CODE_OF_CONDUCT.md).
By participating, you are expected to uphold this code. Please report unacceptable behavior
to <devrel@syntropynet.com>.


## I Have a Question

> If you want to ask a question, we assume that you have read the available [Documentation](https://docs.syntropynet.com/docs).

Before you ask a question, it is best to search for existing [Issues](https://github.com/SyntropyNet/pubsub-rust/issues) that might help you. In case you have found a suitable issue and still need clarification, you can write your question in this issue. It is also advisable to search the internet for answers first.

If you then still feel the need to ask a question and need clarification, we recommend the following:

- Open an [Issue](https://github.com/SyntropyNet/pubsub-rust/issues/new).
- Provide as much context as you can about what you're running into.
- Provide project and platform versions (nodejs, npm, etc), depending on what seems relevant.

We will then take care of the issue as soon as possible.


## I Want To Contribute

> ### Legal Notice <!-- omit in toc -->
> When contributing to this project, you must agree that you have authored 100% of the content, that you have the necessary rights to the content and that the content you contribute may be provided under the project license.

### Reporting Bugs

<!-- omit in toc -->
#### Before Submitting a Bug Report

A good bug report shouldn't leave others needing to chase you up for more information. Therefore, we ask you to investigate carefully, collect information and describe the issue in detail in your report. Please complete the following steps in advance to help us fix any potential bug as fast as possible.

- Make sure that you are using the latest version.
- Determine if your bug is really a bug and not an error on your side e.g. using incompatible environment components/versions (Make sure that you have read the [documentation](https://docs.syntropynet.com/docs). If you are looking for support, you might want to check [this section](#i-have-a-question)).
- To see if other users have experienced (and potentially already solved) the same issue you are having, check if there is not already a bug report existing for your bug or error in the [bug tracker](https://github.com/SyntropyNet/pubsub-rust/issues?q=label%3Abug).
- Also make sure to search the internet (including Stack Overflow) to see if users outside of the GitHub community have discussed the issue.
- Collect information about the bug:
  - Stack trace (Traceback)
  - OS, Platform and Version (Windows, Linux, macOS, x86, ARM)
  - Version of the interpreter, compiler, SDK, runtime environment, package manager, depending on what seems relevant.
  - Possibly your input and the output
  - Can you reliably reproduce the issue? And can you also reproduce it with older versions?

<!-- omit in toc -->
#### How Do I Submit a Good Bug Report?

> You must never report security related issues, vulnerabilities or bugs including sensitive information to the issue tracker, or elsewhere in public. Instead sensitive bugs must be sent by email to <devops@syntropynet.com>.


We use GitHub issues to track bugs and errors. If you run into an issue with the project:

- Open an [Issue](https://github.com/SyntropyNet/pubsub-rust/issues/new). (Since we can't be sure at this point whether it is a bug or not, we ask you not to talk about a bug yet and not to label the issue.)
- Explain the behavior you would expect and the actual behavior.
- Please provide as much context as possible and describe the *reproduction steps* that someone else can follow to recreate the issue on their own. This usually includes your code. For good bug reports you should isolate the problem and create a reduced test case.
- Provide the information you collected in the previous section.

Once it's filed:

- The project team will label the issue accordingly.
- A team member will try to reproduce the issue with your provided steps. If there are no reproduction steps or no obvious way to reproduce the issue, the team will ask you for those steps and mark the issue as `needs-repro`. Bugs with the `needs-repro` tag will not be addressed until they are reproduced.
- If the team is able to reproduce the issue, it will be marked `needs-fix`, as well as possibly other tags (such as `critical`), and the issue will be left to be [implemented by someone](#your-first-code-contribution).



### Suggesting Enhancements

This section guides you through submitting an enhancement suggestion for Syntropy PubSub Rust, **including completely new features and minor improvements to existing functionality**. Following these guidelines will help maintainers and the community to understand your suggestion and find related suggestions.

<!-- omit in toc -->
#### Before Submitting an Enhancement

- Make sure that you are using the latest version.
- Read the [documentation](https://docs.syntropynet.com/docs) carefully and find out if the functionality is already covered, maybe by an individual configuration.
- Perform a [search](https://github.com/SyntropyNet/pubsub-rust/issues) to see if the enhancement has already been suggested. If it has, add a comment to the existing issue instead of opening a new one.
- Find out whether your idea fits with the scope and aims of the project. It's up to you to make a strong case to convince the project's developers of the merits of this feature. Keep in mind that we want features that will be useful to the majority of our users and not just a small subset. If you're just targeting a minority of users, consider writing an add-on/plugin library.

<!-- omit in toc -->
#### How Do I Submit a Good Enhancement Suggestion?

Enhancement suggestions are tracked as [GitHub issues](https://github.com/SyntropyNet/pubsub-rust/issues).

- Use a **clear and descriptive title** for the issue to identify the suggestion.
- Provide a **step-by-step description of the suggested enhancement** in as many details as possible.
- **Describe the current behavior** and **explain which behavior you expected to see instead** and why. At this point you can also tell which alternatives do not work for you.
- You may want to **include screenshots and animated GIFs** which help you demonstrate the steps or point out the part which the suggestion is related to. You can use [this tool](https://www.cockos.com/licecap/) to record GIFs on macOS and Windows, and [this tool](https://github.com/colinkeenan/silentcast) or [this tool](https://github.com/GNOME/byzanz) on Linux. <!-- this should only be included if the project has a GUI -->
- **Explain why this enhancement would be useful** to most Syntropy PubSub Rust users. You may also want to point out the other projects that solved it better and which could serve as inspiration.

<!-- You might want to create an issue template for enhancement suggestions that can be used as a guide and that defines the structure of the information to be included. If you do so, reference it here in the description. -->

### Your First Code Contribution
Once you have set up your development environment and IDE, it's time to start coding! Follow these steps to get started:

1. Select an Issue: Visit our issue tracker on GitHub and find an open issue that you would like to work on. Make sure to read the issue description, requirements, and any discussions related to it.

2. Create a New Branch: Before making any changes, create a new branch for your work. Use a descriptive branch name that relates to the issue you are addressing.

3. Start Coding: Now it's time to write code! Follow the project's coding conventions and style guidelines. Write clean, readable, and well-documented code.

4. Test Your Changes: Test your changes thoroughly to ensure they work as expected. Run any relevant tests and verify that the existing test suite passes.

5. Commit and Push: Once you are satisfied with your changes, commit them with a clear and descriptive commit message. Push your changes to your forked repository.

6. Create a Pull Request: Go to our project's repository on GitHub and click the "New Pull Request" button. Fill in the necessary details, including the branch you worked on and a description of your changes. Submit the pull request for review.

### Improving The Documentation
Updating, improving, and correcting the documentation is crucial for maintaining a clear and helpful resource for users and contributors. If you'd like to contribute to the documentation, here's a short guide to get you started:

1. Identify areas for improvement: Review the existing documentation and identify sections that may require updates, clarifications, or corrections. Look for outdated information, unclear instructions, or missing details.

2. Make changes locally: Clone the repository to your local development environment. Locate the documentation files that need modification. You can use a text editor or integrated development environment (IDE) to make the necessary changes.

3. Follow the documentation style guide: Maintain consistency by following the established documentation style guide. Pay attention to formatting, grammar, and syntax to ensure a cohesive and professional appearance.

4. Test your changes: If applicable, test any code snippets or examples provided in the documentation. Ensure they are accurate and work as intended. Verify that the updated documentation reflects the expected behavior.

5. Submitting your changes: Once you are satisfied with your changes, commit them to your local repository. Push your changes to a new branch and create a pull request (PR) to propose your modifications. Provide a clear and concise description of the changes you made.

6. Review and feedback: The maintainers and contributors will review your PR. They may provide feedback or request revisions. Be responsive to any comments and make the necessary adjustments as requested.

7. Merge and publish: Once your PR is approved, it will be merged into the main documentation repository. Your changes will then become part of the official documentation. Congratulations on your contribution!

Remember, documentation is a collaborative effort, and your contributions help improve the overall quality and usefulness of the resource. Your attention to detail and dedication to accurate and up-to-date documentation are greatly appreciated by the community.

## Styleguides
### Commit Messages

Consistent and descriptive commit messages are important for maintaining a clean and understandable version history of our project. To ensure consistency across commits, we follow the guidelines outlined in this commit style guide.

#### Commit Message Format

Each commit message should consist of a concise summary followed by an optional detailed description. The summary should be descriptive yet concise, providing a clear idea of the changes made. If necessary, the detailed description can provide additional context, explanations, or any relevant information.

## Commit Message Format

We follow a standardized commit message format to ensure consistency and clarity in our Git commit history. Please adhere to the following guidelines when writing commit messages:

- Use present tense and imperative mood (e.g., "Add feature" instead of "Added feature").
- Limit the first line to 50 characters or less.
- Separate the subject from the body with a blank line.
- Start the commit message subject with a capitalized verb.
- Use the body of the commit message to provide more detailed information if necessary.

### Commit Template

To make it easier to write commit messages, we provide a commit template that you can use. The template includes the following sections:

```
### Commit Template

[Short Description]

[Optional: Longer Description]

[Optional: Issue/Task/Feature ID]

```

- `[Short Description]`: This is where you provide a concise summary of the changes made in the commit. Keep it brief and descriptive.
- `[Optional: Longer Description]`: If needed, you can add a more detailed description of the changes made in the commit.
- `[Optional: Issue/Task/Feature ID]`: If your project uses an issue tracking system or has specific task or feature IDs, you can include them here.

Feel free to modify this template according to your preferences and project requirements.

To use the commit template, copy the content from the [commit_template.md](commit_template.md) file and save it in your local repository as `commit_template.md`. Then, follow the steps provided earlier to set it up in your repository.

By following this commit message format and using the commit template, we can have a well-structured and informative commit history.

#### Commit Guidelines

To maintain clarity and consistency, follow these guidelines when crafting commit messages:

- Use present tense in the summary: Write the summary in the present tense, as if describing the commit's effects after being applied.

- Be specific and descriptive: Make sure the summary provides enough information to understand the changes at a glance. Avoid vague or ambiguous wording.

- Keep commits focused: Each commit should represent a single logical change. If you have multiple unrelated changes, it's better to split them into separate commits.

- Reference relevant issues: If your commit relates to a specific issue or task, include the issue number in the commit message. For example, `Fixes #123` or `Closes #456`.

- Use imperative mood: Start the summary with an imperative verb that describes what the commit does. For example, `Add feature X`, `Fix bug Y`, or `Refactor component Z`.

#### Example

Certainly! Here are a few examples of commit messages using the short commit template:

Example 1:
```

Fix typo in README.md
```

Example 2:
```

Update CSS styles for homepage

- Adjusted font sizes and colors
- Fixed alignment issues
```

Example 3:
```

Implement user registration functionality

- Added registration form and validation
- Stored user details in the database
- Created registration success page
```

By following these commit style guidelines, we can maintain a clean and informative commit history, making it easier for team members and future contributors to understand and navigate the project's development process.

### Pull Request Guidelines
Certainly! Here's an example section you can add to your contribution guidelines to explain the Pull Request (PR) template:

## Pull Request Template

We highly encourage contributors to use a PR template when submitting pull requests. A PR template helps provide structured and consistent information about the changes being proposed. This ensures that reviewers have all the necessary details to understand and evaluate the PR effectively.

To create a new PR, please follow these steps:

1. Fork the repository and create a new branch for your changes.
2. Make your desired changes to the codebase.
3. Before submitting the PR, ensure that your branch is up to date with the latest changes from the upstream repository.
4. Create a new pull request on the repository.
5. Provide a meaningful title and description for your pull request. Include a clear and concise summary of the changes made, along with any relevant context or motivation.
6. Fill out the PR template with the requested information.
#### PR Template
```
## Description
Please provide a brief description of the changes made in this PR.

## Related Issue
Closes #[issue number]

## Proposed Changes
List the specific changes or additions made in this PR.

- Change 1
- Change 2
- ...

## Additional Info
Add any additional information or context that may be helpful for reviewers.

## Checklist
- [ ] I have tested these changes locally
- [ ] I have reviewed the code for any potential issues
- [ ] I have documented any necessary changes
```


Please ensure that you fill out the PR template with the necessary information and check off the relevant checklist items before submitting the PR.

Thank you for your contributions and for using the PR template to provide clear and well-documented pull requests. It helps streamline the review process and ensures that your changes can be effectively evaluated.

#### Example

```
## Description
This PR updates the homepage layout and adds a new feature for user authentication.

## Related Issue
Closes #42

## Proposed Changes
- Updated the CSS styles for the homepage to improve responsiveness.
- Added a login form to the homepage for user authentication.
- Implemented backend API endpoints for user authentication.

## Additional Info
The updated homepage layout provides a better user experience on mobile devices and ensures consistent styling across different screen sizes. The new login form allows users to securely authenticate and access personalized features. The backend API endpoints handle user login requests, validate credentials, and return authentication tokens for subsequent API calls.

## Checklist
- [x] I have tested these changes locally and verified that the homepage layout is responsive and the login form functions as expected.
- [x] I have reviewed the code for any potential issues and ensured that it follows the project's coding conventions.
- [x] I have documented the necessary changes in the project's documentation, including the new API endpoints and any updated usage instructions.

Please review this pull request and provide any feedback or suggestions for improvement. Thank you!
```
## Join The Project Team

We welcome you to join our project team and be a part of our exciting journey in revolutionizing data availability. If you have any questions, ideas, or simply want to connect with us, we encourage you to reach out through any of the following channels:

- **Discord**: Join our vibrant community on Discord at [https://discord.com/invite/jqZur5S3KZ](https://discord.com/invite/jqZur5S3KZ). Engage in discussions, seek assistance, and collaborate with like-minded individuals.

- **Telegram**: Connect with us on Telegram at [https://t.me/SyntropyNet](https://t.me/SyntropyNet). Stay updated with the latest news, announcements, and interact with our team members and community.

- **Email**: If you prefer email communication, feel free to reach out to us at devrel@syntropynet.com. We're here to address your inquiries, provide support, and explore collaboration opportunities.

We value your contributions and look forward to having you as a part of our project team. Together, we can shape the future of data availability and create innovative solutions that empower the Web3 ecosystem.

<!-- omit in toc -->
## Attribution
This guide is based on the **contributing-gen**. [Make your own](https://github.com/bttger/contributing-gen)!
