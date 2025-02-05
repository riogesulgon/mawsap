# Mawsap - Mouse Jiggler Utility

Mawsap is a simple Rust-based mouse jiggler utility that prevents system sleep/lock by moving the mouse cursor. Press ESC to exit the program.

## Linux Build Instructions

### Dependencies
1. Rust (install via [rustup](https://rustup.rs/))
2. xdotool and libxdo-dev

### Installation Steps
Clone the repository:

```bash
git clone git@github.com:riogesulgon/mawsap.git
```

1. Install Rust (if not already installed):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Linux**: Install required system packages:
   ```bash
   sudo apt-get install -y xdotool libxdo-dev
   ```

(This step is not needed in mac os)

3. Build and run the project:
   ```bash
   cargo build
   cargo run
   ```
4. Install
   ```bash
   cargo install --path=.
   ```
   
### Usage
- Run the program with `mawsap`
- The mouse will start jiggling automatically
- Press ESC to exit the program
