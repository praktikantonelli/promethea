# Promethea

⚠️ This project is currently under heavy construction and will be completely revamped. 

Due to insufficient planning beforehand, the project has been saved under the `pre-alpha` tag and then reset to an empty repository. Until planning is finished, no code will be added here.

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

## Shared Types
Because the backend and frontend are written in different languages but may need to know about the same types (e.g., backend responds with a `CustomType` to an HTTP GET), shared types are auto-generated using [`ts-rs`](https://github.com/aleph-alpha/ts-rs). To generated the bindings, `ts-rs` generates a Cargo test that generates TypeScript types into [`frontend/packages/lib/src/bindings`](./frontend/packages/lib/src/bindings/) where each type gets written into its own `.ts` file with the same name as the type. Related types are automatically imported too.

To update the types, run
```sh
just test backend
```

This is a `just` recipe that runs all backend tests, regardless of CWD. 
