# MHDB (Monster Hunter Database)

MHDB is a local, open-source database browser for the Monster Hunter series, built with modern web technologies and Rust. It provides a fast, offline-capable interface to browse monsters, items, quests, and weapons across multiple Monster Hunter titles.

==CURRENTLY IN PRE-ALPHA DEVELOPMENT==

## Features

- **Multi-Game Support**: Architecture designed to support databases from various Monster Hunter generations.
- **Local & Offline**: Runs locally on your machine purely offline.
- **Fast & Lightweight**: Built with Tauri and Rust for native performance with a small footprint.

## Tech Stack

- **Frontend**: [React](https://reactjs.org/), [TypeScript](https://www.typescriptlang.org/), [Vite](https://vitejs.dev/)
- **Backend**: [Rust](https://www.rust-lang.org/), [Tauri v2](https://tauri.app/)

## Getting Started

### Prerequisites

- [Node.js](https://nodejs.org/) (Project uses [pnpm](https://pnpm.io/))
- [Rust](https://www.rust-lang.org/tools/install)

### Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/yourusername/mhdb.git
   cd mhdb
   ```

2. Install frontend dependencies:

   ```bash
   pnpm install
   ```

3. Run the development server:

   ```bash
   pnpm tauri dev
   ```

   This command will start the frontend server and launch the Tauri application window.

4. Build the project:

    ```bash
    pnpm tauri build
    ```
