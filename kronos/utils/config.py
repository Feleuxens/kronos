from os import environ, getenv


class Config:
    TOKEN: str
    VERBOSITY: str

    LOG_DATE_FORMAT: str
    VERSION: str = "0.1.0"
    REPOSITORY = "https://github.com/Feleuxens/Kronos"


Config.TOKEN = environ["BOT_TOKEN"]
Config.VERBOSITY = getenv("VERBOSITY", "INFO")
Config.LOG_DATE_FORMAT = getenv("LOG_DATE_FORMAT", "%Y-%m-%d %H:%M:%S")
