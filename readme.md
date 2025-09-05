# ATSPI MCP

This is a simple example of an MCP server for retrieving the state of the accessibility tree using ATSPI (Assistive Technology Service Provider Interface) on Linux.

## Testing with a client

You can use any MCP client of your choice, such [claude desktop](https://github.com/aaddrick/claude-desktop-debian) which provides a free MCP client to test this server. If you use claude desktop, you can run the `./setupClaude.sh` script to set up the server automatically.

Note, apparmor can block atspi communication so you should launch your server in a terminal outside of an electron app like VSCode that has an apparmor sandbox. [Read more here](https://github.com/odilia-app/atspi/issues/257)