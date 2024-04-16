using System.Runtime.InteropServices;

Main();

void Main()
{
    try
    {
        throw new COMException("COM error simulation", unchecked((int)0x80131500)); // COMException
    }
    catch (Exception ex)
    {
    }
    finally
    {
    }
}