using System.Threading.Tasks;
using Microsoft.AspNetCore.SignalR;
using Microsoft.Extensions.Logging;

namespace BlazorWebAssemblySignalRApp.Server.Hubs
{
    public class ChatHub : Hub
    {
        private readonly ILogger<ChatHub> _logger;

        public ChatHub(ILogger<ChatHub> logger)
        {
            _logger = logger;
        }

        public async Task SendMessage(string user, string message)
        {
            _logger.Log(LogLevel.Information, "Message: from {0}: {1}", user, message);

            await Clients.All.SendAsync("ReceiveMessage", user, message);
        }
    }
}