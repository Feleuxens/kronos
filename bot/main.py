from discord import Intents
from discord.ext.commands import Bot

from utils.config import TOKEN
from utils.logs import get_logger

logger = get_logger(__name__)

intents = Intents.all()
bot = Bot(command_prefix=".", intents=intents)


@bot.event
async def on_error(*_, **__):
    # sentry_sdk.capture_exception()
    raise


@bot.event
async def on_ready():
    logger.info(f"Logged in as {bot.user}\n")


@bot.command()
async def ping(ctx):
    await ctx.send('pong')


def main():
    bot.run(TOKEN)


if __name__ == "__main__":
    main()
