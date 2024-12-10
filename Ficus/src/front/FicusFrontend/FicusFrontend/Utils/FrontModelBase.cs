namespace FicusFrontend.Utils;

public abstract class FrontModelBase
{
  public IUserDateHolder UserData { get; } = new UserDateHolderBase();
}
