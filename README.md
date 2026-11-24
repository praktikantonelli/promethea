# Promethea

⚠️ This project is currently under heavy construction and will be completely revamped. 

Due to insufficient planning beforehand, the project has been saved under the `pre-alpha` tag and then reset to an empty repository. Until planning is finished, no code will be added here.

## Development
For development on Windows, install the necessary components manually, including
- the Rust toolchain for the backend with `cargo`, `clippy`, `rustfmt`, `rust-src` and `rust-analyzer` (as listed in [`rust-toolchain.toml`](./rust-toolchain.toml))
- `cocogitto` for commit linting (a fast and small alternative to `husky` and `commitlint` without needing the `nodejs` ecosystem)
- `just` for running recipes

On Unix systems, specifically NixOS, clone the repository, install `direnv` (or `nix-direnv` on NixOS), run `direnv allow` inside the repository and voilà!

## Commit Signing
This project uses signed commits with ssh. To set up:

```bash
# use ssh, not GPG
git config --global gpg.format ssh

# use the public ssh key to sign
git config --global user.signingkey ~/.ssh/id_ed25519.pub

# enable commit signing
git config --global commit.gpgSign true

# create a file, add your email address and the content of your public ssh key
touch ~/.ssh/allowedSignersFile

echo "email@example.com namespaces=\"git\"" <your-public-ssh-key> > ~/.ssh/allowedSignersFile

# tell git to use the file
git config --global gpg.ssh.allowedSignersFile ~/.ssh/allowedSignersFile

# verify it's working, commit something, then run

git verify-commit HEAD

# expected output
$ Good "git" signature for email@example.com with ED25519 key SHA256:<your-ssh-key>
```

Note that you also need to register your public ssh key as a GitHub *signing* key (as opposed to a regular ssh access key). 

## Shell
While technically not a hard requirement, the `justfile` hard-codes nushell as the shell environment. This is done because nushell is my shell of choice, and not having that option set in the `justfile` leads to problems when using `just` recipes in nushell.
