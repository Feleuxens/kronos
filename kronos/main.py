from discord import Intents
from aiohttp import ClientSession
import asyncio

from kronos.utils.config import Config
from kronos.utils.logs import get_logger
from kronos.bot import Kronos

logger = get_logger(__name__)


async def main():
    # needed for logging from the discord library
    _ = get_logger("discord")

    intents = Intents.all()
    extensions = ["kronos.modules.help"]
    bot = Kronos(command_prefix=".", intents=intents, initial_extensions=extensions)

    async with ClientSession():
        await bot.start(Config.TOKEN)


if __name__ == "__main__":
    asyncio.run(main())
