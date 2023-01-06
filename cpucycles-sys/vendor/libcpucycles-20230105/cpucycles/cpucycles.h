// version 20230105
// public domain
// djb

#ifndef cpucycles_h
#define cpucycles_h

#ifdef __cplusplus
extern "C" {
#endif

#define cpucycles_version "20230105"

extern long long (*cpucycles)(void) __attribute__((visibility("default")));;
extern const char *cpucycles_implementation(void) __attribute__((visibility("default")));;
extern long long cpucycles_persecond(void) __attribute__((visibility("default")));;
extern void cpucycles_tracesetup(void) __attribute__((visibility("default")));

#ifdef __cplusplus
}
#endif

#endif
