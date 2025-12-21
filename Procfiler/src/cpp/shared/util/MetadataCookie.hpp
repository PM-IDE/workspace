//
// Created by HYPERPC on 9/13/2025.
//

#ifndef PROCFILER_METADATACOOKIE_H
#define PROCFILER_METADATACOOKIE_H
#include <cor.h>


class MetadataCookie {
    IMetaDataImport2* myMetadata;

public:
    explicit MetadataCookie(IMetaDataImport2* metadata) {
        myMetadata = metadata;
    }

    ~MetadataCookie() {
        myMetadata->Release();
    }
};


#endif //PROCFILER_METADATACOOKIE_H
