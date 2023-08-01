from discord import app_commands, Interaction, Embed
from discord.app_commands import ContextMenu, Command, Group
from discord.ext.commands import Cog, Bot
from typing import Optional, List, Union

from kronos.utils.colors import Colors
from kronos.utils.config import Config


async def setup(bot: Bot):
    bot.remove_command("help")
    await bot.add_cog(HelpCog(bot))


class HelpCog(Cog):
    @app_commands.command(name="help", description="Displays this help message")
    async def _help(self, interaction: Interaction, command: Optional[str]):
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
            command_list: List[Command, Group] = list(
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
    async def about(self, interaction: Interaction):
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
