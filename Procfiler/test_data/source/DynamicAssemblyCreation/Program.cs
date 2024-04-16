//https://learn.microsoft.com/ru-ru/dotnet/api/system.reflection.emit.assemblybuilder?view=net-7.0

using System.Reflection;
using System.Reflection.Emit;

namespace DynamicAssemblyCreation;

internal class Program
{
  public static void Main(string[] args)
  {
    var name = new AssemblyName("DynamicAssemblyExample");
    var assembly = AssemblyBuilder.DefineDynamicAssembly(name, AssemblyBuilderAccess.Run);

    var module = assembly.DefineDynamicModule(name.Name + "_MODULE");
    var type = module.DefineType("MyDynamicType", TypeAttributes.Public);
    var numberField = type.DefineField("m_number", typeof(int), FieldAttributes.Private);

    var ctor = type.DefineConstructor(MethodAttributes.Public, CallingConventions.Standard, new[] { typeof(int) });

    var ctorIl = ctor.GetILGenerator();


    ctorIl.Emit(OpCodes.Ldarg_0);
    ctorIl.Emit(OpCodes.Call, typeof(object).GetConstructor(Type.EmptyTypes));
    ctorIl.Emit(OpCodes.Ldarg_0);
    ctorIl.Emit(OpCodes.Ldarg_1);
    ctorIl.Emit(OpCodes.Stfld, numberField);
    ctorIl.Emit(OpCodes.Ret);

    var defaultConstructor = type.DefineConstructor(MethodAttributes.Public, CallingConventions.Standard, Type.EmptyTypes);

    var defaultIlConstructor = defaultConstructor.GetILGenerator();

    defaultIlConstructor.Emit(OpCodes.Ldarg_0);
    defaultIlConstructor.Emit(OpCodes.Ldc_I4_S, 42);
    defaultIlConstructor.Emit(OpCodes.Call, ctor);
    defaultIlConstructor.Emit(OpCodes.Ret);

    var numberProperty = type.DefineProperty("Number", PropertyAttributes.HasDefault, typeof(int), null);

    const MethodAttributes GetSetAttr = MethodAttributes.Public |
                                        MethodAttributes.SpecialName | 
                                        MethodAttributes.HideBySig;


    var mbNumberGetAccessor = type.DefineMethod("get_Number", GetSetAttr, typeof(int), Type.EmptyTypes);

    var numberGetIl = mbNumberGetAccessor.GetILGenerator();

    numberGetIl.Emit(OpCodes.Ldarg_0);
    numberGetIl.Emit(OpCodes.Ldfld, numberField);
    numberGetIl.Emit(OpCodes.Ret);

    var mbNumberSetAccessor = type.DefineMethod("set_Number", GetSetAttr, null, new[] { typeof(int) });

    var numberSetIL = mbNumberSetAccessor.GetILGenerator();
    numberSetIL.Emit(OpCodes.Ldarg_0);
    numberSetIL.Emit(OpCodes.Ldarg_1);
    numberSetIL.Emit(OpCodes.Stfld, numberField);
    numberSetIL.Emit(OpCodes.Ret);

    numberProperty.SetGetMethod(mbNumberGetAccessor);
    numberProperty.SetSetMethod(mbNumberSetAccessor);

    var method = type.DefineMethod("MyMethod", MethodAttributes.Public, typeof(int), new[] { typeof(int) });

    var methodIl = method.GetILGenerator();
    methodIl.Emit(OpCodes.Ldarg_0);
    methodIl.Emit(OpCodes.Ldfld, numberField);
    methodIl.Emit(OpCodes.Ldarg_1);
    methodIl.Emit(OpCodes.Mul);
    methodIl.Emit(OpCodes.Ret);

    var createdType = type.CreateType();
    var mi = createdType.GetMethod("MyMethod");
    var pi = createdType.GetProperty("Number");

    var o1 = Activator.CreateInstance(createdType);
    Console.WriteLine("o1.Number: {0}", pi.GetValue(o1, null));

    pi.SetValue(o1, 127, null);
    Console.WriteLine("o1.Number: {0}", pi.GetValue(o1, null));

    object[] arguments = { 22 };
    Console.WriteLine("o1.MyMethod(22): {0}", mi.Invoke(o1, arguments));

    var o2 = Activator.CreateInstance(createdType, 5280);
    Console.WriteLine("o2.Number: {0}", pi.GetValue(o2, null));
  }
}