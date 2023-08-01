from logging import StreamHandler, Formatter, Logger, DEBUG, WARNING, getLogger
from sys import stdout

from kronos.utils.config import Config

logging_handler = StreamHandler(stdout)
formatter = Formatter(
    "[{asctime}] [{levelname}] {name}: {message}", Config.LOG_DATE_FORMAT, style="{"
)
logging_handler.setFormatter(formatter)


def get_logger(name: str) -> Logger:
    logger: Logger = getLogger(name)
    logger.addHandler(logging_handler)
    logger.setLevel(Config.VERBOSITY.upper())

    return logger
