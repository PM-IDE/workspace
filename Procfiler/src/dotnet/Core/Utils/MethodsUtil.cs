namespace Core.Utils;

public static class MethodsUtil
{
  public static string ConcatenateMethodDetails(string methodName, string methodNamespace, string signature) =>
    string.Intern(methodNamespace +
                  (methodNamespace.EndsWith('.') ? "" : ".") +
                  methodName +
                  $"[{signature.Replace(' ', '.')}]");
}