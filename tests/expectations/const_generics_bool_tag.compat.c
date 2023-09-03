#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef const char *Str;

struct HashTable_Str__c_char__false {
  uintptr_t num_buckets;
  uintptr_t capacity;
  uint8_t *occupied;
  Str *keys;
  char *vals;
};

typedef struct HashTable_Str__c_char__false MySet;

typedef void (*SetCallback)(Str key);

struct HashTable_Str__u64__true {
  uintptr_t num_buckets;
  uintptr_t capacity;
  uint8_t *occupied;
  Str *keys;
  uint64_t *vals;
};

typedef void (*MapCallback)(Str key, uint64_t val);

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

MySet *new_set(void);

void set_for_each(const MySet *set, SetCallback callback);

struct HashTable_Str__u64__true *new_map(void);

void map_for_each(const struct HashTable_Str__u64__true *map, MapCallback callback);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
