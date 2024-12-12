namespace FicusFrontend.Utils;

public abstract class FrontModelBase
{
  public IUserDataHolder UserData { get; } = new UserDataHolder();
}
