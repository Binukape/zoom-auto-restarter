# Zoom Auto-Restarter

A lightweight Rust-based tool to automatically restart Zoom meetings when the 40-minute limit is reached.

## ​Key Features:

1. ​Zero Browser Overhead: Uses the zoommtg:// protocol to launch meetings directly, bypassing the browser entirely for a faster rejoin.
2. ​Smart Monitoring: Automatically detects when a meeting ends by monitoring system processes and triggers an instant restart.

## How to Use

1. Download the `zoom-auto-restarter.exe` from the [Releases](../../releases/latest) tab.
2. Run the program.
3. Paste your Zoom link when prompted. (Use the link from a scheduled meeting with a longer time period.)
4. Keep the terminal open; it will handle the rest!
5. When you need to stop it, just close the window.

Important: USE A SCHEDULED MEETING LINK (WITH A LONGER TIME PERIOD) TO BYPASS THE 10 MINUTE COOLDOWN.

## Build from Source

If you prefer to compile the code yourself, follow our [Step-by-Step Build Guide](./BUILDING.md).

Disclaimer: This project is an independent open-source tool and is not affiliated with, authorized, maintained, sponsored, or endorsed by Zoom Communications, Inc.

_Created by Binuka Perera_
