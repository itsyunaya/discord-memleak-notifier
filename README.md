# discord-memleak-notifier (dmn)
## Introduction

Discord, the platform that most people probably use for chatting, is notoriously
unstable when it comes to features like streaming, voice chats, or really anything 
in general. Therefore, things like memory leaks are unfortunately very common.

Since the Discord developers have decided to approach this issue by [just restarting
the app if it uses too much memory](https://www.techspot.com/news/110542-discord-force-restarting-itself-windows-11-stop-eating.html),
I decided to come up with my own bandaid solution.

**discord-memleak-notifier** (or **dmn**) is a simple program for Linux and macOS which
continuously monitors the Discord process, and automatically sends a system
notification, if it detects that its CPU usage exceeds a certain threshhold.

---

## Installation
### From source
```bash
cargo install --git https://github.com/itsyunaya/discord-memleak-notifier.git
# or
git clone https://github.com/itsyunaya/discord-memleak-notifier.git && cd discord-memleak-notifier
cargo install --path .
```

### Via Nix Flakes
Add the repository to your flake inputs
```nix
inputs = {
    dmn.url = "github:itsyunaya/discord-memleak-notifier";
};
```
Then in `configuration.nix`
```nix
environment.systemPackages = [
    inputs.dmn.packages.${pkgs.stdenv.hostPlatform.system}.default
];
```

---

## Usage

Set it to autostart on login through whatever method is most convenient, and you're 
done! You will now receive a desktop notification every time Discord is detected to 
use too many system resources

---

## Details

**dmn** supports monitoring the following Discord clients:
- Regular Discord binary
- Arch binary
- Canary client
- Vesktop

Most of the resource leaks (which I experienced) stem from trying to share-screen for
an extended amount of time at once, therefore it is recommended to just restart the
stream if this occurrs.

---

## Afterword
Although some users have found certain ways to resolve the issues regarding resource 
leaks, none of them seem to consistently fix it across all platforms or clients. 
Therefore, I considered none of them real solutions and created this project, which 
to be entirely fair, isn't a real solution itself, but rather what works best for
me personally.