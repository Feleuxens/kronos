# Kronos

A small Discord Bot including features like roles via component buttons and user verficiation.

## Development

### Requirements

- [Rust](https://www.rust-lang.org/) (I'm using the most recent version, it might work on older versions but current I can't give a MSRV)
- [Git](https://git-scm.com/)
- [Discord API Token](https://discord.com/developers/)
- [MongoDB](https://www.mongodb.com/) (Currenty, selfhosting the db is not supported but will be soon.)

### Running the bot

To run the bot, you have to clone the repository and put you environment variables in a file name `.env` (see `env.sample`) or set them as environment variables. Then you can use

```
  cargo run --release
```
  
to download all dependencies and compile them.


## Environment variables

| Variable | Description | Required? |
| :--- | :--- | :--- |
| TOKEN | Discord Bot Token | :heavy_check_mark: |
| VERBOSITY | Verbosity of console output. Possible values are `trace`, `debug`, `info`, `warn` or `error`. | :heavy_check_mark: |
| | | | |
| MONGO_USER | Username to use to access the db. | :heavy_check_mark: |
| MONGO_PASSWORD | Password of the user for the db. | :heavy_check_mark: |
| MONGO_CLUSTER | Name of the cluster. | :heavy_check_mark: |
| DATABASE_NAME | Name of the db that will be created in the cluster. | :heavy_check_mark: |
