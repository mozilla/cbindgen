#ifdef MY_PLATFORM
#define API __declspec(dllimport)
#else
#define API
#endif