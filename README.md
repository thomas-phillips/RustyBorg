# RustyBorg

A tool for *simplifying* BorgBackup for general backup use.

## Features
- Initialise Borg repository.
- Create archives with custom name or epoch time.
- Verify connection with SSH Server to validate SSH connection before BorgBackup.
- List repository details.
- Scheduling archive creation with CRON expressions.
- Daemonize binary when scheduling archive creation.

## Usage
### Initialise repository
```bash
rusty_borg init "<REPOSITORY_DIRECTORY>" "<PASSPHRASE>"
```

### Create Archive
```bash
rusty_borg create "<REPOSITORY_DIRECTORY>" -p "<PASSPHRASE>" -a "<ARCHIVE_NAME|OPTIONAL>" --paths ... --include_patterns ... --exclude_patterns ...
```

### List Repository Details
```bash
rusty_borg list "<REPOSITORY_DIRECTORY>" "<PASSPHRASE>" --last-modified --encryption --archives
```

### Verify SSH Connection
```bash
rusty_borg verify "<USER>" "<HOST>" --port "<PORT>"
```

### Schedule Archive Creation
```bash
rusty_borg schedule --daemonize --verbose --expression "<CRON_EXPRESSION>" --timezone "<TIMEZONE>" --repository "<REPOSITORY_DIRECTORY" --passphrase "<PASSPHRASE>" --archive "<ARCHIVE_NAME|OPTIONAL>" --paths ... --include-patterns ... --exclude-patterns ...
```

## Why does this exist?
For the past year and a half I have been building and working on my own homelab server to improve my developer and DevOps skills. Over time the homelab has seen adoption from various people. Due to this I have been researching many backup methods I can use to create a good and reliable `3 2 1` backup solution.

The backup solutions I have seen are good, reliable and well tested but unfortunately don't fit how I would want my backup system to work. I could get a solution to fit my needs but it felt like pushing a square through a circle hole. Being a primarily Linux user I gravitated to BorgBackup but I found the CLI cumbersome to use (especially for automation... see initial attempt at this: [BorgBackupDocker](https://github.com/thomas-phillips/BorgBackupDocker)).

My initial implementation has been annoying me for a while as I was not confident in the reliability and repeatability of my script of BorgBackup, so I began thinking of a solution. I finally decided on using Rust to create a simple wrapper around BorgBackup to help make the tool bearable to use for automation... and then I got busy with work so that had to wait a few months.

Fast forward to December 2024, I'm on leave and now have the time to create my ideal backup solution. I kept with my initial plan to use Rust for the memory safety, error handling and speed it provides plus it'll be a great opportunity to learn the language as I've been putting it off for a while. I discovered an amazing Rust crate that wraps BorgBackup by **myOmikron** which has proved essential for getting this project off the ground, my gratitude to you.

This project has become a bit *bigger* than I expected but I still have more plans for this tool.

### Plans
- Remove any use of `.unwrap()` so that errors can be properly handled
- Create Unraid Docker app template so that this can be installed from the Unraid UI.
- Improve testing for using this tool on the CLI.
- Multiple archives per schedule instead of just the one.
- Allow for no passphrase.
- Async archive creation so a big archive won't prevent others from being created.

