from discord import app_commands, Interaction, Embed, Status, Member
from discord.app_commands import ContextMenu, Command, Group
from discord.ext.commands import Cog, Bot
from typing import Optional, List, Union

from kronos.utils.colors import Colors
from kronos.utils.config import Config
from kronos.utils.logs import get_logger

logger = get_logger(__name__)


async def setup(bot: Bot):
    bot.remove_command("help")
    await bot.add_cog(HelpCog(bot))


class HelpCog(Cog):
    @app_commands.command(name="help", description="Displays this help message")
    async def _help(self, interaction: Interaction, command: Optional[str]) -> None:
        """
        Displays this help message
        :param interaction: The interaction payload
        :param command: Cog or command to get help
        :return: None
        """
        embed: Embed
        if command:
            app_command = interaction.client.tree.get_command(command)
            if len(app_command.parameters) == 0:
                description = app_command.description
            else:
                description = f"{app_command.description}\n\nParameters:\n" + "\n".join(
                    f":small_blue_diamond: {parameter.name} - {parameter.description}"
                    for parameter in app_command.parameters
                )
            if app_command:
                embed = Embed(
                    title=app_command.name,
                    description=description,
                    color=Colors.GREEN,
                )
            else:
                embed = Embed(
                    title="Help",
                    description=f"Command `{command}` does not exist.",
                    color=Colors.GREEN,
                )
        else:
            command_list: List[Union[Command, Group]] = list(
                filter(
                    lambda c: (type(c) != ContextMenu),
                    interaction.client.tree.get_commands(),
                )
            )
            command_list.sort(key=lambda c: c.name)
            embed = Embed(
                title="Help",
                description="\n".join(
                    f":small_blue_diamond: {cmd.name} - {cmd.description}"
                    for cmd in command_list
                ),
                color=Colors.GREEN,
            )

        await interaction.response.send_message(embed=embed, ephemeral=True)

    @app_commands.command(name="about", description="Get information about the bot")
    async def about(self, interaction: Interaction) -> None:
        """
        Print information about the bot.
        :param interaction: The interaction payload
        :return: None
        """
        embed = Embed(title=interaction.client.application.name, color=Colors.GREEN)
        embed.add_field(name="Author", value="<@206815202375761920>", inline=True)
        embed.add_field(
            name="Version",
            value=Config.VERSION,
            inline=True,
        )
        embed.add_field(name="GitHub", value=Config.REPOSITORY, inline=False)
        embed.add_field(
            name="Bug Reports / Feature Requests",
            value=f"Please open an issue on [GitHub]({Config.REPOSITORY})",
            inline=False,
        )
        embed.set_thumbnail(url=f"{interaction.client.user.avatar.url}")
        await interaction.response.send_message(embed=embed, ephemeral=True)

    @app_commands.command(name="server", description="Get information about the server")
    @app_commands.guild_only()
    async def server(self, interaction: Interaction) -> None:
        """
        Print information about the server.
        :param interaction: The interaction payload
        :return: None
        """
        if interaction.guild is None:
            logger.error('Guild only command "/server" was called.')
            return

        guild = interaction.guild
        embed = Embed(title=guild.name, description="Server Info", color=Colors.GREEN)
        if guild.icon is not None:
            embed.set_thumbnail(url=guild.icon.url)

        created = guild.created_at
        embed.add_field(
            name="Creation Date",
            value=f"{created.day}.{created.month}.{created.year}",
            inline=True,
        )
        members: List[Member] = [
            member for member in guild.members if member.bot is False
        ]
        members_online = len(
            [member for member in members if member.status != Status.offline]
        )
        embed.add_field(
            name=f"{len(members)} Members",
            value=f"{members_online} online",
            inline=True,
        )

        if guild.owner is not None:
            embed.add_field(name="Owner", value=guild.owner.mention, inline=True)
        # assume a moderator can at least mute members
        embed.add_field(
            name="Moderators",
            value="\n".join(
                ":small_blue_diamond: " + member.mention
                for member in members
                if member.guild_permissions.mute_members
                and not member.guild_permissions.administrator  # don't include administrators
            ),
        )
        embed.add_field(
            name="Administrator",
            value="\n".join(
                ":small_blue_diamond: " + member.mention
                for member in members
                if member.guild_permissions.administrator
            ),
        )

        await interaction.response.send_message(embed=embed, ephemeral=True)
