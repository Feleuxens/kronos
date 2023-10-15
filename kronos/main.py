from discord import Intents
from aiohttp import ClientSession
from aiohttp.web import Application, Response, get, AppRunner
import asyncio

from kronos.utils.config import Config
from kronos.utils.logs import get_logger
from kronos.bot import Kronos

logger = get_logger(__name__)


async def healthz(request):
    return Response(status=200)

async def server_main():
    logger.debug("Preparing healthz endpoint")
    server = Application()
    server.add_routes([get("/healthz", healthz)])
    loop = asyncio.get_event_loop()
    runner = AppRunner(server)
    await runner.setup()
    listener = await loop.create_server(runner.server, "0.0.0.0", 8080)
    await listener.wait_closed()


async def bot_main():
    # needed for logging from the discord library
    _ = get_logger("discord")

    intents = Intents.all()
    extensions = ["kronos.modules.help"]
    bot = Kronos(command_prefix=".", intents=intents, initial_extensions=extensions)

    async with ClientSession():
        await bot.start(Config.TOKEN)


async def main():
    await asyncio.gather(bot_main(), server_main())


if __name__ == "__main__":
    asyncio.run(main())
