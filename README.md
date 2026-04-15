# discord-memleak-notifier (dmn)
## Introduction

Discord, the platform that most people probably use for chatting, is notoriously
unstable when it comes to features like streaming, voice chats, or really anything 
in general. Therefore, things like memory leaks are unfortunately very common.

Since the Discord developers have decided to approach this issue by [just restarting
the app if it uses too much memory](https://www.techspot.com/news/110542-discord-force-restarting-itself-windows-11-stop-eating.html),
I decided to come up with my own bandaid solution.

**discord-memleak-notifier** is a simple program for Linux and macOS which
continuously monitors the Discord process, and automatically sends a system
notification, if it detects that its CPU usage exceeds a certain threshhold.

---

## Usage
todo!

---

## Details

**dmn** supports monitoring the following Discord clients:
- Regular Discord binary
- Arch binary
- Canary client
- Vesktop

Most of the memory leaks (which I experienced) stem from trying to share-screen for
an extended amount of time at once, therefore it is recommended to just restart the
stream if this occurrs.

---

## Afterword
Although some users have found certain ways to resolve the issues regarding memory 
leaks, none of them seem to consistently fix it across all platforms or across clients. 
Therefore, I considered none of them real solutions and created this project. 
(which to be entirely fair, isn't a real solution itself)