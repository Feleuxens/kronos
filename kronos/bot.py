from typing import List
from discord.ext.commands import Bot

from kronos.utils.logs import get_logger


class Kronos(Bot):
    def __init__(self, *args, initial_extensions: List[str], **kwargs):
        super().__init__(*args, **kwargs)
        self.initial_extensions = initial_extensions
        self.logger = get_logger(__name__)

    async def setup_hook(self) -> None:
        for extension in self.initial_extensions:
            await self.load_extension(extension)

        await self.tree.sync()

    async def on_ready(self):
        self.logger.info(f"Logged in as {self.user}\n")

    async def on_error(self, event: str, /, *args, **kwargs):
        # sentry_sdk.capture_exception()
        self.logger.error(event)
        raise
