Main();

void Main()
{
    try
    {
        List<int> list = new List<int> {1, 2, 3};
        int element = list[3]; // ArgumentOutOfRangeException
    }
    catch (Exception ex)
    {
    }
    finally
    {
    }
}