using System;
using System.Collections.Generic;
using System.Text;
using Microsoft.AspNetCore.SignalR.Client;

namespace SignalRApp.ConsoleApp.Client
{
    class Program
    {
        static void Main(string[] args)
        {
            Console.WriteLine("SignalR Console Client");

            var signalRConnection = new SignalRConnection();
            signalRConnection.Start();

            Console.Read();
        }
    }
}
