﻿using Bxes.Models.Domain;
using Bxes.Models.System;

namespace Core.Bxes;

public static class BxesUtil
{
  public static SystemMetadata CreateSystemMetadata()
  {
    var metadata = new SystemMetadata();
    metadata.ValueAttributeDescriptors.Add(new ValueAttributeDescriptor(TypeIds.I64, "QpcStamp"));

    return metadata;
  }
}