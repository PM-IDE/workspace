using System.Globalization;
using System.Xml;
using Bxes.Models;
using Bxes.Models.Domain;
using Bxes.Models.Domain.Values;
using Bxes.Models.Domain.Values.Lifecycle;
using Bxes.Reader;
using Bxes.Writer;

namespace Bxes.Xes.BxesToXes;

internal readonly struct StartEndElementCookie : IDisposable
{
  public static StartEndElementCookie CreateStartEndElement(
    XmlWriter xmlWriter, string? prefix, string tagName, string? @namespace)
  {
    xmlWriter.WriteStartElement(prefix, tagName, @namespace);
    return new StartEndElementCookie(xmlWriter);
  }


  private readonly XmlWriter myXmlWriter;


  private StartEndElementCookie(XmlWriter xmlWriter)
  {
    myXmlWriter = xmlWriter;
  }


  public void Dispose() => myXmlWriter.WriteEndElement();
}

public class BxesToXesConverter : IBetweenFormatsConverter
{
  public void Convert(string filePath, string outputPath)
  {
    var log = new SingleFileBxesReader().Read(filePath);

    using var fs = File.OpenWrite(outputPath);
    using var writer = XmlWriter.Create(fs, new XmlWriterSettings
    {
      Indent = true
    });

    using var _ = StartEndElementCookie.CreateStartEndElement(writer, null, XesConstants.LogTagName, null);
    WriteMetadata(writer, log.Metadata);
    foreach (var traceVariant in log.Traces)
    {
      WriteTrace(traceVariant, writer);
    }
  }

  private void WriteMetadata(XmlWriter writer, IEventLogMetadata metadata)
  {
    foreach (var property in metadata.Properties)
    {
      WriteKeyValuePair(writer, property);
    }

    foreach (var extension in metadata.Extensions)
    {
      WriteExtensionTag(writer, extension.Name.Value, extension.Prefix.Value, extension.Uri.Value);
    }

    foreach (var global in metadata.Globals)
    {
      WriteGlobal(writer, global);
    }

    foreach (var classifier in metadata.Classifiers)
    {
      WriteClassifier(writer, classifier);
    }
  }

  private static void WriteClassifier(XmlWriter writer, BxesClassifier classifier)
  {
    using var _ = StartEndElementCookie.CreateStartEndElement(writer, null, XesConstants.ClassifierTagName, null);
    WriteAttribute(writer, XesConstants.ClassifierNameAttribute, classifier.Name.Value);

    var keys = string.Join(' ', classifier.Keys.Select(key => key.Value));
    WriteAttribute(writer, XesConstants.ClassifierKeysAttribute, keys);
  }

  private static void WriteGlobal(XmlWriter writer, BxesGlobal global)
  {
    using var _ = StartEndElementCookie.CreateStartEndElement(writer, null, XesConstants.GlobalTagName, null);
    WriteAttribute(writer, XesConstants.GlobalScopeAttribute, global.Kind.ToString().ToLower());

    foreach (var attribute in global.Globals)
    {
      WriteKeyValuePair(writer, attribute);
    }
  }

  private void WriteTrace(ITraceVariant variant, XmlWriter writer)
  {
    writer.WriteStartElement(null, XesConstants.TraceTagName, null);
    foreach (var attribute in variant.Metadata)
    {
      WriteKeyValuePair(writer, attribute);
    }

    foreach (var @event in variant.Events)
    {
      WriteEventNode(writer, @event);
    }

    writer.WriteEndElement();
  }

  private static void WriteEventNode(XmlWriter writer, IEvent currentEvent)
  {
    using var _ = StartEndElementCookie.CreateStartEndElement(writer, null, XesConstants.EventTagName, null);

    WriteDateTag(writer, currentEvent.Timestamp);
    WriteValueTag(writer, XesConstants.StringTagName, XesConstants.ConceptName, currentEvent.Name);

    foreach (var attribute in currentEvent.Attributes)
    {
      WriteKeyValuePair(writer, attribute);
    }
  }

