# Project Structure

## New Structure
```
src/
├── commands/
│   ├── mod.rs
│   └── types.rs
├── config/
│   └── mod.rs
├── handlers/
│   └── mod.rs
├── ssh/
│   └── mod.rs
├── utils/
│   ├── mod.rs
│   └── encryption.rs
├── lib.rs
└── main.rs
```

## Changes Made

1. **Commands Module** - Separated command definitions from handlers:
   - `src/commands/types.rs` - Contains command enums and argument structs
   - `src/commands/mod.rs` - Exports the command types

2. **Handlers Module** - Moved all command handler functions to a dedicated module:
   - `src/handlers/mod.rs` - Contains all handler functions (`handle_add`, `handle_remove`, etc.)

3. **Config Module** - Moved to its own directory:
   - `src/config/mod.rs` - Configuration management

4. **SSH Module** - Moved to its own directory:
   - `src/ssh/mod.rs` - SSH connection functionality

5. **Utils Module** - Already well-structured:
   - `src/utils/mod.rs` - Main utils module
   - `src/utils/encryption.rs` - Encryption functionality

6. **Library Exports** - Updated `src/lib.rs` to export all modules:
   - Added `pub mod handlers;` export

7. **Main Application** - Updated to use the new structure:
   - Uses `ali_bastion::handlers` for handler functions
   - Uses `ali_bastion::commands::Commands` for command types

This restructuring provides better separation of concerns and makes the codebase more maintainable.