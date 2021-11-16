This example is based on the SignalR walkthrough at https://docs.microsoft.com/en-us/aspnet/core/tutorials/signalr

To run the server and C# client:
1. Load the BlazorWebAssemblySignalRApp.sln
2. Rebuild All
3. Right-click the BlazorWebAssemblySignalRApp.Server project then choose Debug->Start Without Debugging
4. Right-click the SignalRApp.Console.Client project then choose Debug->Start Without Debugging
5. In the browser that was shown in step 3, observe the 'ConsoleApp' message. Enter a User and Message and press Send. Observe the message in the SignalRApp.Console.Client app.

Build the Rust console client app in the RustSignalRClient folder. Note that this requires OpenSSL to be configured. The readme.txt in that folder has some tips for configuring OpenSSL on Windows if you are building on Windows.

Once built, run the executable.

Code Notes:
For the C# SignalRApp.Console.Client, the SignalR code is located in the SignalRConnection.cs file.


Notes for Visual Studio 2022:
The docker-compose project doesn't appear to be supported in Visual Studio 2022. You can ignore this and just run the server directly.


