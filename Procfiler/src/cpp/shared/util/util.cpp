#include <cstdlib>
#include "util.h"


std::string GetEnvironmentValue(const std::string& name) {
    const auto cstr = std::getenv(name.c_str());
    if (cstr == nullptr) {
        return "";
    }

    return std::string(cstr);
}

wstring ToString(const std::vector<WCHAR>& data, size_t length) {
    if (data.empty() || length == 0) {
        return wstring();
    }

    auto result = wstring(data.begin(), data.begin() + length);
    while (result[result.length() - 1] == 0) {
        result.resize(result.length() - 1);
    }

    return result;
}

std::vector<WCHAR> ToRaw(const wstring& str) {
    return std::vector<WCHAR>(str.begin(), str.end());
}

std::vector<BYTE> ToRaw(PCCOR_SIGNATURE signature, ULONG length) {
    return std::vector<BYTE>(&signature[0], &signature[length]);
}

WCHAR operator "" _W(const char c) { return WCHAR(c); }

wstring operator "" _W(const char* arr, size_t size) {
    const std::string str(arr, size);
    return ToWString(str);
}

std::string ToString(const wstring& wstr) {
    const std::u16string ustr(reinterpret_cast<const char16_t*>(wstr.c_str()));
    return miniutf::to_utf8(ustr);
}

wstring ToWString(const char* str) {
    return ToWString(std::string(str));
}

wstring ToWString(const std::string& str) {
    const auto ustr = miniutf::to_utf16(str);
    return wstring(reinterpret_cast<const WCHAR*>(ustr.c_str()));
}

wstring Trim(const wstring& str) {
    if (str.length() == 0) {
        return ""_W;
    }

    wstring trimmed = str;

    const auto trimSymbols = " \t\r\n\0"_W;

    const auto lpos = trimmed.find_first_not_of(trimSymbols);
    if (lpos != std::string::npos && lpos > 0) {
        trimmed = trimmed.substr(lpos);
    }

    const auto rpos = trimmed.find_last_not_of(trimSymbols);
    if (rpos != std::string::npos) {
        trimmed = trimmed.substr(0, rpos + 1);
    }

    return trimmed;
}

std::string Trim(const std::string& str) {
    if (str.length() == 0) {
        return "";
    }

    std::string trimmed = str;

    const auto trimSymbols = " \t\r\n\0";

    const auto lpos = trimmed.find_first_not_of(trimSymbols);
    if (lpos != std::string::npos && lpos > 0) {
        trimmed = trimmed.substr(lpos);
    }

    const auto rpos = trimmed.find_last_not_of(trimSymbols);
    if (rpos != std::string::npos) {
        trimmed = trimmed.substr(0, rpos + 1);
    }

    return trimmed;
}
