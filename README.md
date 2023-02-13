# griptoe
Monorepo containing tools and scripts for managing crypto assets. Intended to be self-contained and exportable to arbitrary debian systems.

## Usage
This library is intended to be run from the inside of a fresh debian box. Run these scripts on your personal machine at your own peril

### Build an exportable set of binaries on a fresh machine
run
```
./griptoe build-full
```

### Setup
run the setup script
```
./griptoe system-setup
```

### Fetch and build external tools
run
```
./griptoe build-external
```

### Build tool for export
run
```
./griptoe package
```

