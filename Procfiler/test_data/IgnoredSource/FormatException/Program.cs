Main();

void Main()
{
    try
    {
        string text = "not a number";
        int number = int.Parse(text); // FormatException
    }
    catch (Exception ex)
    {
    }
    finally
    {
    }
}