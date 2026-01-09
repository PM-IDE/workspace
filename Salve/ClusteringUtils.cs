using System.Numerics;

namespace Salve;

internal static class ClusteringUtils
{
  public static int CalculateEditDistance<T>(ReadOnlySpan<T> first, ReadOnlySpan<T> second) where T : IEqualityOperators<T, T, bool>
  {
    if (first.Length == 0) return second.Length;
    if (second.Length == 0) return first.Length;

    var current = 1;
    var previous = 0;

    var r = new int[2, second.Length + 1];
    for (var i = 0; i <= second.Length; i++)
    {
      r[previous, i] = i;
    }

    for (var i = 0; i < first.Length; i++)
    {
      r[current, 0] = i + 1;
      for (var j = 1; j <= second.Length; j++)
      {
        var cost = (second[j - 1] == first[i]) ? 0 : 1;
        r[current, j] = Min(r[previous, j] + 1, r[current, j - 1] + 1, r[previous, j - 1] + cost);
      }

      previous = (previous + 1) % 2;
      current = (current + 1) % 2;
    }

    return r[previous, second.Length];
  }

  private static int Min(int e1, int e2, int e3) => Math.Min(Math.Min(e1, e2), e3);

  public record struct LcsInfo<T>(T[] Lcs, List<int> FirstIndices, List<int> SecondIndices);

  public static LcsInfo<T> FindLcs<T>(ReadOnlySpan<T> first, ReadOnlySpan<T> second)
    where T : IEqualityOperators<T, T, bool>
  {
    var n = first.Length;
    var m = second.Length;
    var dp = new int[n + 1, m + 1];

    for (var i = 1; i <= n; i++)
    {
      for (var j = 1; j <= m; j++)
      {
        if (first[i - 1] == second[j - 1])
        {
          dp[i, j] = dp[i - 1, j - 1] + 1;
        }
        else
        {
          dp[i, j] = Math.Max(dp[i - 1, j], dp[i, j - 1]);
        }
      }
    }

    return RestoreLcs(first, second, dp, n, m);
  }

  private static LcsInfo<T> RestoreLcs<T>(ReadOnlySpan<T> first, ReadOnlySpan<T> second, int[,] dp, int n, int m)
    where T : IEqualityOperators<T, T, bool>
  {
    int i = n, j = m;
    List<T> lcs = [];
    List<int> firstIndices = [];
    List<int> secondIndices = [];

    while (i > 0 && j > 0)
    {
      if (first[i - 1] == second[j - 1])
      {
        firstIndices.Add(i - 1);
        secondIndices.Add(j - 1);

        lcs.Add(first[i - 1]);

        i--;
        j--;
      }
      else if (dp[i - 1, j] > dp[i, j - 1])
      {
        i--;
      }
      else
      {
        j--;
      }
    }

    firstIndices.Reverse();
    secondIndices.Reverse();
    lcs.Reverse();

    return new LcsInfo<T>(lcs.ToArray(), firstIndices, secondIndices);
  }
}