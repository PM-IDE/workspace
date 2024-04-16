using System.Net;
using System.Net.Sockets;

CancellationTokenSource cancellationTokenSource = new();
var token = cancellationTokenSource.Token;
var serverTask = Task.Factory.StartNew(() =>
{
  var ipAddress = IPAddress.Parse("127.0.0.1");
  var server = new TcpListener(ipAddress, 5501);
  server.Start();

  while (!token.IsCancellationRequested)
  {
    Console.WriteLine("The server is waiting on {0}:5501...", ipAddress);

    var attachedClient = server.AcceptTcpClient();

    var clientIn = new StreamReader(attachedClient.GetStream());
    while (!token.IsCancellationRequested && clientIn.ReadLine() is { } msg)
    {
      Console.WriteLine("The server received: {0}", msg);
    }
  }
  
  server.Stop();
});

using (TcpClient client = new())
{
  var result = client.BeginConnect("localhost", 5501, null, null);
  result.AsyncWaitHandle.WaitOne(500);

  if (client.Connected)
  {
    Console.WriteLine("The client is connected to the server...");
    var clientOut = new StreamWriter(client.GetStream());

    clientOut.AutoFlush = true;

    for (var i = 0; i < 10; i++)
    {
      clientOut.WriteLine("Hello");
    }
  }

  await Task.Delay(1000);
  cancellationTokenSource.Cancel();
  client.Close();
}

await Task.WhenAny(serverTask, Task.Delay(1000));