  private static void WriteKeyValuePair(XmlWriter writer, AttributeKeyValue attribute)
  {
    var keyValue = attribute.Key.Value;
    switch (attribute.Value)
    {
      case BxesInt32Value:
      case BxesInt64Value:
      case BxesUint32Value:
      case BxesUint64Value:
        WriteValueTag(writer, XesConstants.IntTagName, keyValue, attribute.Value.ToString()!);
        break;
      case BxesFloat32Value:
      case BxesFloat64Value:
        WriteValueTag(writer, XesConstants.FloatTagName, keyValue, attribute.Value.ToString()!);
        break;
      case BxesStringValue stringValue:
        WriteValueTag(writer, XesConstants.StringTagName, keyValue, stringValue.Value);
        break;
      case BxesBoolValue boolValue:
        WriteValueTag(writer, XesConstants.BoolTagName, keyValue, boolValue.Value ? "true" : "false");
        break;
      case BxesTimeStampValue timeStampValue:
        WriteValueTag(writer, XesConstants.DateTagName, keyValue, timeStampValue.Timestamp.ToString("O"));
        break;
      case BxesArtifactModelsListValue artifactItem:
        WriteArtifact(writer, artifactItem);
        break;
      case BxesDriversListValue driversListValue:
        WriteDrivers(writer, driversListValue);
        break;
      case BxesGuidValue guidValue:
        WriteValueTag(writer, XesConstants.StringTagName, keyValue, guidValue.Value.ToString());
        break;
      case BxesSoftwareEventTypeValue softwareEvent:
        WriteValueTag(writer, XesConstants.StringTagName, keyValue, softwareEvent.ToStringValue());
        break;
      case IEventLifecycle lifecycle:
        WriteValueTag(writer, XesConstants.StringTagName, keyValue, lifecycle.ToStringValue());
        break;
      default:
        throw new ArgumentOutOfRangeException();
    }
  }

  private static void WriteDrivers(XmlWriter writer, BxesDriversListValue drivers)
  {
    using var _ = StartEndElementCookie.CreateStartEndElement(writer, null, XesConstants.ListTagName, null);
    WriteAttribute(writer, XesConstants.KeyAttributeName, XesConstants.CostDrivers);

    foreach (var driver in drivers.Value)
    {
      using (StartEndElementCookie.CreateStartEndElement(writer, null, XesConstants.CostDriver, null))
      {
        WriteValueTag(writer, XesConstants.StringTagName, XesConstants.CostDriver, driver.Name);
        WriteValueTag(writer, XesConstants.StringTagName, XesConstants.ArtifactItemTransition, driver.Type);
        WriteValueTag(writer, XesConstants.FloatTagName, XesConstants.CostAmount, driver.Amount.ToString(CultureInfo.InvariantCulture));
      }
    }
  }

  private static void WriteArtifact(XmlWriter writer, BxesArtifactModelsListValue artifact)
  {
    using var _ = StartEndElementCookie.CreateStartEndElement(writer, null, XesConstants.ListTagName, null);
    WriteAttribute(writer, XesConstants.KeyAttributeName, XesConstants.ArtifactMoves);

    using var __ = StartEndElementCookie.CreateStartEndElement(writer, null, XesConstants.ValuesTagName, null);
    foreach (var item in artifact.Value)
    {
      using (StartEndElementCookie.CreateStartEndElement(writer, null, XesConstants.StringTagName, null))
      {
        WriteAttribute(writer, XesConstants.ArtifactItemModel, item.Model);

        WriteValueTag(writer, XesConstants.StringTagName, XesConstants.ArtifactItemInstance, item.Instance);
        WriteValueTag(writer, XesConstants.StringTagName, XesConstants.ArtifactItemTransition, item.Transition);
      }
    }
  }

  private static void WriteExtensionTag(XmlWriter writer, string name, string prefix, string uri)
  {
    using var _ = StartEndElementCookie.CreateStartEndElement(writer, null, XesConstants.ExtensionTagName, null);
    writer.WriteAttributeString(null, XesConstants.ExtensionNameAttribute, null, name);
    writer.WriteAttributeString(null, XesConstants.ExtensionPrefixAttribute, null, prefix);
    writer.WriteAttributeString(null, XesConstants.ExtensionUriAttribute, null, uri);
  }

  private static void WriteValueTag(XmlWriter writer, string valueTagName, string key, string value)
  {
    using var _ = StartEndElementCookie.CreateStartEndElement(writer, null, valueTagName, null);
    WriteKeyAttribute(writer, key);
    WriteAttribute(writer, XesConstants.ValueAttributeName, value);
  }

  private static void WriteDateTag(XmlWriter writer, long stamp)
  {
    using var _ = StartEndElementCookie.CreateStartEndElement(writer, null, XesConstants.DateTagName, null);
    WriteKeyAttribute(writer, XesConstants.TimeTimestamp);

    var dateString = DateTimeOffset.UnixEpoch.AddTicks(stamp / 100).ToString("O");
    WriteAttribute(writer, XesConstants.ValueAttributeName, dateString);
  }

  private static void WriteAttribute(XmlWriter writer, string name, string value)
  {
    writer.WriteAttributeString(null, name, null, value);
  }

  private static void WriteKeyAttribute(XmlWriter writer, string value)
  {
    writer.WriteAttributeString(null, XesConstants.KeyAttributeName, null, value);
  }
}