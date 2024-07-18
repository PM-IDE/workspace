#ifndef PROCFILER_PERFORMANCE_COUNTER_H
#define PROCFILER_PERFORMANCE_COUNTER_H

#ifndef WIN32
// The number of nanoseconds in a second.
const int tccSecondsToNanoSeconds = 1000000000;

//copied and pasted from coreclr time.cpp
bool QueryPerformanceCounter2(OUT LARGE_INTEGER* lpPerformanceCount) {
    BOOL retval = TRUE;

#if HAVE_CLOCK_GETTIME_NSEC_NP
    lpPerformanceCount->QuadPart = (LONGLONG)clock_gettime_nsec_np(CLOCK_UPTIME_RAW);
#elif HAVE_CLOCK_MONOTONIC
    struct timespec ts;
    int result = clock_gettime(CLOCK_MONOTONIC, &ts);

    if (result != 0) {
        retval = FALSE;
    } else {
        lpPerformanceCount->QuadPart = ((LONGLONG) (ts.tv_sec) * (LONGLONG) (tccSecondsToNanoSeconds)) + (LONGLONG) (ts.tv_nsec);
    }
#else
#error "The PAL requires either mach_absolute_time() or clock_gettime(CLOCK_MONOTONIC) to be supported."
#endif

    return retval;
}

#else
bool QueryPerformanceCounter2(OUT LARGE_INTEGER* lpPerformanceCount) {
    return QueryPerformanceCounter(lpPerformanceCount);
}
#endif

#endif //PROCFILER_PERFORMANCE_COUNTER_H
