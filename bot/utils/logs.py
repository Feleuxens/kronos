from logging import StreamHandler, Formatter, Logger, DEBUG, WARNING, getLogger
from sys import stdout

from utils.config import VERBOSITY

logging_handler = StreamHandler(stdout)
if VERBOSITY.upper() == "DEBUG":
    logging_handler.setFormatter(Formatter("[%(asctime)s] [%(module)s] [%(levelname)s] %(message)s"))
else:
    logging_handler.setFormatter(Formatter("[%(asctime)s] [%(levelname)s] %(message)s"))


def get_logger(name: str) -> Logger:
    logger: Logger = getLogger(name)
    logger.addHandler(logging_handler)
    logger.setLevel(VERBOSITY.upper())

    return logger
