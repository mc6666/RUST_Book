#if defined(WIN32) || defined(_WIN32)
#define EXPORT __declspec(dllexport)
#else
#define EXPORT
#endif

extern "C" {
    EXPORT const char * introduce(const char * name, int age);

    EXPORT void deallocate_string(const char * s);
}
