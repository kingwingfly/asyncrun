<a name="readme-top"></a>

<!--
*** This README is modified from https://github.com/othneildrew/Best-README-Template
-->

<!-- PROJECT SHIELDS -->
[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]


<!-- PROJECT LOGO -->
<br />
<div align="center">
  <a href="https://github.com/kingwingfly/asyncrun">
    <img src="images/logo.png" alt="Logo" width="80" height="80">
  </a>

<h3 align="center">async_run</h3>

  <p align="center">
    Asynchronously execute shell commands in filtered subdirectories.
    <br />
    <a href="https://github.com/kingwingfly/asyncrun"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <a href="https://github.com/kingwingfly/asyncrun">View Demo</a>
    ·
    <a href="https://github.com/kingwingfly/asyncrun/issues/new?labels=bug&template=bug-report---.md">Report Bug</a>
    ·
    <a href="https://github.com/kingwingfly/asyncrun/issues/new?labels=enhancement&template=feature-request---.md">Request Feature</a>
  </p>
</div>



<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#built-with">Built With</a></li>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#roadmap">Roadmap</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
    <li><a href="#acknowledgments">Acknowledgments</a></li>
  </ol>
</details>



<!-- ABOUT THE PROJECT -->
## About The Project

[![Product Name Screen Shot][product-screenshot]](https://github.com/kingwingfly/asyncrun)

Do things like `find . -type d -name ".git" | xargs -I _ -P 64 sh -c "cd _/.. && git pull --rebase &> /dev/tty"`
easier if you're not a unix-pro guy.

Instead, just `asyncrun -e .git -- "git pull --rebase"`.


<p align="right">(<a href="#readme-top">back to top</a>)</p>



### Built With

* [![Rust][Rust]][Rust-url]

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- GETTING STARTED -->
## Getting Started

Download at [release](https://github.com/kingwingfly/asyncrun/releases) page. Or
```sh
cargo install asyncrun
```
Or compile yourself:
### Prerequisites

* Install Rust
  ```sh
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

### Compilation

1. Clone the repo
   ```sh
   git clone https://github.com/kingwingfly/asyncrun.git
   ```
2. Compilation
   ```sh
   cargo build --release
   ```

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- USAGE EXAMPLES -->
## Usage

```sh
Asynchronously execute shell commands in filtered subdirectories

Usage: asyncrun [OPTIONS] [PATH] -- <COMMANDS>...

Arguments:
  [PATH]         Path to search for directories [default: /Users/louis/rust/asyncrun]
  <COMMANDS>...  Commands to run in each directory

Options:
  -e <EXIST>          Filter by file/dir names all exist
  -E <NOT_EXIST>      Filter by file/dir names all non-exist
  -n <ASYNC_NUM>      Number of async commands to run concurrently [default: 1024]
  -h, --help          Print help
  -V, --version       Print version
```

### Example

```sh
asyncrun -e .git -- "git fetch -p" "git pull --rebase" "git gc"
asyncrun -e .git -e Cargo.toml -- "cargo update" "git add ." "git commit -m 'update deps'" "git push"
asyncrun -e Cargo.toml -e target -- "cargo clean"
asyncrun -e .git -e .DS_Store -e .gitignore -- "git pull --rebase" "rm .DS_Store" "git add ." "git commit -m 'rm .DS_Store'" "grep -qxF '.DS_Store' .gitignore || echo '.DS_Store' >> .gitignore" "git add ." "git commit -m 'modified .gitignore'" "git push" | tee output.txt
```

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- ROADMAP -->
## Roadmap

- [ ] Feature

See the [open issues](https://github.com/kingwingfly/asyncrun/issues) for a full list of proposed features (and known issues).

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- CONTRIBUTING -->
## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- LICENSE -->
## License

Distributed under the MIT License. See `LICENSE.txt` for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- CONTACT -->
## Contact

Louis - 836250617@qq.com

Project Link: [https://github.com/kingwingfly/asyncrun](https://github.com/kingwingfly/asyncrun)

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- ACKNOWLEDGMENTS -->
## Acknowledgments

* []()

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[contributors-shield]: https://img.shields.io/github/contributors/kingwingfly/asyncrun.svg?style=for-the-badge
[contributors-url]: https://github.com/kingwingfly/asyncrun/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/kingwingfly/asyncrun.svg?style=for-the-badge
[forks-url]: https://github.com/kingwingfly/asyncrun/network/members
[stars-shield]: https://img.shields.io/github/stars/kingwingfly/asyncrun.svg?style=for-the-badge
[stars-url]: https://github.com/kingwingfly/asyncrun/stargazers
[issues-shield]: https://img.shields.io/github/issues/kingwingfly/asyncrun.svg?style=for-the-badge
[issues-url]: https://github.com/kingwingfly/asyncrun/issues
[license-shield]: https://img.shields.io/github/license/kingwingfly/asyncrun.svg?style=for-the-badge
[license-url]: https://github.com/kingwingfly/asyncrun/blob/master/LICENSE.txt
[product-screenshot]: images/screenshot.png
[Rust]: https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=Rust&logoColor=orange
[Rust-url]: https://www.rust-lang.org
