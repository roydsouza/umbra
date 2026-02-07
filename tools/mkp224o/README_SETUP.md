# mkp224o - Vanity Onion Address Generator

`mkp224o` is a vanity address generator for Tor onion v3 (ed25519) services. It allows you to generate `.onion` addresses that start with a specific prefix (e.g., `umbra...onion`).

## 1. Description

Tor v3 onion addresses are derived from ed25519 public keys. They are 56 characters long and end in `.onion`. `mkp224o` solves the cryptographic puzzle to find keys that match your desired prefix by brute-forcing millions of keys per second.

It resides in: `~/antigravity/umbra/mkp224o`
**Binary Location**: `~/bin/mkp224o`

## 2. Secure Key Storage

> [!IMPORTANT]
> **Recommended Storage Location**: `~/antigravity/umbra/keys`
> This location is **git-ignored**, ensuring your sensitive private keys never accidentally leave your machine.

**Usage Command**:
```bash
~/bin/mkp224o -d ~/antigravity/umbra/keys <my-prefix>
```

## 3. Installation & Build

### Prerequisites
Ensure you have the necessary build tools and libraries installed via Homebrew:

```bash
brew install libsodium autoconf automake pcre2
```

### Build Instructions
The project is a git submodule in `umbra/mkp224o`. To build it:

1.  Initialize and update the submodule:
    ```bash
    cd ~/antigravity/umbra
    git submodule update --init --recursive mkp224o
    cd mkp224o
    ```

2.  Generate the configuration script:
    ```bash
    ./autogen.sh
    ```

3.  Configure the build (optimized for Apple Silicon):
    ```bash
    ./configure --enable-intfilter=native --enable-binsearch
    ```
    *   `--enable-intfilter=native`: Optimizes filtering for the M5 processor (64-bit).
    *   `--enable-binsearch`: Speeds up lookups if you provide many filters.

4.  Compile:
    ```bash
    make
    ```

This will produce the `mkp224o` binary in the directory. You should move it to your path:
```bash
mv mkp224o ~/bin/
```

## 4. Usage

### Basic Generation
To generate an onion address starting with "umbra":

```bash
~/bin/mkp224o umbra
```

This will run indefinitely, printing keys to the console as they are found.

### Saving to Directory (Recommended)
To prevent clutter and save keys neatly:

```bash
~/bin/mkp224o -d keys umbra
```
*   `-d keys`: Output found keys to the `keys/` directory.
*   `umbra`: The prefix you are looking for.

### Batch Processing
You can search for multiple prefixes at once:

```bash
~/bin/mkp224o -d keys umbra gravity darkmatter
```

### Statistics
To see how fast your M5 chip is generating keys:

```bash
~/bin/mkp224o -s umbra
```

## 5. Integration with Arti/Tor

Once you have a generated key directory (e.g., `keys/umbra...onion/`):

1.  It will contain:
    *   `hs_ed25519_secret_key` (The critical private key)
    *   `hs_ed25519_public_key`
    *   `hostname`

2.  **For C-Tor**: Copy the contents to your hidden service directory (e.g., `/var/lib/tor/myservice/`).
3.  **For Arti**: Arti currently uses a different key format roughly compatible with C-Tor but requires specific configuration. Refer to `ARTI.md` for details on configuring onion services in Arti.

## 6. Updates

To update `mkp224o` to the latest version:

1.  Pull the latest changes:
    ```bash
    cd ~/antigravity/umbra
    git submodule update --remote mkp224o
    cd mkp224o
    ```

2.  Clean and Rebuild:
    ```bash
    make clean
    ./autogen.sh
    ./configure --enable-intfilter=native --enable-binsearch
    make
    ```
