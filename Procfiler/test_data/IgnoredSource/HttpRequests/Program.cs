// See https://aka.ms/new-console-template for more information
Console.WriteLine("Hello, World!");

using var client = new HttpClient();
var response = client.GetAsync("https://google.com").GetAwaiter().GetResult();
Console.WriteLine(response.Headers.Count());