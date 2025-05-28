# QMeet

QMeet (Quick Meet) is a lightweight desktop app that runs in the background. It allows you to paste a Google Meet URL anywhere using a configurable keyboard shortcut.

## What problem does it solve

I have always had this problem: I text a colleague on WhatsApp about a group assignment. Then one of us asks the other to send a Google Meet link to discuss the matter further. Then I need to go through this dance:

- Leave WhatsApp & open the browser
- Go to `meet.google.com`
- Create a new meeting
- Copy the link
- Open WhatsApp again
- Paste the link and send it
- Go back to the browser to join the meeting

Only if there were a tool that could generate a Google Meet link and paste it using a shortcut. Oh, wait, I am a developer! I will make such a tool and call it QMeet, it shouldn't take an hour (I was lying).

## Features

- System tray integration
- Customizable global keyboard shortcuts
- Auto-start on system boot
- Cross-platform support (Windows, macOS, Linux)

## Tech Stack

**Frontend:**
- Next.js 15.3.2
- React 19
- TypeScript
- Tailwind CSS

**Backend:**
- Rust
- Tauri 2.5.1

## Development

```bash
# Install dependencies
npm install

# Run Tauri app
npm run tauri dev

# Build for production
npm run tauri build
```
