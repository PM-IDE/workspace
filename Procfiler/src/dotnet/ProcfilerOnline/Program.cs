using Core.Container;
using Microsoft.Extensions.Logging;
using ProcfilerOnline.Core.Container;

ProgramEntryPoint.SetupContainerAndRun("procfiler-online", args, ConfigurationUtil.AddConfiguration, LogLevel.Debug);