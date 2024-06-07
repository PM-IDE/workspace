using Procfiler.Utils;

namespace Procfiler.Core.Documentation.Markdown;

public class MdTable(int columnsCount) : IMdDocumentPart, IEnumerable
{
  private const char CellsDelimiter = '|';
  private const char Space = ' ';
  private const char SeparatorPath = '-';


  private readonly List<MdTableCell[]> myRows = [];


  public MdTableCell[]? Header { get; set; }
  public int ColumnsCount { get; } = columnsCount;


  public void Add(IEnumerable<string[]> rows)
  {
    foreach (var row in rows)
    {
      Add(row);
    }
  }

  public void Add(string[] row) => Add(row.Select(rawCell => new MdTableCell(rawCell)).ToArray());

  public void Add(MdTableCell[] row)
  {
    if (row.Length != ColumnsCount)
    {
      throw new IncorrectRowsCountException(row.Length, ColumnsCount);
    }

    myRows.Add(row);
  }

  public StringBuilder Serialize(StringBuilder sb)
  {
    var rowLengths = CalculateColumnsWidths();

    if (Header is { } header)
    {
      AddRowToStringBuilder(header, rowLengths, sb);
    }
    else
    {
      var emptyHeader = Enumerable.Range(0, ColumnsCount).Select(_ => new MdTableCell(string.Empty)).ToArray();
      Debug.Assert(emptyHeader.Length == ColumnsCount);
      AddRowToStringBuilder(emptyHeader, rowLengths, sb);
    }

    AddSeparatorBetweenHeaderAndTableContents(rowLengths, sb);
    foreach (var row in myRows)
    {
      AddRowToStringBuilder(row, rowLengths, sb);
    }

    return sb;
  }

  public IEnumerator GetEnumerator() => myRows.GetEnumerator();

  private int[] CalculateColumnsWidths()
  {
    var columnsLengths = new int[ColumnsCount];

    foreach (var cells in myRows)
    {
      for (var i = 0; i < cells.Length; i++)
      {
        columnsLengths[i] = Math.Max(columnsLengths[i], cells[i].ContentLength);
      }
    }

    if (Header is { } header)
    {
      for (var i = 0; i < header.Length; i++)
      {
        columnsLengths[i] = Math.Max(columnsLengths[i], header[i].ContentLength);
      }
    }

    return columnsLengths;
  }

  private static void AddRowToStringBuilder(MdTableCell[] row, int[] rowsLengths, StringBuilder sb)
  {
    sb.Append(CellsDelimiter);

    foreach (var (cell, length) in row.Zip(rowsLengths))
    {
      var spacesToAdd = length - cell.ContentLength;
      cell.Serialize(sb).Append(Space, spacesToAdd).Append(CellsDelimiter);
    }

    sb.AppendNewLine();
  }

  private static void AddSeparatorBetweenHeaderAndTableContents(int[] rowsLengths, StringBuilder sb)
  {
    sb.Append(CellsDelimiter);

    foreach (var length in rowsLengths)
    {
      sb.Append(SeparatorPath, length).Append(CellsDelimiter);
    }

    sb.AppendNewLine();
  }
}