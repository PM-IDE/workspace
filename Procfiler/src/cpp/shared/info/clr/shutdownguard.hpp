#pragma once
#include <thread>
#include <atomic>

class ShutdownGuard
{
    static std::atomic<bool> s_preventHooks;
    static std::atomic<int> s_hooksInProgress;

public:
    ShutdownGuard()
    {
        ++s_hooksInProgress;
    }

    ~ShutdownGuard()
    {
        --s_hooksInProgress;
    }

    static void Initialize()
    {
        s_preventHooks = false;
        s_hooksInProgress = 0;
    }

    static bool HasShutdownStarted()
    {
        return s_preventHooks.load();
    }

    static void WaitForInProgressHooks()
    {
        s_preventHooks = true;

        while (s_hooksInProgress.load() > 0)
        {
            std::this_thread::sleep_for(std::chrono::milliseconds(10));
        }
    }
};

// Managed code can keep running after Shutdown() is called, and things like
// ELT hooks will continue to be called. We would AV if we tried to call
// in to freed resources.
#define SHUTDOWNGUARD()                         \
    ShutdownGuard shutdownGuard;                \
    if (ShutdownGuard::HasShutdownStarted())    \
    {                                           \
        return S_OK;                            \
    }

#define SHUTDOWNGUARD_RETVOID()                 \
    ShutdownGuard shutdownGuard;                \
    if (ShutdownGuard::HasShutdownStarted())    \
    {                                           \
        return;                                 \
    }