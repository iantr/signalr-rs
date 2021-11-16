using System;
using System.Collections.Generic;
using System.Linq;
using System.Net;
using System.Text;
using System.Threading.Tasks;
using Microsoft.AspNetCore.SignalR.Client;

namespace SignalRApp.ConsoleApp.Client
{
    public class SignalRConnection
    {
        public async void Start()
        {
            var url = "https://localhost:5001/chathub"; 

            var hubConnection = new HubConnectionBuilder()
                .WithUrl(url)
                .WithAutomaticReconnect()
                .Build();

            // Avoid certificate errors for now
            ServicePointManager.ServerCertificateValidationCallback += (sender, cert, chain, sslPolicyErrors) => true;

            // receive a message from the hub
            hubConnection.On<string, string>("ReceiveMessage", (user, message) => OnReceiveMessage(user, message));

            var task = hubConnection.StartAsync();

            task.Wait();

            // send a message to the hub
            await hubConnection.InvokeAsync("SendMessage", "ConsoleApp", "Message from the console app");
        }

        private void OnReceiveMessage(string user, string message)
        {
            Console.WriteLine($"{user}: {message}");
        }
    }
}
