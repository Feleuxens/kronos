from os import environ, getenv

TOKEN: str = environ["BOT_TOKEN"]
VERBOSITY: str = getenv("VERBOSITY", "INFO")

SENTRY_DSN: str = getenv("SENTRY_DSN")
SENTRY_ENVIRONMENT: str = getenv("SENTRY_ENVIRONMENT", "dev")