#include "utils.h"
#include <string>

void writeFunctionEvent(const FunctionEvent& event, std::ofstream& fout) {
    const char startOrEnd = event.EventKind == FunctionEventKind::Started ? 1 : 0;
    fout.write(&startOrEnd, sizeof(char));
    fout.write((char*) &event.Timestamp, sizeof(long long));
    fout.write((char*) &event.Id, sizeof(long long));
}

std::string createBinStackSavePath(const std::string& directory, const ThreadID& threadId) {
    return directory + "binstack_" + std::to_string(threadId) + ".bin";
}
