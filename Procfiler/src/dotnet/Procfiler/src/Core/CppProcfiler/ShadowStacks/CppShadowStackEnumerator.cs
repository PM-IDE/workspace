namespace Procfiler.Core.CppProcfiler.ShadowStacks;

internal class CppShadowStackEnumerator(BinaryReader reader, long framesCount) : IEnumerator<FrameInfo>
{
  private const int FramesToRead = 5000;

  private readonly byte[] myBuffer = new byte[FramesToRead * CppShadowStackHelpers.OneFrameLength];

  private int myNextIndexInBuffer = -1;
  private int myFramesCountInBuffer;
  private int myReadFramesForCurrentBuffer;
  private long myTotalReadFrames;

  
  object IEnumerator.Current => Current;
  public FrameInfo Current { get; } = new();

  
  public bool MoveNext()
  {
    if (myNextIndexInBuffer < 0 || myNextIndexInBuffer >= myBuffer.Length)
    {
      ReadFrames();
      myReadFramesForCurrentBuffer = 0;
      myNextIndexInBuffer = 0;
    }
    
    if (myReadFramesForCurrentBuffer >= myFramesCountInBuffer || myTotalReadFrames >= framesCount)
    {
      return false;
    }

    Current.IsStart = myBuffer[myNextIndexInBuffer++] == 1;
    
    Current.TimeStamp = ToLong(myNextIndexInBuffer);
    myNextIndexInBuffer += sizeof(long);

    Current.FunctionId = ToLong(myNextIndexInBuffer);
    myNextIndexInBuffer += sizeof(long);

    myReadFramesForCurrentBuffer++;
    myTotalReadFrames++;
    
    return true;
  }

  private void ReadFrames()
  {
    var readBytes = reader.Read(myBuffer, 0, myBuffer.Length);
    myFramesCountInBuffer = readBytes / (int)CppShadowStackHelpers.OneFrameLength;
  }

  private long ToLong(int startIndex)
  {
    var span = myBuffer.AsSpan()[startIndex..(startIndex + 8)];
    return BitConverter.ToInt64(span);
  }

  public void Reset()
  {
  }

  public void Dispose()
  {
    reader.Dispose();
  }
}