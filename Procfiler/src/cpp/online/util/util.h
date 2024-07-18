#pragma once

#include <corhlpr.h>
#include <string>
#include <vector>
#include <vector>
#include "../miniutf/miniutf.hpp"


typedef std::basic_string<WCHAR> wstring;
WCHAR operator"" _W(const char c);
wstring operator"" _W(const char* arr, size_t size);
wstring ToWString(const char* str);
wstring ToWString(const std::string& str);
std::string ToString(const wstring& wstr);

std::string GetEnvironmentValue(const std::string& name);

wstring Trim(const wstring& str);
std::string Trim(const std::string& str);

wstring ToString(const std::vector<WCHAR>& data, size_t length);
std::vector<WCHAR> ToRaw(const wstring& str);
std::vector<BYTE> ToRaw(PCCOR_SIGNATURE signature, ULONG length);