using Bxes.Models;
using Bxes.Models.System;
using Bxes.Utils;

namespace Bxes.Writer.Stream;

public class SingleFileBxesStreamWriterImpl<TEvent> : 
  IBxesStreamWriter, IXesToBxesStatisticsCollector where TEvent : IEvent
{
  private readonly MultipleFilesBxesStreamWriterImpl<TEvent> myMultipleWriter;
  private readonly string mySavePath;
  private readonly uint myBxesVersion;
  private readonly TempFolderContainer myFolderContainer;


  public SingleFileBxesStreamWriterImpl(string savePath, uint bxesVersion) 
    : this(savePath, bxesVersion, SystemMetadata.Default)
  {
  }

  public SingleFileBxesStreamWriterImpl(string savePath, uint bxesVersion, ISystemMetadata metadata)
  {
    mySavePath = savePath;
    myBxesVersion = bxesVersion;
    myFolderContainer = new TempFolderContainer();
    myMultipleWriter = new MultipleFilesBxesStreamWriterImpl<TEvent>(myFolderContainer.Path, bxesVersion, metadata);
  }


  public void HandleEvent(BxesStreamEvent @event) => myMultipleWriter.HandleEvent(@event);

  public void Dispose()
  {
    myMultipleWriter.Dispose();

    MergeFilesIntoOne();

    myFolderContainer.Dispose();
  }

  private void MergeFilesIntoOne()
  {
    PathUtil.EnsureDeleted(mySavePath);

    using var tempFileCookie = new TempFilePathContainer();

    using (var writer = new BinaryWriter(File.OpenWrite(tempFileCookie.Path)))
    {
      writer.Write(myBxesVersion);

      BinaryReader OpenRead(string fileName) => new(File.OpenRead(Path.Join(myFolderContainer.Path, fileName)));

      SkipVersionAndCopyContents(OpenRead(BxesConstants.SystemMetadataFileName), writer);
      SkipVersionAndCopyContents(OpenRead(BxesConstants.ValuesFileName), writer);
      SkipVersionAndCopyContents(OpenRead(BxesConstants.KVPairsFileName), writer);
      SkipVersionAndCopyContents(OpenRead(BxesConstants.MetadataFileName), writer);
      SkipVersionAndCopyContents(OpenRead(BxesConstants.TracesFileName), writer);
    }

    BxesWriteUtils.CreateZipArchive(new[] { tempFileCookie.Path }, mySavePath);
  }

  private static void SkipVersionAndCopyContents(BinaryReader reader, BinaryWriter writer)
  {
    try
    {
      const int VersionSize = sizeof(int);
      reader.BaseStream.Seek(VersionSize, SeekOrigin.Begin);

      WriteFromReaderToWriter(reader, writer);
    }
    finally
    {
      reader.Dispose();
    }
  }

  private static void WriteFromReaderToWriter(BinaryReader reader, BinaryWriter writer)
  {
    var buffer = new byte[1024];

    while (true)
    {
      var readCount = reader.Read(buffer);
      if (readCount == 0)
      {
        break;
      }

      writer.Write(buffer, 0, readCount);
    }
  }

  public XesToBxesConversionStatistics ObtainStatistics() => myMultipleWriter.ObtainStatistics();
}