using Procfiler.Core.EventRecord;

namespace ProcfilerTests.Tests.EventMetadataTests;

[TestFixture]
public class EventsMetadataTest
{
  [Test]
  public void SimpleTest()
  {
    var metadata = new EventMetadata();

    var (key, value) = ("Key", "Value");
    metadata.Add(key, value);
    Assert.That(metadata, Has.Count.EqualTo(1));
    Assert.That(metadata.ContainsKey(key), Is.True);
    Assert.That(metadata.Contains(new KeyValuePair<string, string>(key, value)), Is.True);
    Assert.That(metadata[key], Is.EqualTo(value));
    Assert.That(metadata.IsReadOnly, Is.False);

    const string NotExistingKey = nameof(NotExistingKey);
    Assert.That(metadata.ContainsKey(NotExistingKey), Is.False);
    Assert.That(metadata.Contains(new KeyValuePair<string, string>(NotExistingKey, "asdasd")), Is.False);
    Assert.That(metadata.Remove(NotExistingKey), Is.False);
    Assert.That(metadata.Count, Is.EqualTo(1));

    Assert.That(metadata.Remove(key), Is.True);
    Assert.That(metadata.ContainsKey(key), Is.False);
    Assert.That(metadata.Contains(new KeyValuePair<string, string>(key, value)), Is.False);
    Assert.That(metadata, Has.Count.Zero);
    Assert.Throws<KeyNotFoundException>(() => { _ = metadata[key]; });
  }
}