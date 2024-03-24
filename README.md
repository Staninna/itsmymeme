# itsmymeme

**itsmymeme** is a lightweight meme protection tool designed for Discord servers. It safeguards your cherished memes from being pilfered by unauthorized users. By scrutinizing the User-Agent of incoming requests, it can discern whether the request originates from Discord. If not, it promptly returns a 403 Forbidden error, ensuring your memes remain safe and sound.

## Installation

To install **itsmymeme**, simply follow these steps:

```bash
git clone https://github.com/Staninna/itsmymeme.git
cd itsmymeme
cargo build --release
```

## Usage

Once installed, launch **itsmymeme** with the following command:

```bash
./target/release/itsmymeme
```

## Configuration

**itsmymeme** offers flexibility through easy configuration. Customize its behavior by editing the `.env` file:

```env
FORBIDDEN="Unauthorized access! Keep your hands off my memes!"        # Customize the forbidden message (optional)
NOT_FOUND="Oops! This meme seems to have vanished! or never existed!" # Customize the not found message (optional)
PASSWORD="password"                                                   # Set a secure password for uploading memes
CONTENT_DIR="content"                                                 # Specify the directory where memes are stored relative to the binary (optional)
```

## API

### GET /:file

Retrieve a specific meme by its filename.

### POST/GET /upload

Securely upload your memes.

### GET /list

View a list of all available memes.

### Note

This route is not implemented yet.

## License

This project is licensed under the [LICENSE](LICENSE) agreement.
