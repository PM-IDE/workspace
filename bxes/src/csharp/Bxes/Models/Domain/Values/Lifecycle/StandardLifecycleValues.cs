namespace Bxes.Models.Domain.Values.Lifecycle;

public enum StandardLifecycleValues : byte
{
  Unspecified = 0,
  Assign = 1,
  AteAbort = 2,
  Autoskip = 3,
  Complete = 4,
  ManualSkip = 5,
  PiAbort = 6,
  ReAssign = 7,
  Resume = 8,
  Schedule = 9,
  Start = 10,
  Suspend = 11,
  Unknown = 12,
  Withdraw = 13
}

public static class StandardLifecycleValuesUtil
{
  public static StandardLifecycleValues? TryParse(string value) => value switch
  {
    "unspecified" => StandardLifecycleValues.Unspecified,
    "assign" => StandardLifecycleValues.Assign,
    "ate_abort" => StandardLifecycleValues.AteAbort,
    "autoskip" => StandardLifecycleValues.Autoskip,
    "complete" => StandardLifecycleValues.Complete,
    "manualskip" => StandardLifecycleValues.ManualSkip,
    "pi_abort" => StandardLifecycleValues.PiAbort,
    "reassign" => StandardLifecycleValues.ReAssign,
    "resume" => StandardLifecycleValues.Resume,
    "schedule" => StandardLifecycleValues.Schedule,
    "start" => StandardLifecycleValues.Start,
    "suspend" => StandardLifecycleValues.Suspend,
    "unknown" => StandardLifecycleValues.Unknown,
    "withdraw" => StandardLifecycleValues.Withdraw,
    _ => null
  };
}